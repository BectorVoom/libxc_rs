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


def merge_texts(outname: str, files: dict[str, str], *, cap: int = 0
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

    head = (
        f"//! Merged {outname} kernel — value-numbered across "
        f"{len(called)} parts by tools/translate_rayon/vnmerge.py.\n"
        f"//! {defs_out} distinct values from {total_defs} original "
        f"definitions, in {len(buckets)} group(s); bit-identical to the\n"
        "//! sequential part calls.\n"
        "#![allow(unused_imports, unused_variables, non_snake_case, "
        "clippy::excessive_precision, clippy::too_many_arguments, "
        "clippy::needless_return)]\n\n"
        + "\n".join(uses) + "\n\n"
    )

    if len(buckets) == 1:
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
    }
    return text, stats


def merge_output_dir(outdir: Path, *, dry_run: bool = False,
                     cap: int = 0) -> dict:
    """Disk-facing wrapper: merge `outdir/` into a sibling `<outdir>.rs`."""
    files = {p.name: p.read_text() for p in outdir.iterdir() if p.is_file()}
    if any(p.is_dir() for p in outdir.iterdir()):
        raise MergeError(f"{outdir.name}: nested directories; flatten first")
    text, stats = merge_texts(outdir.name, files, cap=cap)
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
    for a in argv:
        if a.startswith("--cap="):
            cap = int(a.split("=", 1)[1])
    args = [a for a in argv if not a.startswith("--")]
    src = Path(args[0])
    for d in sorted(p for p in src.iterdir() if p.is_dir()):
        try:
            st = merge_output_dir(d, dry_run=dry, cap=cap)
        except MergeError as exc:
            print(f"skip {d.name}: {exc}")
            continue
        print(f"{st['output']}: {st['parts']} parts -> {st['groups']} group(s), "
              f"{st['defs_in']} -> {st['defs_out']} defs "
              f"({st['defs_in'] / max(st['defs_out'], 1):.2f}x), "
              f"{st['stores']} stores")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
