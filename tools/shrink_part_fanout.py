#!/usr/bin/env python3
"""Phase 9 — shrink CubeCL macro fan-out on split-part kernels.

The translator emits every `<level>_<spin>_partN.rs` file as a standalone
`#[cube(launch_unchecked)]` function. Per the project's
`docs/manual/Cubecl/cubecl_macro_fanout_manual.md` (Anti-pattern 1: "Every
helper is launchable"), this generates host-side launch wrappers for code
that the dispatch layer never invokes — split parts have NO callers because
`src/eval/{gga,mgga}_dispatch/` references only the unsplit symbol.

This script demotes every `_partN` file from `#[cube(launch_unchecked)]` to
plain `#[cube]`, removing the launch-wrapper boilerplate while preserving
the IR-builder body. The transformation is reversible (`#[cube]` →
`#[cube(launch_unchecked)]`) and does not modify file bodies.

Sister change: tools/translate_{gga,mgga,lda}.py emit `#[cube]` for the
post-split helper files going forward, so regen no longer reintroduces the
launch annotation. This script only fixes the in-tree state from the prior
regen pass.

Usage:
    python3 tools/shrink_part_fanout.py [--dry-run]
"""

import os
import re
import sys


REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES_DIR = os.path.join(REPO_ROOT, "crates")

PART_RE = re.compile(r"^(exc|vxc|fxc|kxc|lxc)_(unpol|pol)_part\d+(_[\w]+)*\.rs$")
LAUNCH_LINE_RE = re.compile(r"^#\[cube\(launch_unchecked\)\]\s*$", re.MULTILINE)


def is_part_file(filename: str) -> bool:
    return bool(PART_RE.match(filename))


def find_part_files() -> list[str]:
    out = []
    for name in sorted(os.listdir(CRATES_DIR)):
        if not (name.startswith("kernel-gga") or name.startswith("kernel-mgga") or name.startswith("kernel-lda")):
            continue
        src = os.path.join(CRATES_DIR, name, "src")
        if not os.path.isdir(src):
            continue
        for func in sorted(os.listdir(src)):
            func_dir = os.path.join(src, func)
            if not os.path.isdir(func_dir):
                continue
            for f in sorted(os.listdir(func_dir)):
                if is_part_file(f):
                    out.append(os.path.join(func_dir, f))
    return out


def transform_file(path: str, dry_run: bool) -> tuple[bool, int]:
    """Returns (changed, count_replaced)."""
    with open(path) as f:
        text = f.read()
    if "#[cube(launch_unchecked)]" not in text:
        return (False, 0)
    new_text, count = LAUNCH_LINE_RE.subn("#[cube]", text)
    if count == 0 or new_text == text:
        return (False, 0)
    if not dry_run:
        with open(path, "w") as f:
            f.write(new_text)
    return (True, count)


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    files = find_part_files()
    if not files:
        print("No _partN.rs files found — nothing to do.", file=sys.stderr)
        return 0
    total_changed = 0
    total_replacements = 0
    by_family: dict[str, int] = {}
    for path in files:
        changed, count = transform_file(path, dry_run)
        if changed:
            total_changed += 1
            total_replacements += count
            family = path.split("/crates/kernel-")[1].split("/")[0].split("-")[0]
            by_family[family] = by_family.get(family, 0) + 1
    print(f"Inspected {len(files)} _partN.rs files")
    print(f"{'Would modify' if dry_run else 'Modified'}: {total_changed} files "
          f"({total_replacements} #[cube(...)] annotation lines)")
    for fam, n in sorted(by_family.items()):
        print(f"  by family: {fam} = {n}")
    if dry_run:
        print("\nDry run -- no changes made.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
