#!/usr/bin/env python3
"""Demote `#[cube(launch_unchecked)]` -> `#[cube]` for kernels that no
runtime dispatch path ever invokes.

Background:
- `crates/kernel-{lda,gga,mgga}-*/` collectively contain ~3000+
  `#[cube(launch_unchecked)]` entry-kernel decls.
- The runtime dispatch (`src/eval/dispatch.rs`,
  `src/eval/{gga,mgga}_dispatch/*`) only routes a small fraction:
    * LDA: 37 routed of 41 functionals (4 deferred)
    * GGA: 105 routed of 256 functionals (rest unrouted)
    * MGGA: 25 routed of 146 functionals (rest unrouted)
- Per `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` Anti-pattern 1
  ("Every Helper Is Launchable"), the launch-wrapper boilerplate emitted
  for unreachable kernels is pure compile-time waste.

Strategy: scan each translator output module in
`crates/kernel-{lda,gga,mgga}-*/src/<func>/` for whether `<func>` is in
the dispatch routing table. If NOT routed, demote every
`#[cube(launch_unchecked)]` line to `#[cube]` for that functional.

This is the same surgery as
- tools/shrink_part_fanout.py (1396 _partN files, scope = split helpers)
- tools/demote_deferred_lda_fanout.py (38 deferred-LDA launches)

This script generalises to all unrouted functionals across all 3 families.
The routing tables are derived live from `src/model/{lda,gga,mgga}_functional.rs`
so this stays correct as routing changes.

Usage:
    python3 tools/demote_unrouted_kernels.py [--dry-run]
"""

import re
import sys
from pathlib import Path

# Reuse the shared routing helper so the translator and the demoter stay
# in sync on which functionals are "routed".
sys.path.insert(0, str(Path(__file__).resolve().parent))
from kernel_routing import collect_func_dirs, routed_funcnames  # noqa: E402


REPO_ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = REPO_ROOT / "crates"
LAUNCH_LINE_RE = re.compile(r"^#\[cube\(launch_unchecked\)\]\s*$", re.MULTILINE)


def demote_dir(d: Path, dry_run: bool) -> tuple[int, int]:
    """Return (files_modified, total_replacements)."""
    files = 0
    repls = 0
    for rs in sorted(d.glob("*.rs")):
        text = rs.read_text()
        new_text, count = LAUNCH_LINE_RE.subn("#[cube]", text)
        if count == 0 or new_text == text:
            continue
        files += 1
        repls += count
        if not dry_run:
            rs.write_text(new_text)
    return files, repls


def process_family(family: str, model_path: Path, dry_run: bool) -> None:
    func_dirs = collect_func_dirs(family)
    routed = routed_funcnames(family, known_dirs=set(func_dirs))
    print(f"=== {family.upper()} ===")
    print(f"  Routed functionals (from {model_path.name}): {len(routed)}")
    print(f"  Distinct functional dirs across sub-crates: {len(func_dirs)}")

    unrouted = sorted(name for name in func_dirs if name not in routed)
    print(f"  Unrouted (demote candidates): {len(unrouted)}")

    if not unrouted:
        print()
        return

    total_files = 0
    total_repls = 0
    for func in unrouted:
        per_func_files = 0
        per_func_repls = 0
        for d in func_dirs[func]:
            f, r = demote_dir(d, dry_run)
            per_func_files += f
            per_func_repls += r
        if per_func_repls > 0:
            print(f"    {func}: {per_func_files} file(s), {per_func_repls} demoted")
        total_files += per_func_files
        total_repls += per_func_repls

    print(f"  TOTAL: {total_files} file(s) / {total_repls} #[cube(...)] lines\n")


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    process_family(
        "lda",
        REPO_ROOT / "src" / "model" / "lda_functional.rs",
        dry_run,
    )
    process_family(
        "gga",
        REPO_ROOT / "src" / "model" / "gga_functional.rs",
        dry_run,
    )
    process_family(
        "mgga",
        REPO_ROOT / "src" / "model" / "mgga_functional.rs",
        dry_run,
    )
    if dry_run:
        print("Dry run -- no changes made.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
