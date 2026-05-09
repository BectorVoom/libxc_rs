#!/usr/bin/env python3
"""
Unified Maple→kernel driver: orchestrates the existing translate_*.py and
split_*.py family tools behind one CLI with consistent splitting-criteria
knobs for OOM mitigation.

This is a THIN ORCHESTRATOR. It does not reimplement translation or
splitting logic; it delegates to the existing per-family tools via
subprocess and centralises knob plumbing.

# Splitting-criteria knobs

The CubeCL `#[cube]` proc-macro fans out into very large generated Rust per
kernel (see `docs/manual/Cubecl/cubecl_macro_fanout_manual.md`). Two knobs
control how that mass is distributed across files and sub-crates:

  --split-threshold N   per-cube-fn line cap (default 100000)
                        Used by the translator. Larger value ⇒ each
                        translated `#[cube]` function file holds more
                        lines ⇒ FEWER per-functional `.rs` files.

  --target-max N        per-sub-crate line cap (default 500000)
                        Used by the splitter. Larger value ⇒ each
                        kernel-* sub-crate holds more lines ⇒ FEWER
                        sub-crates.

User-direction memo (see feedback_splitting_terminology.md):
  "fewer files / less aggressive splitting" ⇒ RAISE these values
  "more files / more aggressive splitting" ⇒ LOWER these values
The arithmetic and the natural-language phrasing are inverted; always
confirm the desired file/sub-crate COUNT direction before tweaking.

# Wrapped tools

  translate:
    LDA   tools/translate_lda_v2.py  --batch --write-to <dir>
          (does NOT accept --split-threshold; driver warns if non-default)
    GGA   tools/translate_gga.py     --batch --write-to <dir> [--split-threshold N]
    MGGA  tools/translate_mgga.py    --batch --write-to <dir> [--split-threshold N]

  split:
    LDA   tools/split_lda_subcrates.py [--target-max=N] [--dry-run]
    GGA   tools/split_oversized_kernel.py gga  [--dry-run]
          (does NOT accept --target-max; driver warns if non-default)
    MGGA  tools/split_oversized_kernel.py mgga [--dry-run]
          (does NOT accept --target-max; driver warns if non-default)

  all = translate, then split, for the selected family.

# Usage

  tools/maple_to_kernels.py translate --family all
  tools/maple_to_kernels.py translate --family gga --split-threshold 100000
  tools/maple_to_kernels.py split     --family lda --target-max 500000
  tools/maple_to_kernels.py all       --family mgga --dry-run
  tools/maple_to_kernels.py all       --family all --dry-run

The companion script `tools/regen_phase09.py` is a Phase 9 Plan 09-04
specific orchestrator (single-sub-crate functionals only). It is not
replaced by this driver; both coexist.
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

WRITE_TO = {
    "lda": REPO_ROOT / "crates" / "kernel-lda" / "src" / "funcs",
    "gga": REPO_ROOT / "crates" / "kernel-gga" / "src" / "funcs",
    "mgga": REPO_ROOT / "crates" / "kernel-mgga" / "src" / "funcs",
}

FAMILIES = ("lda", "gga", "mgga")


def families_from_arg(family: str) -> list[str]:
    return list(FAMILIES) if family == "all" else [family]


def translate_cmd(family: str, split_threshold: int) -> list[list[str]]:
    write_to = str(WRITE_TO[family])
    if family == "lda":
        if split_threshold != DEFAULT_SPLIT_THRESHOLD:
            print(
                f"WARN: translate_lda_v2.py does not accept --split-threshold; "
                f"requested {split_threshold} ignored. Edit SPLIT_THRESHOLD in "
                f"tools/translate_lda_v2.py to change.",
                file=sys.stderr,
            )
        return [[
            sys.executable, str(TOOLS / "translate_lda_v2.py"),
            "--batch", "--write-to", write_to,
        ]]
    tool = "translate_gga.py" if family == "gga" else "translate_mgga.py"
    return [[
        sys.executable, str(TOOLS / tool),
        "--batch", "--write-to", write_to,
        "--split-threshold", str(split_threshold),
    ]]


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
    if dry_run:
        print(f"[dry-run] {pretty}")
        return 0
    print(f"[run] {pretty}", flush=True)
    return subprocess.run(cmd, cwd=str(REPO_ROOT)).returncode


def do_translate(args: argparse.Namespace) -> int:
    rc = 0
    for fam in families_from_arg(args.family):
        for cmd in translate_cmd(fam, args.split_threshold):
            rc = run_or_print(cmd, args.dry_run) or rc
            if rc and not args.dry_run:
                return rc
    return rc


def do_split(args: argparse.Namespace) -> int:
    rc = 0
    for fam in families_from_arg(args.family):
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
        help="Print the underlying tool invocations without executing them.",
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
