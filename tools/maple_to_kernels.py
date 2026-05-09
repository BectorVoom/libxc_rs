#!/usr/bin/env python3
"""
Unified Maple→kernel driver: orchestrates the existing per-functional
translators (via regen_phase09.py) and split_*.py family tools behind one
CLI with consistent splitting-criteria knobs for OOM mitigation.

This is a THIN ORCHESTRATOR. It does not reimplement translation or
splitting logic; it delegates to the existing per-family tools via
subprocess and centralises knob plumbing.

# Splitting-criteria knobs

The CubeCL `#[cube]` proc-macro fans out into very large generated Rust per
kernel (see `docs/manual/Cubecl/cubecl_macro_fanout_manual.md`). Two knobs
control how that mass is distributed across files and sub-crates:

  --split-threshold N   per-cube-fn line cap (default 100000)
                        Currently a module-level constant inside
                        translate_lda_v2.py / translate_gga.py /
                        translate_mgga.py. regen_phase09.py reads the
                        translators' active value at translation time.
                        The driver does NOT mutate the constants; if a
                        non-default value is requested, it warns and
                        proceeds with whatever the translators have
                        compiled in. Larger value ⇒ FEWER per-functional
                        .rs files.

  --target-max N        per-sub-crate line cap (default 500000)
                        Honored by split_lda_subcrates.py via
                        --target-max=N. split_oversized_kernel.py uses a
                        module-level constant; non-default value warns and
                        falls back. Larger value ⇒ FEWER sub-crates.

User-direction memo (see feedback_splitting_terminology.md):
  "fewer files / less aggressive splitting" ⇒ RAISE these values
  "more files / more aggressive splitting" ⇒ LOWER these values
The arithmetic and the natural-language phrasing are inverted; always
confirm the desired file/sub-crate COUNT direction before tweaking.

# Wrapped tools

  translate (delegates to regen_phase09.py — per-functional iteration with
            atomic per-functional directory replacement, multi-sub-crate
            functionals SKIPPED per CONTEXT D-09):
    LDA, GGA, MGGA, all
      tools/regen_phase09.py --family <family> [--dry-run]

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

# Why delegate translation to regen_phase09.py?

The per-family translators (translate_*.py) operate on a SINGLE
<c_file> <func_name> at a time. translate_lda_v2.py does not have a
--batch mode at all; only translate_gga.py and translate_mgga.py do, and
even their batch outputs go to a flat directory that does not match the
actual bin-packed sub-crate layout (kernel-{lda,gga,mgga}-N/...).

regen_phase09.py already encapsulates the correct per-functional flow:
it discovers each functional's existing sub-crate location, runs the
appropriate translator into a temporary staging directory, then
atomically replaces the existing per-functional directory contents.
That is the flow we want; the driver just exposes a unified CLI on top.
"""

import argparse
import shlex
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
TOOLS = REPO_ROOT / "tools"

DEFAULT_SPLIT_THRESHOLD = 100_000
DEFAULT_TARGET_MAX = 500_000

FAMILIES = ("lda", "gga", "mgga")


def translate_cmd(family: str, split_threshold: int, dry_run: bool) -> list[list[str]]:
    if split_threshold != DEFAULT_SPLIT_THRESHOLD:
        print(
            f"WARN: regen_phase09.py does not accept --split-threshold; "
            f"requested {split_threshold} ignored. Edit SPLIT_THRESHOLD in "
            f"tools/translate_lda_v2.py / translate_gga.py / translate_mgga.py "
            f"to change. Active value at translation time wins.",
            file=sys.stderr,
        )
    cmd = [sys.executable, str(TOOLS / "regen_phase09.py"), "--family", family]
    if dry_run:
        cmd.append("--dry-run")
    return [cmd]


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
    rc = 0
    # regen_phase09.py accepts --family {lda,gga,mgga,all} directly, so we
    # do not iterate per-family here — one invocation covers the whole set.
    for cmd in translate_cmd(args.family, args.split_threshold, args.dry_run):
        rc = run_or_print(cmd, args.dry_run) or rc
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
            "not honor this; non-default value warns and falls back."
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
