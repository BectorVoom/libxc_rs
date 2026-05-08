#!/usr/bin/env python3
"""Split every oversized MGGA sub-crate into per-file bin-packed sub-crates.

Background: rebatch_mgga.py bin-packs at FUNCTIONAL granularity (≤50K
lines/crate). When a single functional exceeds 50K it goes in a solo crate;
those solo crates OOM rustc during compile. This script does PER-FILE
bin-packing within each oversized functional, producing kernel-mgga-Na, Nb,
Nc, ... sub-crates each ≤50K lines.

Each new crate keeps the same module path `<functional>::<file>` so call
sites are unaffected.

Usage: python3 tools/split_oversized_mgga.py [--dry-run] [--target-max LINES]
"""

import os
import sys
import shutil

WORKSPACE = "/workspace"
CRATES_DIR = os.path.join(WORKSPACE, "crates")
# TARGET_MAX is the per-SUB-CRATE bin-packing budget (sum of all .rs lines
# inside one crates/kernel-mgga-NX/ tree), NOT a per-#[cube] function line
# cap. The per-function line cap lives in tools/translate_mgga.py
# (SPLIT_THRESHOLD = 18000 as of Phase 9 Plan 09-04, CONTEXT D-06). The 50K
# bin budget here is unrelated to that 18K function cap and stays as-is per
# CONTEXT D-09 (sub-crate re-bin-packing is OUT of scope for Phase 9).
TARGET_MAX = 50000
SUFFIXES = list("abcdefghijklmnop")  # 16 should suffice (largest functional needs 12)


def find_oversized():
    """Return list of (crate_name, functional, [(file, lines), ...])."""
    out = []
    for entry in sorted(os.listdir(CRATES_DIR)):
        if not entry.startswith("kernel-mgga-") or entry == "kernel-mgga":
            continue
        # Skip already-split crates (kernel-mgga-Na, Nb, etc.)
        if any(entry.endswith(s) for s in ['a', 'b', 'c', 'd', 'e', 'f', 'g',
                                           'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p']):
            # Heuristic: ends with single letter after digits (e.g. mgga-7a)
            stripped = entry[len("kernel-mgga-"):]
            if stripped[-1].isalpha() and stripped[:-1].isdigit():
                continue
        src_dir = os.path.join(CRATES_DIR, entry, "src")
        if not os.path.isdir(src_dir):
            continue
        for item in sorted(os.listdir(src_dir)):
            path = os.path.join(src_dir, item)
            if not os.path.isdir(path):
                continue
            files = []
            for rs in os.listdir(path):
                if rs.endswith(".rs") and rs != "mod.rs":
                    n = sum(1 for _ in open(os.path.join(path, rs)))
                    files.append((rs, n))
            total = sum(n for _, n in files)
            if total > TARGET_MAX:
                out.append((entry, item, files))
    return out


def bin_pack(files, target):
    bins = []
    for fname, n in sorted(files, key=lambda x: -x[1]):
        for b in bins:
            if sum(x[1] for x in b) + n <= target:
                b.append((fname, n))
                break
        else:
            bins.append([(fname, n)])
    bins.sort(key=lambda b: sorted(x[0] for x in b)[0])
    return bins


CARGO_TOML_TEMPLATE = """[package]
name = "libxc-kernel-mgga-{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = {{ version = "0.10.0", default-features = false, features = ["cpu"] }}
libxc-kernel-math = {{ path = "../kernel-math" }}

[profile.dev]
debug = 0
codegen-units = 16
opt-level = 0
incremental = true

[profile.test]
debug = 0
codegen-units = 16
opt-level = 0
incremental = true
"""


def make_lib_rs(crate_name, functional):
    return (
        '#![allow(clippy::excessive_precision)]\n'
        '#![allow(clippy::needless_late_init)]\n'
        '#![allow(clippy::too_many_arguments)]\n'
        '\n'
        f'//! MGGA kernel translations: {crate_name} ({functional} subset).\n'
        '\n'
        f'pub mod {functional};\n'
    )


def make_mod_rs(file_subset, functional):
    lines = [f'//! {functional.upper()} kernel — split into per-function files.\n', '\n']
    for fname, _ in sorted(file_subset, key=lambda x: x[0]):
        modname = fname[:-3]
        lines.append(f'pub mod {modname};\n')
    return ''.join(lines)


def update_workspace_and_aggregator(crate_renames):
    """crate_renames: list of (old_crate, [new_crate, ...])."""
    ws_cargo = os.path.join(WORKSPACE, "Cargo.toml")
    mg_cargo = os.path.join(CRATES_DIR, "kernel-mgga", "Cargo.toml")
    mg_lib = os.path.join(CRATES_DIR, "kernel-mgga", "src", "lib.rs")

    # Read all
    with open(ws_cargo) as f: ws_content = f.read()
    with open(mg_cargo) as f: mg_content = f.read()
    with open(mg_lib) as f:   lib_content = f.read()

    for old, news in crate_renames:
        n_old = old.replace("kernel-mgga-", "")  # e.g. "20"
        # workspace dep line
        old_dep_ws = f'libxc-kernel-mgga-{n_old} = {{ path = "crates/{old}" }}'
        new_dep_ws = '\n'.join(
            f'libxc-kernel-mgga-{nc.replace("kernel-mgga-", "")} = '
            f'{{ path = "crates/{nc}" }}' for nc in news
        )
        ws_content = ws_content.replace(old_dep_ws, new_dep_ws)
        # workspace member entry
        old_member = f'    "crates/{old}",'
        new_members = '\n'.join(f'    "crates/{nc}",' for nc in news)
        ws_content = ws_content.replace(old_member, new_members)

        # aggregator Cargo dep
        old_dep_mg = f'libxc-kernel-mgga-{n_old} = {{ path = "../{old}" }}'
        new_dep_mg = '\n'.join(
            f'libxc-kernel-mgga-{nc.replace("kernel-mgga-", "")} = '
            f'{{ path = "../{nc}" }}' for nc in news
        )
        mg_content = mg_content.replace(old_dep_mg, new_dep_mg)

        # aggregator lib.rs re-export
        old_export = f'pub use libxc_kernel_mgga_{n_old} as batch{n_old};'
        new_exports = '\n'.join(
            f'pub use libxc_kernel_mgga_{nc.replace("kernel-mgga-", "")} '
            f'as batch{nc.replace("kernel-mgga-", "")};' for nc in news
        )
        lib_content = lib_content.replace(old_export, new_exports)

    with open(ws_cargo, 'w') as f: f.write(ws_content)
    with open(mg_cargo, 'w') as f: f.write(mg_content)
    with open(mg_lib, 'w') as f:   f.write(lib_content)


def main():
    dry_run = '--dry-run' in sys.argv

    oversized = find_oversized()
    print(f"Found {len(oversized)} oversized functionals (>{TARGET_MAX} lines):\n")

    plan = []
    for crate, func, files in oversized:
        bins = bin_pack(files, TARGET_MAX)
        n_old = crate.replace("kernel-mgga-", "")
        # Skip if already a multi-letter suffix (already split)
        if not n_old.isdigit():
            print(f"  SKIP {crate} (non-pure-numeric suffix)")
            continue
        new_crates = [f"kernel-mgga-{n_old}{SUFFIXES[i]}" for i in range(len(bins))]
        total = sum(n for _, n in files)
        print(f"  {crate}/{func}: {total:,}L → {len(bins)} sub-crates "
              f"({', '.join(c.replace('kernel-mgga-', '') for c in new_crates)})")
        plan.append((crate, func, bins, new_crates))

    if dry_run:
        print(f"\nTotal new crates: {sum(len(p[3]) for p in plan)}")
        print(f"Total deletions: {len(plan)}")
        print("\nDry run -- no changes made.")
        return

    # Execute
    for crate, func, bins, new_crates in plan:
        src_dir = os.path.join(CRATES_DIR, crate, "src", func)
        for new_crate, b in zip(new_crates, bins):
            new_dir = os.path.join(CRATES_DIR, new_crate)
            new_func = os.path.join(new_dir, "src", func)
            os.makedirs(new_func, exist_ok=True)
            for fname, _n in b:
                shutil.copy2(os.path.join(src_dir, fname),
                             os.path.join(new_func, fname))
            with open(os.path.join(new_func, "mod.rs"), 'w') as f:
                f.write(make_mod_rs(b, func))
            n_new = new_crate.replace("kernel-mgga-", "")
            with open(os.path.join(new_dir, "Cargo.toml"), 'w') as f:
                f.write(CARGO_TOML_TEMPLATE.format(name=n_new))
            with open(os.path.join(new_dir, "src", "lib.rs"), 'w') as f:
                f.write(make_lib_rs(new_crate, func))

    update_workspace_and_aggregator([(p[0], p[3]) for p in plan])

    # Remove old crates AFTER updating references
    for crate, _f, _b, _new in plan:
        shutil.rmtree(os.path.join(CRATES_DIR, crate))

    print(f"\nDone. Created {sum(len(p[3]) for p in plan)} new sub-crates, "
          f"removed {len(plan)} old ones.")


if __name__ == '__main__':
    main()
