#!/usr/bin/env python3
"""Reorder a merged kernel's statements to cut register pressure.

Why this pass exists
--------------------

`vnmerge.py` emits each output's definitions in value-numbering order, which
puts the shared prefix first and every output's own arithmetic after it. That is
the worst possible order for register pressure: a value computed in the prefix
and used by the last output stays live across the entire body. Measured on the
built tree (`objdump`, release, `target-cpu=native`), the slowest kernels spend
about a quarter of their instructions moving values to and from the stack:

    kernel              insns  spill reload  stack%
    mgga_c_r2scan vxc    3050    417    297     23%
    mgga_x_scan   vxc    1771    188    141     19%
    gga_c_lyp     vxc     853     53     33     10%
    gga_x_rge2    vxc     703     10      9      3%

`gga_x_rge2` shows what a low-pressure body looks like, so the traffic in the
others is not inherent to the arithmetic.

This pass keeps the *expressions* untouched -- every value is still computed by
exactly the same operation on exactly the same operands, so the floating-point
result and its operation order are unchanged. Only the order of independent
`let` bindings moves, which is not an FP-order change. That was checked rather
than argued: every output fingerprint from `bench-vs-libxc` is unchanged after
rescheduling.

The schedule
------------

Greedy list scheduling over the dependency DAG. At each step, among the
statements whose operands are all available, take the one that changes the live
count least:

    delta = (1 if this value is used later else 0) - (operands dying here)

A statement that consumes the last use of two values and produces one that is
itself soon dead has delta -1 and is taken immediately; a statement that opens a
new long-lived value has delta +1 and waits. Ties go to the earliest statement in
the original order, which keeps the output stable and close to the input.

That is a heuristic -- minimising register pressure over a DAG is NP-hard -- but
it is the standard one, and it is exactly the choice `vnmerge` is already making
implicitly by emitting in value-numbering order.

Measured outcome: NOT worth wiring in (2026-08-18)
--------------------------------------------------

This pass is **not** called from `translate.py`. It was measured and it does not
pay. Applied to the vxc kernels of `mgga_c_r2scan`, `mgga_x_scan`, `gga_c_lyp`,
`gga_x_b88` and `lda_c_vwn`, rebuilt, and timed with `bench-vs-libxc`:

    kernel            peak live      single-thread ns/pt
    mgga_c_r2scan    131 -> 126        78.50 -> 81.17   (0.97x)
    gga_x_b88         27 ->  25        12.92 -> 12.99   (0.99x)
    mgga_x_scan       55 ->  54        27.47 -> 27.27   (1.01x)

Output fingerprints were unchanged, so the reordering is value-preserving as
intended -- it simply buys nothing. Two reasons, both visible in the numbers:

* The peak-live reduction is 2-7%, and the level that matters is the *absolute*
  one. `mgga_c_r2scan` holds 131 values live at its peak; a vectorised loop puts
  each in a `zmm`, of which there are 32. Going from 131 to 126 does not change
  the fact that most of them spill.
* LLVM re-schedules anyway. Source order biases the initial IR, nothing more.

Kept as a **diagnostic**: run it without `--write` to get the peak-live figure
per kernel, which is the number to watch if `vnmerge` ever changes how much it
shares. Do not wire it into the pipeline without re-measuring runtime -- peak
live is a proxy, and this is the measurement showing the proxy is not enough.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

LET = re.compile(r"^(\s*)let (\w+) = (.*);$")
WRITE = re.compile(r"^(\s*)(\w+)\[([^\]]*)\] \+= (\w+);$")
LOOP = re.compile(r"^\s*for ip in 0\.\.")
# Identifiers that are not local values: parameters, constants, helpers.
IDENT = re.compile(r"\b([A-Za-z_]\w*)\b")


class Stmt:
    __slots__ = ("idx", "indent", "text", "defs", "uses", "is_write", "array")

    def __init__(self, idx, indent, text, defs, uses, is_write, array=None):
        self.idx = idx
        self.indent = indent
        self.text = text
        self.defs = defs        # name defined, or None for a write
        self.uses = uses        # set of local names read
        self.is_write = is_write
        self.array = array


def parse_body(lines: list[str], start: int, end: int) -> list[Stmt] | None:
    """Parse loop-body lines into statements. Returns None if anything is not
    one of the two shapes this pass understands, so an unexpected kernel is
    passed through untouched rather than mangled."""
    stmts: list[Stmt] = []
    defined: set[str] = set()
    for i in range(start, end):
        raw = lines[i]
        s = raw.rstrip("\n")
        if not s.strip():
            continue
        m = LET.match(s)
        if m:
            indent, name, expr = m.group(1), m.group(2), m.group(3)
            uses = {t for t in IDENT.findall(expr) if t in defined}
            stmts.append(Stmt(len(stmts), indent, s, name, uses, False))
            defined.add(name)
            continue
        m = WRITE.match(s)
        if m:
            indent, arr, idx, val = m.groups()
            uses = {t for t in IDENT.findall(idx) if t in defined}
            if val in defined:
                uses.add(val)
            stmts.append(Stmt(len(stmts), indent, s, None, uses, True, arr))
            continue
        return None
    return stmts


def schedule(stmts: list[Stmt]) -> list[Stmt]:
    n = len(stmts)
    def_of: dict[str, int] = {}
    for st in stmts:
        if st.defs is not None:
            def_of[st.defs] = st.idx

    # Dependency edges, and how many statements still need each value.
    preds: list[set[int]] = [set() for _ in range(n)]
    succs: list[set[int]] = [set() for _ in range(n)]
    remaining_uses: list[int] = [0] * n
    for st in stmts:
        for u in st.uses:
            d = def_of.get(u)
            if d is None:
                continue
            preds[st.idx].add(d)
            succs[d].add(st.idx)
            remaining_uses[d] += 1

    # Writes to the same array keep their relative order. Different arrays are
    # independent, but chaining same-array writes costs nothing and removes any
    # question about `+=` ordering.
    last_write: dict[str, int] = {}
    for st in stmts:
        if st.is_write:
            prev = last_write.get(st.array)
            if prev is not None:
                preds[st.idx].add(prev)
                succs[prev].add(st.idx)
            last_write[st.array] = st.idx

    indeg = [len(preds[i]) for i in range(n)]
    live = [False] * n          # is this statement's value currently live
    left = list(remaining_uses)  # uses not yet consumed

    import heapq
    ready: list[tuple[int, int, int]] = []   # (delta, tiebreak=orig idx, idx)

    def delta_of(i: int) -> int:
        st = stmts[i]
        produces = 0 if st.is_write or remaining_uses[i] == 0 else 1
        dies = 0
        for u in st.uses:
            d = def_of.get(u)
            if d is not None and live[d] and left[d] == 1:
                dies += 1
        return produces - dies

    for i in range(n):
        if indeg[i] == 0:
            heapq.heappush(ready, (delta_of(i), i, i))

    out: list[Stmt] = []
    scheduled = [False] * n
    while ready:
        # The stored delta can be stale once other statements have been
        # scheduled, so re-check the best candidate and re-insert if it moved.
        d, _, i = heapq.heappop(ready)
        if scheduled[i]:
            continue
        cur = delta_of(i)
        if cur != d and ready and cur > ready[0][0]:
            heapq.heappush(ready, (cur, i, i))
            continue

        scheduled[i] = True
        out.append(stmts[i])
        st = stmts[i]
        for u in st.uses:
            dd = def_of.get(u)
            if dd is not None:
                left[dd] -= 1
                if left[dd] == 0:
                    live[dd] = False
        if not st.is_write and remaining_uses[i] > 0:
            live[i] = True
        for s2 in succs[i]:
            indeg[s2] -= 1
            if indeg[s2] == 0:
                heapq.heappush(ready, (delta_of(s2), s2, s2))

    if len(out) != n:
        raise RuntimeError("dependency cycle: scheduled %d of %d" % (len(out), n))
    return out


def peak_live(stmts: list[Stmt]) -> int:
    """Max simultaneously-live values for a given order — the quantity the
    schedule is trying to reduce, reported so a change can be judged before
    compiling anything."""
    def_of = {st.defs: k for k, st in enumerate(stmts) if st.defs is not None}
    last_use: dict[str, int] = {}
    for k, st in enumerate(stmts):
        for u in st.uses:
            if u in def_of:
                last_use[u] = k
    live: set[str] = set()
    peak = 0
    for k, st in enumerate(stmts):
        for u in list(live):
            if last_use.get(u, -1) < k:
                live.discard(u)
        if st.defs is not None and last_use.get(st.defs, -1) > k:
            live.add(st.defs)
        peak = max(peak, len(live))
    return peak


def process(path: Path, write: bool) -> tuple[int, int] | None:
    lines = path.read_text().splitlines()
    start = end = None
    for i, l in enumerate(lines):
        if LOOP.match(l):
            start = i + 1
            break
    if start is None:
        return None
    depth = 1
    for i in range(start, len(lines)):
        depth += lines[i].count("{") - lines[i].count("}")
        if depth == 0:
            end = i
            break
    if end is None:
        return None

    stmts = parse_body(lines, start, end)
    if not stmts:
        return None
    before = peak_live(stmts)
    new = schedule(stmts)
    after = peak_live(new)
    if write and after < before:
        body = [st.text for st in new]
        path.write_text("\n".join(lines[:start] + body + lines[end:]) + "\n")
    return before, after


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("paths", nargs="+", type=Path, help=".rs kernel files or directories")
    ap.add_argument("--write", action="store_true", help="rewrite in place")
    args = ap.parse_args()

    files: list[Path] = []
    for p in args.paths:
        files.extend(sorted(p.rglob("*.rs")) if p.is_dir() else [p])

    tot = imp = 0
    worst = []
    for f in files:
        r = process(f, args.write)
        if r is None:
            continue
        b, a = r
        tot += 1
        if a < b:
            imp += 1
            worst.append((b - a, b, a, f))
    worst.sort(reverse=True)
    print(f"{tot} kernel loops, {imp} improved")
    for d, b, a, f in worst[:15]:
        print(f"  peak live {b:>6} -> {a:<6} (-{100*d/b:.0f}%)  {f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
