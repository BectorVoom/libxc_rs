#!/usr/bin/env python3
"""Merge a split output's part functions into one loop, deduplicating by value.

The maple2c pipeline split large outputs into parts and re-derived every shared
intermediate in each part from scratch: measured across the tree, the same
value is computed 3-16x over (dup factor `total defs / distinct values`).
This tool merges the parts of one output directory into a single function and
computes every distinct value exactly once.

Names cannot be trusted across parts (maple2c restarts its CSE numbering, so
`t10` in part1 and `t10` in part2 may be different expressions), so the merge
is by *value numbering*: each definition's RHS is canonicalised by replacing
every operand identifier with the value id it is currently bound to; an RHS
seen before reuses the existing value, a new one is emitted as `let v<id> =`.

Bit-exactness argument: every emitted expression is textually identical to the
original except for operand renaming; a reused value was produced by an
identical expression over identical values, and the kernels are pure
straight-line FP (no reads of output buffers, `+=` stores only), so dropping a
duplicate computation cannot change any result bit. Store order per output
element follows part order within the merged loop, which matches the
sequential part calls of the old wrapper.

Usage:
    vnmerge.py <crate>/src            merge every split-output dir in place
    vnmerge.py --dry-run <crate>/src  report def counts without writing
"""
from __future__ import annotations

import collections
import heapq
import re
import sys
from pathlib import Path

IDENT = re.compile(r"(?<![\w.])([A-Za-z_]\w*)")
LET = re.compile(r"^\s*let (\([^)]*\)|\w+) = (.*);\s*$", re.S)
STORE = re.compile(r"^\s*(\w+)\[([^\]]+)\] \+= (.*);\s*$", re.S)
FOR = re.compile(r"^\s*for ip in 0\.\.(.+?) \{\s*$")
SIG = re.compile(r"(pub fn \w+\(\n.*?\n\) \{)", re.S)
# Operand-shaped names that must already be bound when referenced.
MUST_BIND = re.compile(r"^(t\d+|rho\d+|sigma\d+|lapl\d+|tau\d+)$")

# --- segmentation ---------------------------------------------------------
# A merged output is one function, and a function cannot be split across
# codegen units, so the biggest outputs serialise their crate's build. These
# split the *emitted* function into contiguous segments that pass live values
# through a scratch array, leaving the value numbering (and therefore the
# operation count) exactly as the uncapped merge computed it.
V_TOKEN = re.compile(r"\bv(\d+)\b")
LET_V = re.compile(r"^\s*let v(\d+) = (.*);\s*$", re.S)
# rustc places every mono item in its defining module's codegen unit and only
# ever *merges* CGUs down to the target count -- it never splits one. Sibling
# free functions in one module therefore share a CGU and buy no parallelism,
# so each segment gets its own `mod`.
MAX_SEGMENTS = 16
# Below this multiple of the target there is nothing to gain from cutting.
SEG_MIN_FACTOR = 1.5
# Refuse a cut with more live values than this. Each one costs a store and a
# reload per grid point, so a wide cut buys build parallelism with runtime.
MAX_CUT_WIDTH = 1200


class MergeError(RuntimeError):
    pass


def split_statements(body: str) -> list[str]:
    """Split loop-body text into balanced `...;` statements."""
    stmts, cur, depth = [], [], 0
    for line in body.split("\n"):
        if not line.strip() and not cur:
            continue
        cur.append(line)
        depth += (line.count("{") - line.count("}")
                  + line.count("(") - line.count(")"))
        if depth == 0 and line.rstrip().endswith(";"):
            stmts.append("\n".join(cur))
            cur = []
    if cur and "".join(cur).strip():
        raise MergeError(f"trailing unbalanced text: {cur[0][:80]!r}")
    return stmts


class Merger:
    def __init__(self) -> None:
        self.table: dict[str, int] = {}   # canonical RHS -> value id
        self.out: list[str] = []          # emitted statements
        self.next_id = 0

    def _canon_and_render(self, expr: str, env: dict[str, int]) -> tuple[str, str]:
        expr = re.sub(r"\s+", " ", expr.strip())

        def keyf(m: re.Match) -> str:
            name = m.group(1)
            if name in env:
                return f"#{env[name]}"
            if MUST_BIND.match(name):
                raise MergeError(f"unbound operand `{name}` in {expr[:80]!r}")
            return name

        def renderf(m: re.Match) -> str:
            name = m.group(1)
            return f"v{env[name]}" if name in env else name

        return IDENT.sub(keyf, expr), IDENT.sub(renderf, expr)

    def value_of(self, expr: str, env: dict[str, int]) -> int:
        """Value id of `expr`, emitting a definition if it is new."""
        if expr.strip().startswith("{"):
            return self._value_of_block(expr, env)
        key, rendered = self._canon_and_render(expr, env)
        alias = re.fullmatch(r"#(\d+)", key)
        if alias:
            return int(alias.group(1))
        vid = self.table.get(key)
        if vid is None:
            vid = self.next_id
            self.next_id += 1
            self.table[key] = vid
            self.out.append(f"        let v{vid} = {rendered};")
        return vid

    def _value_of_block(self, expr: str, env: dict[str, int]) -> int:
        raise MergeError("single-bind block reached value_of; use eval_block")

    def eval_block(self, block: str, env: dict[str, int]) -> list[int]:
        """Evaluate `{ stmts...; tail }`, returning tail value ids."""
        inner = block.strip()
        if not (inner.startswith("{") and inner.endswith("}")):
            raise MergeError(f"not a block: {inner[:60]!r}")
        inner = inner[1:-1]
        cut = inner.rfind(";")
        stmt_text, tail = inner[: cut + 1], inner[cut + 1:].strip()
        benv = dict(env)
        self.eval_stmts(split_statements(stmt_text), benv)
        if tail.startswith("(") and tail.endswith(")"):
            elems = [e.strip() for e in tail[1:-1].split(",") if e.strip()]
        else:
            elems = [tail]
        ids = []
        for e in elems:
            if e in benv:
                ids.append(benv[e])
            else:
                ids.append(self.value_of(e, benv))
        return ids

    def eval_stmts(self, stmts: list[str], env: dict[str, int]) -> None:
        for s in stmts:
            m = LET.match(s)
            if m:
                bind, rhs = m.groups()
                if rhs.strip().startswith("{"):
                    ids = self.eval_block(rhs, env)
                    names = ([bind] if not bind.startswith("(") else
                             [n.strip() for n in bind[1:-1].split(",") if n.strip()])
                    if len(names) != len(ids):
                        raise MergeError(
                            f"bind arity {len(names)} != tail arity {len(ids)}"
                        )
                    for n, i in zip(names, ids):
                        env[n] = i
                else:
                    if bind.startswith("("):
                        raise MergeError(f"tuple bind on non-block: {s[:80]!r}")
                    env[bind] = self.value_of(rhs, env)
                continue
            m = STORE.match(s)
            if m:
                buf, idx, rhs = m.groups()
                vid = self.value_of(rhs, env)
                self.out.append(f"        {buf}[{idx}] += v{vid};")
                continue
            raise MergeError(f"unrecognised statement: {s.strip()[:100]!r}")


def parse_params(sig: str) -> list[tuple[str, str]]:
    """[(name, type)] from a `pub fn f(\\n  a: T,\\n) {` signature."""
    inner = sig[sig.index("(") + 1: sig.rindex(")")]
    out = []
    for line in inner.split("\n"):
        line = line.strip().rstrip(",")
        if not line:
            continue
        name, _, ty = line.partition(":")
        out.append((name.strip(), ty.strip()))
    return out


def ordered_stores(stmts: list[str]) -> list[str]:
    """Output buffers written by `stmts`, in first-write order."""
    seen: list[str] = []
    for s in stmts:
        m = STORE.match(s)
        if m and m.group(1) not in seen:
            seen.append(m.group(1))
    return seen


def stride_bound(stmts: list[str]) -> str:
    """Grid-point loop bound, read off the first written buffer's indexing.

    Same rule as the guard rewrite in xform.py: a buffer with D elements per
    grid point is indexed `buf[ip * D + k]`, so the loop must run
    `buf.len() / D` times, not `buf.len()`.
    """
    bufs = ordered_stores(stmts)
    if not bufs:
        raise MergeError("group writes no output; cannot derive a loop bound")
    buf = bufs[0]
    mults, plain = set(), False
    for s in stmts:
        m = STORE.match(s)
        if not m or m.group(1) != buf:
            continue
        idx = m.group(2).strip()
        mm = re.fullmatch(r"ip \* (\d+)(?: \+ \d+)?", idx)
        if mm:
            mults.add(int(mm.group(1)))
        elif idx == "ip":
            plain = True
        else:
            raise MergeError(f"unrecognised store index `{buf}[{idx}]`")
    if mults and plain:
        raise MergeError(f"buffer `{buf}` mixes strided and plain indexing")
    if len(mults) > 1:
        raise MergeError(f"buffer `{buf}` has inconsistent strides {sorted(mults)}")
    d = mults.pop() if mults else 1
    return f"{buf}.len()" if d == 1 else f"{buf}.len() / {d}"


def _refs(text: str, name: str) -> bool:
    """True if `name` appears in `text` as a whole identifier."""
    return re.search(rf"(?<![\w.]){re.escape(name)}(?![\w])", text) is not None


def _analyze(out: list[str]) -> tuple[dict[int, int], dict[int, int], list[set[int]]]:
    """(def position, last-use position, per-statement use set) over `out`.

    Every operand in a merged statement is either a `v<id>` produced earlier in
    the same loop body or something ambient (an input load, `ip`, a scalar
    parameter, a constant, a math fn). Only the former can cross a cut, and
    `\\bv(\\d+)\\b` picks exactly those: an output buffer like `v2rho2` fails the
    trailing word boundary, so stores contribute only their stored value.
    """
    def_pos: dict[int, int] = {}
    last_use: dict[int, int] = {}
    uses: list[set[int]] = []
    for i, s in enumerate(out):
        m = LET_V.match(s)
        u = {int(x) for x in V_TOKEN.findall(m.group(2) if m else s)}
        uses.append(u)
        for vid in u:
            last_use[vid] = i
        if m:
            def_pos[int(m.group(1))] = i
    return def_pos, last_use, uses


def _is_bool_rhs(rhs: str) -> bool:
    """True if `rhs` evaluates to a bool rather than an f64.

    Only operators at parenthesis depth 0 count: `piecewise3` takes a bool and
    returns an f64, so `piecewise3(t <= zeta_threshold, a, b)` is an f64
    definition even though a comparison appears inside it.
    """
    depth = 0
    i = 0
    while i < len(rhs):
        c = rhs[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif depth == 0:
            if rhs[i:i + 2] in ("<=", ">=", "==", "!=", "&&", "||"):
                return True
            if c in "<>":
                return True
            if c == "!" and rhs[i:i + 2] != "!=":
                return True
        i += 1
    return False


def _value_types(out: list[str]) -> dict[int, str]:
    """Rust type of every merged value: `f64` except for comparison results."""
    types: dict[int, str] = {}
    for s in out:
        m = LET_V.match(s)
        if m:
            types[int(m.group(1))] = "bool" if _is_bool_rhs(m.group(2)) else "f64"
    return types


def _live_after(out: list[str], def_pos: dict[int, int],
                last_use: dict[int, int]) -> list[int]:
    """Number of values live across the boundary after each statement."""
    ending: dict[int, list[int]] = collections.defaultdict(list)
    for vid, lu in last_use.items():
        ending[lu].append(vid)
    width: list[int] = []
    live = 0
    for i, s in enumerate(out):
        m = LET_V.match(s)
        if m:
            vid = int(m.group(1))
            if last_use.get(vid, -1) > i:
                live += 1
        for vid in ending.get(i, ()):
            if def_pos.get(vid, -1) < i:
                live -= 1
        width.append(live)
    return width


def _alloc_slots(def_pos: dict[int, int], last_use: dict[int, int],
                 seg_of: list[int], only: set[int] | None = None
                 ) -> tuple[dict[int, int], int]:
    """Assign scratch slots to cut-crossing values by linear scan.

    A value occupies its slot from the segment that defines it through the
    segment that last uses it, so slots are reused down the function and the
    array is sized by the widest cut rather than by the total crossing count.

    A slot frees at the *end* of its last-needed segment rather than after it:
    every segment reads all of its incoming values into locals in a prologue
    before executing any statement, so a value defined in segment t may reuse
    the slot of a value last read in segment t's prologue.
    """
    crossing: dict[int, tuple[int, int]] = {}
    for vid, dp in def_pos.items():
        if only is not None and vid not in only:
            continue
        lu = last_use.get(vid)
        if lu is None:
            continue
        ds, ls = seg_of[dp], seg_of[lu]
        if ds < ls:
            crossing[vid] = (ds, ls)
    free: list[int] = []
    busy: list[tuple[int, int]] = []  # (last segment needed, slot)
    slot_of: dict[int, int] = {}
    nslots = 0
    for vid in sorted(crossing, key=lambda v: (crossing[v][0], v)):
        ds, ls = crossing[vid]
        while busy and busy[0][0] <= ds:
            free.append(heapq.heappop(busy)[1])
        if free:
            slot = free.pop()
        else:
            slot, nslots = nslots, nslots + 1
        slot_of[vid] = slot
        heapq.heappush(busy, (ls, slot))
    return slot_of, nslots


def plan_segments(out: list[str], seg_target: int,
                  max_width: int = MAX_CUT_WIDTH) -> dict | None:
    """Partition merged statements into segments, or None to leave them whole.

    Cut placement is what decides whether segmentation is affordable. Every
    value live across a cut is stored and reloaded *per grid point*, so a cut
    through a wide part of the dataflow costs thousands of memory round-trips
    per point -- measured at roughly 3 cycles each against roughly 1.4 for the
    arithmetic it is meant to relieve. These kernels are wide nearly
    everywhere (median live width on the big outputs runs 2.6k-8k values), so
    cuts are chosen by searching the whole live-value curve for its narrowest
    points, and any cut wider than `max_width` is refused outright. An output
    with no cheap cut stays whole: build parallelism is not worth buying at
    the cost of the numbers coming out slower.
    """
    if seg_target <= 0:
        return None
    n = len(out)
    n_defs = sum(1 for s in out if LET_V.match(s))
    if n_defs < seg_target * SEG_MIN_FACTOR:
        return None
    k = min(MAX_SEGMENTS, max(2, -(-n_defs // seg_target)))
    def_pos, last_use, uses = _analyze(out)
    width = _live_after(out, def_pos, last_use)
    # Cheapest cuts first, keeping segments from degenerating into slivers.
    min_gap = max(1, n // (2 * k))
    cuts: list[int] = []
    for i in sorted(range(1, n), key=lambda i: (width[i - 1], i)):
        if len(cuts) >= k - 1:
            break
        if max_width and width[i - 1] > max_width:
            break
        if i < min_gap or i > n - min_gap:
            continue
        if any(abs(i - c) < min_gap for c in cuts):
            continue
        cuts.append(i)
    cuts.sort()
    if not cuts:
        return None
    bounds = [0] + cuts + [n]
    seg_of = [0] * n
    for t in range(len(bounds) - 1):
        for i in range(bounds[t], bounds[t + 1]):
            seg_of[i] = t
    # Comparison results are `bool`, not `f64`, and a handful of them cross
    # cuts on their way to `piecewise3`. They get their own scratch array
    # rather than an encoding, so no value changes representation.
    types = _value_types(out)
    bools = {v for v, t in types.items() if t == "bool"}
    floats = set(def_pos) - bools
    slot_f, n_f = _alloc_slots(def_pos, last_use, seg_of, floats)
    slot_b, n_b = _alloc_slots(def_pos, last_use, seg_of, bools)
    if n_f + n_b == 0:
        return None
    slot_of: dict[int, tuple[str, int]] = {v: ("w", s) for v, s in slot_f.items()}
    slot_of.update({v: ("wb", s) for v, s in slot_b.items()})
    return {
        "bounds": bounds, "seg_of": seg_of, "uses": uses, "def_pos": def_pos,
        "last_use": last_use, "slot_of": slot_of, "nslots": n_f, "nbools": n_b,
        "defs": n_defs, "max_cut_width": max(width[c - 1] for c in cuts),
    }


def emit_segmented(fn_name: str, sig: str, params: list[tuple[str, str]],
                   out: list[str], plan: dict, bound: str) -> str:
    """Emit a merged output as `mod segN` functions threaded through scratch.

    Each segment holds a contiguous run of the merged statements verbatim; a
    value that outlives its segment is copied to a scratch slot at its
    definition and read back as a plain `let` at the top of every later segment
    that needs it. Every arithmetic expression is therefore byte-identical to
    the unsegmented merge, and an f64 round-trip through memory preserves the
    bit pattern, so results (NaN payloads included) cannot change.
    """
    bounds, seg_of, uses = plan["bounds"], plan["seg_of"], plan["uses"]
    def_pos, last_use = plan["def_pos"], plan["last_use"]
    slot_of, w_len, wb_len = plan["slot_of"], plan["nslots"], plan["nbools"]
    inputs = [n for n, t in params if t == "&[f64]"]
    scalars = [n for n, t in params if t == "f64"]
    outs = [n for n, t in params if t == "&mut [f64]"]
    if any(re.fullmatch(r"v\d+", n) for n, _ in params):
        raise MergeError(f"{fn_name}: a parameter is named like a value id")
    # Scratch arrays, in the order they are threaded through every segment.
    arrays = ([("w", f"&mut [f64; {w_len}]", f"vec![0.0f64; {w_len}]")]
              if w_len else [])
    arrays += ([("wb", f"&mut [bool; {wb_len}]", f"vec![false; {wb_len}]")]
               if wb_len else [])
    aty = {name: ty for name, ty, _init in arrays}

    # Simulated scratch contents, advanced in emission order. Every prologue
    # read is checked against it, so a slot that was reused too early -- the
    # one way this transform could go quietly wrong -- fails the emit instead
    # of producing a kernel that silently reads a stale value.
    holder: dict[tuple[str, int], int] = {}

    mods, calls = [], []
    for t in range(len(bounds) - 1):
        a, b = bounds[t], bounds[t + 1]
        live_in = sorted({vid for i in range(a, b) for vid in uses[i]
                          if seg_of[def_pos[vid]] < t})
        body = []
        for vid in live_in:
            if vid not in slot_of:
                raise MergeError(
                    f"{fn_name}: v{vid} crosses into segment {t} without a slot"
                )
            arr, slot = slot_of[vid]
            if holder.get((arr, slot)) != vid:
                raise MergeError(
                    f"{fn_name}: segment {t} reads v{vid} from {arr}[{slot}], "
                    f"which holds v{holder.get((arr, slot))}"
                )
            body.append(f"        let v{vid} = {arr}[{slot}];")
        for i in range(a, b):
            body.append(out[i])
            m = LET_V.match(out[i])
            if m:
                vid = int(m.group(1))
                if vid in slot_of and seg_of[last_use[vid]] > t:
                    arr, slot = slot_of[vid]
                    holder[(arr, slot)] = vid
                    body.append(f"        {arr}[{slot}] = v{vid};")
        text = "\n".join(body)
        seg_in = [n for n in inputs if _refs(text, n)]
        seg_out = [n for n in outs if _refs(text, n)]
        seg_sc = [n for n in scalars if _refs(text, n)]
        plist = ",\n".join(
            [f"        {n}: &[f64]" for n in seg_in]
            + [f"        {n}: &mut [f64]" for n in seg_out]
            + [f"        {n}: f64" for n in seg_sc]
            + ["        ip: usize"]
            + [f"        {n}: {aty[n]}" for n, _ty, _i in arrays]
        )
        # `pub(super)`, not `pub`: the segments are an emission detail, and the
        # verification harnesses discover entry points by scanning for `pub fn`.
        mods.append(
            f"mod seg{t} {{\n    use super::*;\n    #[inline(never)]\n"
            f"    pub(super) fn run(\n{plist},\n    ) {{\n{text}\n    }}\n}}\n"
        )
        args = seg_in + seg_out + seg_sc + ["ip"] + [n for n, _t, _i in arrays]
        calls.append(f"        seg{t}::run({', '.join(args)});")

    # Scratch lives on the heap and outside the grid loop: one allocation per
    # kernel call (i.e. per rayon chunk), and no stack-depth exposure on a
    # worker thread whatever the width turns out to be. A fixed-size array
    # reference keeps the constant-index accesses free of bounds checks.
    alloc = "".join(
        f"    let mut {n}_vec = {init};\n"
        f"    let {n}: {ty} = (&mut {n}_vec[..]).try_into().unwrap();\n"
        for n, ty, init in arrays
    )
    wrapper = (
        f"{sig}\n{alloc}"
        f"    for ip in 0..{bound} {{\n" + "\n".join(calls) + "\n    }\n}\n"
    )
    return "\n".join(mods) + "\n" + wrapper


def parse_part(text: str, origin: str) -> tuple[str, str, list[str]]:
    """Return (fn_name, guard_expr, body statements) of a flat part file."""
    m = re.search(r"pub fn (\w+)\(", text)
    if not m:
        raise MergeError(f"{origin}: no fn")
    name = m.group(1)
    lines = text.split("\n")
    start = next(i for i, l in enumerate(lines) if FOR.match(l))
    guard = FOR.match(lines[start]).group(1)
    # loop body runs to the matching close; the file ends `    }\n}` so take
    # everything between and strip the two trailing braces.
    depth = 0
    for j in range(start, len(lines)):
        depth += lines[j].count("{") - lines[j].count("}")
        if depth == 0:
            break
    body = "\n".join(lines[start + 1: j])
    return name, guard, split_statements(body)


def merge_texts(outname: str, files: dict[str, str], *, cap: int = 0,
                seg_target: int = 0, max_width: int = MAX_CUT_WIDTH
                ) -> tuple[str, dict]:
    """Merge one output directory given as `{filename: source text}`.

    `files` must hold the wrapper as `mod.rs` plus its `partN.rs` bodies.
    Returns (merged source text, stats). Raises MergeError for any shape the
    merge does not cover, so the caller can fall back to the split form.
    """
    if "mod.rs" not in files:
        raise MergeError(f"{outname}: no mod.rs")
    wrapper = files["mod.rs"]
    sig_m = SIG.search(wrapper)
    if not sig_m:
        raise MergeError(f"{outname}: wrapper signature not found")
    sig = sig_m.group(1)
    if any("pub struct" in t for t in files.values()):
        raise MergeError(f"{outname}: struct-interface output; not merged")

    # Part call order from the wrapper body, not directory order.
    called = re.findall(r"^\s*(\w+)\(", wrapper[sig_m.end():], re.M)
    by_name = {}
    for fname, text in sorted(files.items()):
        if not re.fullmatch(r"part\d+\.rs", fname):
            continue
        name, guard, stmts = parse_part(text, f"{outname}/{fname}")
        by_name[name] = (guard, stmts, fname)
    if set(called) != set(by_name):
        raise MergeError(
            f"{outname}: wrapper calls {len(called)} parts, "
            f"found {len(by_name)} part files"
        )

    params = parse_params(sig)
    inputs = [n for n, t in params if t == "&[f64]"]
    scalars = [n for n, t in params if t == "f64"]
    out_bufs = {n for n, t in params if t == "&mut [f64]"}

    # Dropping a duplicate computation is only sound if nothing reads an
    # output buffer: with `+=` stores the buffers are write-only accumulators,
    # so a value computed once is the same value every part would have
    # recomputed. A read would make the result order-dependent, and the merge
    # reorders nothing but must not silently permit the shape.
    for name in called:
        _g, stmts, fname = by_name[name]
        for s in stmts:
            m = STORE.match(s)
            rhs = m.group(3) if m else (LET.match(s).group(2) if LET.match(s) else s)
            for b in out_bufs:
                if re.search(rf"(?<![\w.]){re.escape(b)}\s*\[", rhs):
                    raise MergeError(
                        f"{outname}/{fname}: output `{b}` is read in an "
                        "expression; merge would be unsound"
                    )

    # Group parts into buckets whose merged def-count stays under `cap`.
    # A single merged function is ideal for total CPU and peak RSS, but it
    # cannot be split across codegen units, so one huge output (mgga_c_kcis
    # lxc_pol: 44k defs) serialises the whole crate's build. Bucketing trades
    # a little cross-bucket dedup for CGU parallelism. cap=0 means unlimited.
    buckets: list[tuple[Merger, list[str]]] = []
    merger, members = Merger(), []
    n_stores_in = 0
    for name in called:
        _guard, stmts, _fname = by_name[name]
        n_stores_in += sum(1 for s in stmts if STORE.match(s))
        merger.eval_stmts(stmts, {})
        members.append(name)
        if cap and merger.next_id >= cap:
            buckets.append((merger, members))
            merger, members = Merger(), []
    if members:
        buckets.append((merger, members))

    n_stores_out = sum(
        1 for m, _ in buckets for s in m.out if not s.lstrip().startswith("let ")
    )
    if n_stores_out != n_stores_in:
        raise MergeError(
            f"{outname}: store count changed {n_stores_in} -> {n_stores_out}"
        )

    uses = sorted({
        l for t in files.values() for l in t.split("\n")
        if l.startswith("use libxc_rkernel_math::") or l.startswith("use libm")
    })
    total_defs = sum(
        1 for _, (g, stmts, _f) in by_name.items() for s in stmts if LET.match(s)
    )
    fn_name = re.search(r"pub fn (\w+)\(", sig).group(1)
    defs_out = sum(m.next_id for m, _ in buckets)

    # Segmentation applies to the whole-output merge only: it is the answer to
    # the same codegen-unit problem `cap` attacks, but without giving up any
    # deduplication, so the two are alternatives rather than a combination.
    plan = (plan_segments(buckets[0][0].out, seg_target, max_width)
            if len(buckets) == 1 else None)

    if plan is not None:
        shape = (f"{len(plan['bounds']) - 1} segment(s), "
                 f"{plan['nslots']} scratch slot(s)")
    else:
        shape = f"{len(buckets)} group(s)"
    head = (
        f"//! Merged {outname} kernel — value-numbered across "
        f"{len(called)} parts by tools/translate_rayon/vnmerge.py.\n"
        f"//! {defs_out} distinct values from {total_defs} original "
        f"definitions, in {shape}; bit-identical to the\n"
        "//! sequential part calls.\n"
        "#![allow(unused_imports, unused_variables, non_snake_case, "
        "clippy::excessive_precision, clippy::too_many_arguments, "
        "clippy::needless_return)]\n\n"
        + "\n".join(uses) + "\n\n"
    )

    if plan is not None:
        body = emit_segmented(fn_name, sig, params, buckets[0][0].out, plan,
                              stride_bound(buckets[0][0].out))
    elif len(buckets) == 1:
        body = (sig + "\n"
                + f"    for ip in 0..{stride_bound(buckets[0][0].out)} {{\n"
                + "\n".join(buckets[0][0].out) + "\n    }\n}\n")
    else:
        chunks, calls = [], []
        for gi, (m, _members) in enumerate(buckets):
            written = [b for b in ordered_stores(m.out)]
            args = inputs + written + scalars
            plist = ",\n".join(
                [f"    {n}: &[f64]" for n in inputs]
                + [f"    {n}: &mut [f64]" for n in written]
                + [f"    {n}: f64" for n in scalars]
            )
            chunks.append(
                f"#[allow(clippy::too_many_arguments)]\n"
                f"fn {fn_name}_g{gi}(\n{plist},\n) {{\n"
                f"    for ip in 0..{stride_bound(m.out)} {{\n"
                + "\n".join(m.out) + "\n    }\n}\n"
            )
            calls.append(f"    {fn_name}_g{gi}({', '.join(args)});")
        body = "\n".join(chunks) + "\n" + sig + "\n" + "\n".join(calls) + "\n}\n"

    text = head + body

    stats = {
        "output": outname, "parts": len(called), "groups": len(buckets),
        "defs_in": total_defs, "defs_out": defs_out,
        "stores": n_stores_out, "fn": fn_name,
        "segments": len(plan["bounds"]) - 1 if plan else 1,
        "slots": plan["nslots"] if plan else 0,
        "max_cut_width": plan["max_cut_width"] if plan else 0,
    }
    return text, stats


def merge_output_dir(outdir: Path, *, dry_run: bool = False, cap: int = 0,
                     seg_target: int = 0,
                     max_width: int = MAX_CUT_WIDTH) -> dict:
    """Disk-facing wrapper: merge `outdir/` into a sibling `<outdir>.rs`."""
    files = {p.name: p.read_text() for p in outdir.iterdir() if p.is_file()}
    if any(p.is_dir() for p in outdir.iterdir()):
        raise MergeError(f"{outdir.name}: nested directories; flatten first")
    text, stats = merge_texts(outdir.name, files, cap=cap,
                              seg_target=seg_target, max_width=max_width)
    if not dry_run:
        target = outdir.parent / f"{outdir.name}.rs"
        # Preserve mtime on an unchanged re-merge so cargo does not rebuild
        # kernel crates a regen did not actually alter.
        try:
            unchanged = target.read_text() == text
        except FileNotFoundError:
            unchanged = False
        if not unchanged:
            target.write_text(text)
        for p in outdir.iterdir():
            p.unlink()
        outdir.rmdir()
    return stats


def main() -> int:
    argv = sys.argv[1:]
    dry = "--dry-run" in argv
    cap = 0
    seg_target = 0
    max_width = MAX_CUT_WIDTH
    for a in argv:
        if a.startswith("--cap="):
            cap = int(a.split("=", 1)[1])
        elif a.startswith("--seg-target="):
            seg_target = int(a.split("=", 1)[1])
        elif a.startswith("--seg-max-width="):
            max_width = int(a.split("=", 1)[1])
    args = [a for a in argv if not a.startswith("--")]
    src = Path(args[0])
    for d in sorted(p for p in src.iterdir() if p.is_dir()):
        try:
            st = merge_output_dir(d, dry_run=dry, cap=cap,
                                  seg_target=seg_target,
                                  max_width=max_width)
        except MergeError as exc:
            print(f"skip {d.name}: {exc}")
            continue
        shape = (f"{st['groups']} group(s)" if st["segments"] == 1 else
                 f"{st['segments']} segment(s), {st['slots']} slots "
                 f"(widest cut {st['max_cut_width']})")
        print(f"{st['output']}: {st['parts']} parts -> {shape}, "
              f"{st['defs_in']} -> {st['defs_out']} defs "
              f"({st['defs_in'] / max(st['defs_out'], 1):.2f}x), "
              f"{st['stores']} stores")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
