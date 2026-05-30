#!/usr/bin/env python3
"""Prototype: per-functional size-band split-threshold selector.

WHY
---
The global `split_threshold` (4500) is a blunt instrument. Measured 2026-05-30
(see memory project_compile_rss_model_chunk_sizing): per-part rustc compile RSS
is steeply SUPER-LINEAR in part SIZE, so smaller chunks slash peak RSS
(gga_c_pbe: 16.5 GB @4500 -> 6.0 GB @2500 -> 3.3 GB @2000). BUT below a
functional-specific CLIFF the part count EXPLODES, regressing time/disk and
giving no further RSS win (gga_c_acgga: 23 parts @4500 -> 4752 @3000). The cliff
location varies wildly per functional (pbe ~1750, acgga ~3600), so no single
global value is good: 2500 is ideal for pbe and catastrophic for acgga.

WHAT
----
Pick, PER FUNCTIONAL, the LOWEST threshold (= smallest part size = lowest RSS)
that (a) does not cross that functional's explosion cliff and (b) gets the
largest part down into a target SIZE BAND. Functionals already small are left at
the default. Functionals whose largest part is CSE-IRREDUCIBLE *and* whose cliff
is near the default are flagged `needs-sharding` (threshold can't help them — the
fix is split_per_functional_subcrate.py, a separate process per part).

HOW (robust)
------------
Walk a descending threshold ladder; for each candidate, emit the functional to a
temp dir IN AN ISOLATED SUBPROCESS (RLIMIT_AS memory cap + wall-clock timeout)
and count `#[cube]` parts + max part size (lines). Isolation matters: an
exploding functional builds tens of thousands of part-source strings and will
OOM the host otherwise (observed). A killed/timed-out subprocess IS the cliff
signal. Stop at the cliff (back off to the last safe threshold) or once the max
part is within the size band; emit a {(family,func): threshold} decision map.

This is a PROTOTYPE: the selection uses real emits for fidelity (slow on big
functionals). A production version would add a dry-run part counter to
per_functional.emit_functional (count the split plan without assembling source).
"""

import argparse
import glob
import json
import os
import resource
import subprocess
import sys
import tempfile

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TOOLS = os.path.join(REPO, "tools")

# --- size-band knobs ---------------------------------------------------------
DEFAULT_THRESHOLD = 4500       # the current global default / ladder ceiling
SIZE_TARGET = 2500             # land the largest part at <= this many lines
                               # (~6 GB/crate regime on pbe; good RSS/time balance)
LADDER = [4250, 4000, 3750, 3500, 3250, 3000, 2750, 2500, 2250, 2000]
EXPLOSION_FACTOR = 5           # part-count jump >= this vs the previous (safe)
                               # candidate == cliff
SKIP_IF_MAX_PART_UNDER = 2500  # functional already fine -> keep default
SUBPROC_MEM_GB = 6             # RLIMIT_AS per isolated emit (normal regen << 1 GB;
                               # an explosion blows past this and is killed)
SUBPROC_TIMEOUT_S = 120        # wall-clock cap per isolated emit

_MOD = {"gga": "translate_gga", "lda": "translate_lda_v2", "mgga": "translate_mgga"}


# --- in-process emit + measure (the subprocess body) -------------------------
def _emit_measure(family, func, cfile, is_vxc, threshold):
    """Emit one functional at one threshold to a temp dir; return
    (part_count, max_part_lines). Run ONLY inside an isolated subprocess."""
    import importlib
    sys.path.insert(0, TOOLS)
    from translate_v2 import emit
    root = tempfile.mkdtemp(prefix="adaptive_")
    try:
        emit.set_kernels_root(root)
        mod = importlib.import_module(_MOD[family])
        mod.emit_per_functional(cfile, func, family, is_vxc, threshold)
        parts = 0
        max_lines = 0
        for p in glob.glob(os.path.join(root, family, func, "src", "**", "*.rs"),
                           recursive=True):
            with open(p) as f:
                text = f.read()
            parts += text.count("#[cube")
            n = text.count("\n")
            if n > max_lines:
                max_lines = n
        return parts, max_lines
    finally:
        subprocess.run(["rm", "-rf", root], check=False)


def _emit_count_isolated(family, func, cfile, is_vxc, threshold):
    """Spawn `_emit_measure` in a memory-capped, time-limited subprocess.

    Returns (parts, max_lines) on success, or None when the subprocess is
    killed / times out / errors -> treated as an EXPLOSION (cliff) signal."""
    def _limit():
        cap = SUBPROC_MEM_GB * 1024 * 1024 * 1024
        try:
            resource.setrlimit(resource.RLIMIT_AS, (cap, cap))
        except (ValueError, OSError):
            pass

    env = dict(os.environ, PYTHONPATH=TOOLS)
    try:
        r = subprocess.run(
            [sys.executable, os.path.abspath(__file__), "--emit-count",
             family, func, cfile, "1" if is_vxc else "0", str(threshold)],
            capture_output=True, text=True, env=env, cwd=REPO,
            preexec_fn=_limit, timeout=SUBPROC_TIMEOUT_S,
        )
    except subprocess.TimeoutExpired:
        return None
    if r.returncode != 0:
        return None
    for line in r.stdout.splitlines():
        if line.startswith("COUNT "):
            _, parts, maxl = line.split()
            return int(parts), int(maxl)
    return None


# --- per-functional selection ------------------------------------------------
def select_threshold(family, func, cfile, is_vxc):
    """Return a decision dict for one functional."""
    base = _emit_count_isolated(family, func, cfile, is_vxc, DEFAULT_THRESHOLD)
    if base is None:
        return {"family": family, "func": func, "threshold": DEFAULT_THRESHOLD,
                "parts": None, "max_part": None, "status": "base-emit-failed"}
    base_parts, base_max = base

    # Already small enough: nothing worth splitting; keep the default.
    if base_max <= SKIP_IF_MAX_PART_UNDER:
        return {"family": family, "func": func, "threshold": DEFAULT_THRESHOLD,
                "parts": base_parts, "max_part": base_max, "status": "default-ok"}

    prev_t, prev_parts, prev_max = DEFAULT_THRESHOLD, base_parts, base_max
    for t in LADDER:
        res = _emit_count_isolated(family, func, cfile, is_vxc, t)
        if res is None:
            # cliff: this candidate exploded past the memory/time cap.
            return _decide(family, func, prev_t, prev_parts, prev_max,
                           cliff_at=t)
        parts, maxl = res
        if parts >= EXPLOSION_FACTOR * max(prev_parts, 1):
            # cliff: part count jumped -> back off to the last safe threshold.
            return _decide(family, func, prev_t, prev_parts, prev_max,
                           cliff_at=t)
        if maxl <= SIZE_TARGET:
            # size band reached without exploding -> this is the pick.
            return {"family": family, "func": func, "threshold": t,
                    "parts": parts, "max_part": maxl, "status": "in-band"}
        prev_t, prev_parts, prev_max = t, parts, maxl
    # walked the whole ladder without reaching the band or a cliff.
    return _decide(family, func, prev_t, prev_parts, prev_max, cliff_at=None)


def _decide(family, func, t, parts, maxl, cliff_at):
    """Last-safe threshold reached without landing in the size band. If the
    largest part is still big, it is CSE-irreducible at this threshold -> the
    functional needs SHARDING, not finer chunking."""
    if maxl > SIZE_TARGET:
        status = "needs-sharding"   # big irreducible part; threshold can't help
    else:
        status = "in-band"
    return {"family": family, "func": func, "threshold": t, "parts": parts,
            "max_part": maxl, "status": status,
            "cliff_below": cliff_at}


# --- CLI ---------------------------------------------------------------------
def _discover(family):
    sys.path.insert(0, TOOLS)
    import maple_to_kernels as mk
    return [(str(cf), fn, is_vxc) for cf, fn, is_vxc in mk.discover_maple_sources(family)]


DEMO = [
    ("gga", "gga_c_pbe", "libxc-master/src/maple2c/gga_exc/gga_c_pbe.c", False),
    ("gga", "gga_c_acgga", "libxc-master/src/maple2c/gga_exc/gga_c_acgga.c", False),
    ("gga", "gga_x_optx", "libxc-master/src/maple2c/gga_exc/gga_x_optx.c", False),
    ("mgga", "mgga_x_task", "libxc-master/src/maple2c/mgga_exc/mgga_x_task.c", False),
]


def _print_row(d):
    cliff = d.get("cliff_below")
    extra = f"  (cliff<{cliff})" if cliff else ""
    print(f"  {d['family']}/{d['func']:<18} -> thr={str(d['threshold']):<5} "
          f"parts={str(d['parts']):<5} max_part={str(d['max_part']):<5}L  "
          f"[{d['status']}]{extra}", flush=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--emit-count", nargs=5,
                    metavar=("FAM", "FUNC", "CFILE", "ISVXC", "THR"),
                    help="(internal) emit one functional at THR; print COUNT parts maxlines")
    ap.add_argument("--demo", action="store_true",
                    help="run selection on a small built-in sample")
    ap.add_argument("--select", nargs="+",
                    metavar="FAM FUNC CFILE [ISVXC]",
                    help="select threshold for one functional")
    ap.add_argument("--all", choices=("lda", "gga", "mgga", "all"),
                    help="select for every functional in a family; write JSON map")
    ap.add_argument("--out", default="tools/adaptive_thresholds.json")
    args = ap.parse_args()

    if args.emit_count:
        fam, func, cfile, isvxc, thr = args.emit_count
        parts, maxl = _emit_measure(fam, func, cfile, isvxc == "1", int(thr))
        print(f"COUNT {parts} {maxl}")
        return 0

    if args.demo:
        print(f"size-band selector (SIZE_TARGET={SIZE_TARGET}L, "
              f"explosion>={EXPLOSION_FACTOR}x, ladder bottom={LADDER[-1]}):")
        for fam, func, cfile, isvxc in DEMO:
            _print_row(select_threshold(fam, func, cfile, isvxc))
        return 0

    if args.select:
        fam, func, cfile = args.select[:3]
        isvxc = len(args.select) > 3 and args.select[3] in ("1", "true", "vxc")
        _print_row(select_threshold(fam, func, cfile, isvxc))
        return 0

    if args.all:
        fams = ["lda", "gga", "mgga"] if args.all == "all" else [args.all]
        decisions = {}
        for fam in fams:
            for cfile, func, isvxc in _discover(fam):
                d = select_threshold(fam, func, cfile, isvxc)
                _print_row(d)
                decisions[f"{fam}/{func}"] = d
        with open(args.out, "w") as f:
            json.dump(decisions, f, indent=2)
        print(f"\nwrote {len(decisions)} decisions -> {args.out}")
        return 0

    ap.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
