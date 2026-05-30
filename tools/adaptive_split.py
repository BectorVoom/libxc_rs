#!/usr/bin/env python3
"""Per-functional size-band split-threshold selector.

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

HOW
---
Walk a descending threshold ladder; for each candidate use the DRY-RUN part
counter `per_functional.count_functional` (via `emit_per_functional(...,
count_only=True)`) — it computes the exact `#[cube]` part count + max part size
from the split PLAN, with NO Rust source assembled and NO files written
(validated exact vs real emit, ~1 s even on functionals that explode to thousands
of parts). Stop at the cliff (part count jumps >= EXPLOSION_FACTOR) or once the
largest part lands in the size band; emit a {(family,func): decision} JSON map
consumed by `maple_to_kernels.py translate --thresholds-map`.

Requires the `src/model` routing symlink (same prereq as regen — see memory
project_kernel_routing_model_path_stale): `ln -sfn ../crates/libxc-core/src/model
src/model`.
"""

import argparse
import importlib
import json
import os
import sys

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

_MOD = {"gga": "translate_gga", "lda": "translate_lda_v2", "mgga": "translate_mgga"}
_MODS = {}


def _mod(family):
    if family not in _MODS:
        sys.path.insert(0, TOOLS)
        _MODS[family] = importlib.import_module(_MOD[family])
    return _MODS[family]


def _count(family, func, cfile, is_vxc, threshold):
    """Exact (#[cube] parts, max_part_lines, max_part_irreducible) via the
    dry-run counter — no emit, no disk. Cheap enough to call across the whole
    ladder for every functional."""
    return _mod(family).emit_per_functional(
        cfile, func, family, is_vxc, threshold, count_only=True)


# --- per-functional selection ------------------------------------------------
def select_threshold(family, func, cfile, is_vxc):
    """Return a decision dict for one functional."""
    base_parts, base_max, base_irred = _count(
        family, func, cfile, is_vxc, DEFAULT_THRESHOLD)

    # Already small enough: nothing worth splitting; keep the default.
    if base_max <= SKIP_IF_MAX_PART_UNDER:
        return {"family": family, "func": func, "threshold": DEFAULT_THRESHOLD,
                "parts": base_parts, "max_part": base_max, "status": "default-ok"}

    # The dominant part is ALREADY CSE-irreducible at the loosest (default) cap.
    # A lower cap only cuts it into MORE tiny chunks that reject the same way, so
    # it is irreducible at every threshold -> tuning can't shrink it. Keep the
    # default and flag needs-sharding (and skip the whole ladder — the big speed
    # win for pk09/tpssloc-class monsters).
    if base_irred:
        return _decide(family, func, DEFAULT_THRESHOLD, base_parts, base_max,
                       cliff_at=None)

    prev_t, prev_parts, prev_max = DEFAULT_THRESHOLD, base_parts, base_max
    for t in LADDER:
        parts, maxl, irred = _count(family, func, cfile, is_vxc, t)
        if parts >= EXPLOSION_FACTOR * max(prev_parts, 1):
            # cliff: part count jumped -> back off to the last safe threshold.
            return _decide(family, func, prev_t, prev_parts, prev_max,
                           cliff_at=t)
        if maxl <= SIZE_TARGET:
            # size band reached without exploding -> this is the pick.
            return {"family": family, "func": func, "threshold": t,
                    "parts": parts, "max_part": maxl, "status": "in-band"}
        if irred:
            # the dominant part just became CSE-irreducible -> stop; lowering
            # further only fragments OTHER parts without shrinking the max.
            return _decide(family, func, t, parts, maxl, cliff_at=None)
        prev_t, prev_parts, prev_max = t, parts, maxl
    # walked the whole ladder without reaching the band or a cliff.
    return _decide(family, func, prev_t, prev_parts, prev_max, cliff_at=None)


def _decide(family, func, t, parts, maxl, cliff_at):
    """Last-safe threshold reached without landing in the size band. If the
    largest part is still big, it is CSE-irreducible at this threshold -> the
    functional needs SHARDING, not finer chunking."""
    status = "needs-sharding" if maxl > SIZE_TARGET else "in-band"
    return {"family": family, "func": func, "threshold": t, "parts": parts,
            "max_part": maxl, "status": status, "cliff_below": cliff_at}


# --- CLI ---------------------------------------------------------------------
def _ensure_routing():
    """The dry counter builds the FamilyAdapter, which consults kernel_routing
    (src/model). Fail early with an actionable hint when the symlink is absent."""
    if not os.path.exists(os.path.join(REPO, "src", "model")):
        moved = os.path.join(REPO, "crates", "libxc-core", "src", "model")
        hint = (" — create it: `ln -sfn ../crates/libxc-core/src/model src/model`"
                if os.path.isdir(moved) else "")
        print(f"ERROR: src/model routing path missing{hint}", file=sys.stderr)
        return False
    return True


def _discover(family):
    sys.path.insert(0, TOOLS)
    import maple_to_kernels as mk
    return [(str(cf), fn, is_vxc)
            for cf, fn, is_vxc in mk.discover_maple_sources(family)]


DEMO = [
    ("gga", "gga_c_pbe", "libxc-master/src/maple2c/gga_exc/gga_c_pbe.c", False),
    ("gga", "gga_c_acgga", "libxc-master/src/maple2c/gga_exc/gga_c_acgga.c", False),
    ("gga", "gga_x_optx", "libxc-master/src/maple2c/gga_exc/gga_x_optx.c", False),
    ("mgga", "mgga_x_task", "libxc-master/src/maple2c/mgga_exc/mgga_x_task.c", False),
    ("lda", "lda_c_pk09", "libxc-master/src/maple2c/lda_exc/lda_c_pk09.c", False),
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
    ap.add_argument("--demo", action="store_true",
                    help="run selection on a small built-in sample")
    ap.add_argument("--select", nargs="+", metavar="FAM FUNC CFILE [ISVXC]",
                    help="select threshold for one functional")
    ap.add_argument("--all", choices=("lda", "gga", "mgga", "all"),
                    help="select for every functional in a family; write JSON map")
    ap.add_argument("--out", default="tools/adaptive_thresholds.json")
    args = ap.parse_args()

    if not _ensure_routing():
        return 1

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
        tuned = sharding = 0
        for fam in fams:
            for cfile, func, isvxc in _discover(fam):
                try:
                    d = select_threshold(fam, func, cfile, isvxc)
                except RuntimeError as e:        # check_unimplemented_math etc.
                    print(f"  SKIP {fam}/{func}: {e}", file=sys.stderr)
                    continue
                _print_row(d)
                decisions[f"{fam}/{func}"] = d
                if d["status"] == "needs-sharding":
                    sharding += 1
                elif d["threshold"] != DEFAULT_THRESHOLD:
                    tuned += 1
        with open(args.out, "w") as f:
            json.dump(decisions, f, indent=2)
        print(f"\nwrote {len(decisions)} decisions -> {args.out}  "
              f"({tuned} tuned below default, {sharding} need sharding)")
        return 0

    ap.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
