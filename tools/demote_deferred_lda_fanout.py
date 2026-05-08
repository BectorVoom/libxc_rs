#!/usr/bin/env python3
"""Demote `#[cube(launch_unchecked)]` to `#[cube]` for the 4 LDA functionals
that `LdaFunctional::from_id` explicitly rejects (the deferred list).

Per `crates/kernel-lda/src/deferred.rs`:
    554 lda_c_pk09       (kxc_pol = 17,555 lines — the proc-macro RAM hog)
    259 lda_xc_ksdt
    654 lda_c_pw_erf
    590 lda_c_pmgb06

Dispatch via `LdaFunctional::from_id(...)` returns `UnsupportedFunctional`
for these IDs (verified in `src/model/lda_functional.rs:73`), so the
`<func>_<level>_<spin>` host-launch wrappers generated for them are dead
code from the host-launch perspective. Per
`docs/manual/Cubecl/cubecl_macro_fanout_manual.md` Anti-pattern 1
("Every Helper Is Launchable"), demote them to plain `#[cube]` to skip
the launch-wrapper boilerplate.

Companion to tools/shrink_part_fanout.py (which handled `_partN` split
helpers); same one-line annotation swap, narrower scope.

Usage:
    python3 tools/demote_deferred_lda_fanout.py [--dry-run]
"""

import re
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = REPO_ROOT / "crates"
DEFERRED_FUNCS = ["lda_c_pk09", "lda_xc_ksdt", "lda_c_pw_erf", "lda_c_pmgb06"]
LAUNCH_LINE_RE = re.compile(r"^#\[cube\(launch_unchecked\)\]\s*$", re.MULTILINE)


def find_func_dir(func: str) -> Path | None:
    for sub in sorted(CRATES_DIR.iterdir()):
        if not sub.is_dir() or not sub.name.startswith("kernel-lda"):
            continue
        candidate = sub / "src" / func
        if candidate.is_dir():
            return candidate
    return None


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    total_files = 0
    total_lines = 0
    for func in DEFERRED_FUNCS:
        d = find_func_dir(func)
        if d is None:
            print(f"WARN: {func} dir not found, skipping", file=sys.stderr)
            continue
        for rs in sorted(d.glob("*.rs")):
            text = rs.read_text()
            new_text, count = LAUNCH_LINE_RE.subn("#[cube]", text)
            if count == 0:
                continue
            total_files += 1
            total_lines += count
            if not dry_run:
                rs.write_text(new_text)
            print(f"  {'(dry) ' if dry_run else ''}{rs.relative_to(REPO_ROOT)}: "
                  f"{count} demoted")
    print()
    print(f"{'Would modify' if dry_run else 'Modified'}: "
          f"{total_files} files / {total_lines} #[cube(...)] lines")
    return 0


if __name__ == "__main__":
    sys.exit(main())
