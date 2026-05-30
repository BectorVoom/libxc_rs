#!/usr/bin/env python3
"""
Unified Maple→kernel driver: orchestrates the per-functional translators
and the legacy split_*.py family tools behind one CLI.

This is a THIN ORCHESTRATOR. It does not reimplement translation or
splitting logic. Under the Phase 11 D-10 per-functional-subcrate model the
`translate` subcommand calls each translator's `emit_per_functional` entry
point DIRECTLY (in-process) — see "Why drive the translators directly?"
below. The legacy `split` subcommand still shells out to the split_*.py
tools and is retired in plan 11-06.

# Splitting-criteria knobs (Phase 11 D-10 model)

The CubeCL `#[cube]` proc-macro fans out into very large generated Rust per
kernel (see `docs/manual/Cubecl/cubecl_macro_fanout_manual.md`). Under the
Phase 11 per-functional-subcrate model there is now exactly ONE knob:

  --split-threshold N   per-cube-fn line cap (default 4500)
                        The SPLIT_THRESHOLD module-level constant inside
                        translate_lda_v2.py / translate_gga.py /
                        translate_mgga.py. The `translate` subcommand calls
                        each translator's `emit_per_functional` directly and
                        forwards this value, so a non-default value IS honored
                        (no longer a warn-and-fall-back). Larger value ⇒ FEWER
                        per-cube-fn split parts; D-LOCK-B floor is 4500.

There is NO per-sub-crate line cap any more: each functional is its OWN crate
under `crates/kernels/{family}/<func>/`, so the old `--target-max` knob is
obsolete (D-10). It is still accepted by the `split` subcommand for now (that
subcommand is retired in plan 11-06) but a non-default value warns.

User-direction memo (see feedback_splitting_terminology.md):
  "fewer files / less aggressive splitting" ⇒ RAISE --split-threshold
  "more files / more aggressive splitting" ⇒ LOWER --split-threshold
The arithmetic and the natural-language phrasing are inverted; always
confirm the desired file COUNT direction before tweaking.

# Wrapped tools

  translate (Phase 11 D-10: drives the per-family translators DIRECTLY via
            their `emit_per_functional` entry point, iterating Maple sources
            under libxc-master/src/maple2c/{family}_{exc,vxc}/ and emitting
            one per-functional subcrate per source into
            crates/kernels/{family}/<func>/. Replaces the stale
            regen_phase09.py in-place-replacement pipeline, which assumed
            pre-existing subcrate dirs and the pre-q07 crates/kernel-* layout
            and so cannot drive a D-10a clean-slate regen):
    LDA, GGA, MGGA, all

  split:
    LDA   tools/split_lda_subcrates.py [--target-max=N] [--dry-run]
    GGA   tools/split_oversized_kernel.py gga  [--dry-run]
          (does NOT accept --target-max; driver warns if non-default)
    MGGA  tools/split_oversized_kernel.py mgga [--dry-run]
          (does NOT accept --target-max; driver warns if non-default)

  all = translate, then split, for the selected family.

# Usage

  tools/maple_to_kernels.py translate --family all
  tools/maple_to_kernels.py translate --family gga
  tools/maple_to_kernels.py split     --family lda --target-max 500000
  tools/maple_to_kernels.py all       --family mgga --dry-run
  tools/maple_to_kernels.py all       --family all --dry-run

# Why drive the translators directly? (Phase 11 D-10)

The previous `translate` path delegated to `regen_phase09.py`, which
discovers each functional's EXISTING sub-crate dir and replaces its
contents in place. That flow cannot drive the D-10a clean-slate regen:
plan 11-03 deletes the entire old layout first, so there are no
pre-existing dirs to discover — and `regen_phase09.py` also still scans
the pre-q07 `crates/kernel-*` path prefix. It is therefore bypassed.

Instead, each translator now exposes an `emit_per_functional(c_file,
func_name, family, is_vxc_only, split_threshold)` entry point that emits a
complete per-functional subcrate (Cargo.toml + lib.rs + nested-by-output
src tree) via `tools/translate_v2/emit.py`. This driver iterates the Maple
sources for a family and calls that entry point in-process for each — no
subprocess, no staging dir, no in-place replacement.
"""

import argparse
import shlex
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
TOOLS = REPO_ROOT / "tools"
MAPLE2C = REPO_ROOT / "libxc-master" / "src" / "maple2c"

# Phase 11 D-LOCK-B: per-cube-fn line cap. 4500 leaves headroom vs the 5000
# hard cap; the three translators carry the same SPLIT_THRESHOLD default.
DEFAULT_SPLIT_THRESHOLD = 4500
# Obsolete under D-10 (per-functional subcrates have no per-subcrate cap);
# retained only for the legacy `split` subcommand, retired in plan 11-06.
DEFAULT_TARGET_MAX = 500_000

FAMILIES = ("lda", "gga", "mgga")

# tools/ on the path so the per-family translator modules import cleanly.
sys.path.insert(0, str(TOOLS))


def _hyb_family(func_name: str) -> str | None:
    """Maple `hyb_*` sources live under the base family's maple2c dir."""
    for fam in FAMILIES:
        if func_name.startswith(f"hyb_{fam}"):
            return fam
    return None


def discover_maple_sources(family: str) -> list[tuple[Path, str, bool]]:
    """Every Maple .c source for a family → (c_path, func_name, is_vxc_only).

    Scans libxc-master/src/maple2c/{family}_exc/ and {family}_vxc/. The _vxc
    sources are emitted with is_vxc_only=True. Skips Makefile.am and *Zone*
    template files (mirrors translate_*.py batch_translate)."""
    found = []
    for is_vxc, sub in ((False, f"{family}_exc"), (True, f"{family}_vxc")):
        d = MAPLE2C / sub
        if not d.is_dir():
            continue
        for c in sorted(d.glob("*.c")):
            name = c.name
            if name == "Makefile.am" or "Zone" in name:
                continue
            func_name = c.stem
            # hyb_* sources may sit under a different family's maple2c dir;
            # only take them for the family they actually belong to.
            hf = _hyb_family(func_name)
            if hf is not None and hf != family:
                continue
            found.append((c, func_name, is_vxc))
    return found


def load_thresholds_map(path):
    """Load the per-functional decision map produced by
    `tools/adaptive_split.py --all` (the size-band selector). Returns
    ``{"family/func": decision_dict}`` (each decision carries at least
    ``threshold`` and ``status``), or ``None`` when ``path`` is falsy.

    Wiring this in lets regen use each functional's own split_threshold instead
    of a single global one — capturing the super-linear per-part RSS win where a
    functional's cliff allows it, and falling back to the global value for any
    functional absent from the map. See memory
    project_compile_rss_model_chunk_sizing."""
    if not path:
        return None
    import json
    with open(path) as f:
        m = json.load(f)
    print(f"[thresholds] loaded {len(m)} per-functional decisions from {path}")
    return m


def translate_family(family: str, split_threshold: int, dry_run: bool,
                     thresholds: dict | None = None) -> int:
    """Drive `family`'s translator directly: iterate Maple sources, call the
    translator's `emit_per_functional` for each, emitting one per-functional
    subcrate per source into crates/kernels/{family}/<func>/.

    When ``thresholds`` (the adaptive_split decision map) is supplied, each
    functional regenerates at its OWN ``threshold`` (falling back to the global
    ``split_threshold`` when absent from the map)."""
    translator_mod = {
        "lda": "translate_lda_v2",
        "gga": "translate_gga",
        "mgga": "translate_mgga",
    }[family]
    mod = __import__(translator_mod)
    sources = discover_maple_sources(family)
    src = "per-functional map" if thresholds else f"global={split_threshold}"
    print(f"[{family}] {len(sources)} Maple source(s) discovered "
          f"(split_threshold: {src})")
    ok = skipped = failed = 0
    needs_sharding = []
    for c_path, func_name, is_vxc in sources:
        dec = (thresholds or {}).get(f"{family}/{func_name}")
        thr = dec.get("threshold", split_threshold) if dec else split_threshold
        note = ""
        if dec:
            note = f" [thr={thr} {dec.get('status', '?')}]"
            if dec.get("status") == "needs-sharding":
                needs_sharding.append(func_name)
        if dry_run:
            print(f"  [dry-run] would emit {family}/{func_name} "
                  f"(vxc_only={is_vxc}) from {c_path.name}{note}")
            ok += 1
            continue
        try:
            mods = mod.emit_per_functional(
                str(c_path), func_name, family, is_vxc, thr)
            print(f"  OK: {family}/{func_name} ({len(mods)} output module(s)){note}")
            ok += 1
        except RuntimeError as e:
            # check_unimplemented_math and similar expected skips.
            print(f"  SKIP: {family}/{func_name}: {e}", file=sys.stderr)
            skipped += 1
        except Exception as e:  # noqa: BLE001 — surface, count, keep going
            print(f"  FAIL: {family}/{func_name}: {type(e).__name__}: {e}",
                  file=sys.stderr)
            failed += 1
    print(f"[{family}] ok={ok} skipped={skipped} failed={failed}")
    if needs_sharding:
        print(f"[{family}] {len(needs_sharding)} functional(s) flagged "
              f"needs-sharding (run `split` next): {', '.join(needs_sharding)}",
              file=sys.stderr)
    return 1 if failed else 0


def split_cmd(family: str, target_max: int) -> list[list[str]]:
    if family == "lda":
        return [[
            sys.executable, str(TOOLS / "split_lda_subcrates.py"),
            f"--target-max={target_max}",
        ]]
    if target_max != DEFAULT_TARGET_MAX:
        print(
            f"WARN: split_oversized_kernel.py does not accept --target-max; "
            f"requested {target_max} ignored. Edit TARGET_MAX in "
            f"tools/split_oversized_kernel.py to change.",
            file=sys.stderr,
        )
    return [[
        sys.executable, str(TOOLS / "split_oversized_kernel.py"),
        family,
    ]]


def run_or_print(cmd: list[str], dry_run: bool) -> int:
    pretty = " ".join(shlex.quote(p) for p in cmd)
    # When dry_run is True, the underlying tool already received --dry-run via
    # its own CLI flag (translate via regen_phase09.py --dry-run; split via
    # the per-tool --dry-run that do_split appends). We still execute the
    # subprocess so the tool can print its own dry-run plan, but mark the
    # invocation in our log as a forwarded dry-run for clarity.
    label = "[dry-run forwarded]" if dry_run else "[run]"
    print(f"{label} {pretty}", flush=True)
    return subprocess.run(cmd, cwd=str(REPO_ROOT)).returncode


def _split_families(family: str) -> list[str]:
    """Family list for split_*.py invocations (no 'all' passthrough)."""
    return list(FAMILIES) if family == "all" else [family]


def do_translate(args: argparse.Namespace) -> int:
    """Phase 11 D-10: drive the per-family translators directly (no subprocess,
    no regen_phase09.py). Each family's emit_per_functional writes complete
    per-functional subcrates under crates/kernels/{family}/<func>/."""
    thresholds = load_thresholds_map(getattr(args, "thresholds_map", None))
    rc = 0
    families = list(FAMILIES) if args.family == "all" else [args.family]
    for fam in families:
        rc = translate_family(fam, args.split_threshold, args.dry_run,
                              thresholds) or rc
        if rc and not args.dry_run:
            return rc
    return rc


def do_split(args: argparse.Namespace) -> int:
    rc = 0
    for fam in _split_families(args.family):
        cmds = split_cmd(fam, args.target_max)
        if args.dry_run:
            cmds = [c + ["--dry-run"] for c in cmds]
        for cmd in cmds:
            rc = run_or_print(cmd, args.dry_run) or rc
            if rc and not args.dry_run:
                return rc
    return rc


def do_all(args: argparse.Namespace) -> int:
    rc = do_translate(args)
    if rc and not args.dry_run:
        return rc
    return do_split(args) or rc


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="maple_to_kernels.py",
        description=(
            "Unified Maple→kernel driver: translate + split per family with "
            "consistent splitting-criteria knobs."
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help=(
            "Forward --dry-run to each underlying tool. The tool runs but "
            "produces only a planning/diff summary, no on-disk changes."
        ),
    )

    sub = parser.add_subparsers(dest="cmd", required=True)

    def add_family(p: argparse.ArgumentParser) -> None:
        p.add_argument(
            "--family", choices=("lda", "gga", "mgga", "all"), default="all",
            help="Functional family to operate on (default: all).",
        )

    p_t = sub.add_parser("translate", help="Run Maple→Rust translators.")
    add_family(p_t)
    p_t.add_argument(
        "--split-threshold", type=int, default=DEFAULT_SPLIT_THRESHOLD,
        help=(
            f"Per-cube-fn line cap (default {DEFAULT_SPLIT_THRESHOLD}). "
            "Larger ⇒ fewer per-functional .rs files. LDA translator does "
            "not honor this; non-default value warns and falls back. "
            "Used as the FALLBACK for functionals absent from --thresholds-map."
        ),
    )
    p_t.add_argument(
        "--thresholds-map", default=None, metavar="PATH",
        help=(
            "JSON decision map from `adaptive_split.py --all` (per-functional "
            "size-band thresholds). When given, each functional regenerates at "
            "its own threshold; --split-threshold is the fallback."
        ),
    )
    p_t.set_defaults(func=do_translate)

    p_s = sub.add_parser("split", help="Run post-hoc sub-crate splitters.")
    add_family(p_s)
    p_s.add_argument(
        "--target-max", type=int, default=DEFAULT_TARGET_MAX,
        help=(
            f"Per-sub-crate line cap (default {DEFAULT_TARGET_MAX}). "
            "Larger ⇒ fewer sub-crates. GGA/MGGA splitter does not honor "
            "this; non-default value warns and falls back."
        ),
    )
    p_s.set_defaults(func=do_split)

    p_a = sub.add_parser("all", help="Run translate then split.")
    add_family(p_a)
    p_a.add_argument(
        "--split-threshold", type=int, default=DEFAULT_SPLIT_THRESHOLD,
    )
    p_a.add_argument(
        "--thresholds-map", default=None, metavar="PATH",
        help="Per-functional size-band map from adaptive_split.py --all "
             "(fallback: --split-threshold).",
    )
    p_a.add_argument(
        "--target-max", type=int, default=DEFAULT_TARGET_MAX,
    )
    p_a.set_defaults(func=do_all)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
