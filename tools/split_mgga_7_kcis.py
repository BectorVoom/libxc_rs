#!/usr/bin/env python3
"""Split kernel-mgga-7 (mgga_c_kcis, ~240K lines) into multiple sub-crates.

Per-functional bin-packing (rebatch_mgga.py) cannot subdivide a single
functional. KCIS is 240K lines — far above the 50K-line target — and OOMs
rustc when compiled as one crate. This script does per-FILE bin-packing
within KCIS, producing kernel-mgga-7a..7e each ≤50K lines.

Each new crate keeps the same module path `mgga_c_kcis::<file>` so call
sites are unaffected; the only public-API impact is at the aggregator
(`crates/kernel-mgga/src/lib.rs`) which gains additional `batch7a..7e`
re-exports replacing the old `batch7`.

Usage: python3 tools/split_mgga_7_kcis.py [--dry-run]
"""

import os
import shutil
import sys
import re

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC_CRATE = "kernel-mgga-7"
FUNCTIONAL = "mgga_c_kcis"
TARGET_MAX = 500000
SUFFIXES = ['a', 'b', 'c', 'd', 'e', 'f', 'g']  # extend if more bins needed


def collect_files():
    """Return sorted list of (filename, line_count)."""
    src = os.path.join(REPO_ROOT, "crates", SRC_CRATE, "src", FUNCTIONAL)
    out = []
    for f in sorted(os.listdir(src)):
        if f.endswith(".rs") and f != "mod.rs":
            n = sum(1 for _ in open(os.path.join(src, f)))
            out.append((f, n))
    return out


def bin_pack(files, target):
    """First-fit-decreasing on (name, lines) tuples."""
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


def make_lib_rs(bin_idx_letter):
    return (
        '#![allow(clippy::excessive_precision)]\n'
        '#![allow(clippy::needless_late_init)]\n'
        '#![allow(clippy::too_many_arguments)]\n'
        '\n'
        f'//! MGGA kernel translations batch 7{bin_idx_letter} (mgga_c_kcis subset).\n'
        '\n'
        f'pub mod {FUNCTIONAL};\n'
    )


def make_mod_rs(file_subset):
    lines = [f'//! MGGA_C_KCIS kernel — split into per-function files.\n', '\n']
    for fname, _ in sorted(file_subset, key=lambda x: x[0]):
        modname = fname[:-3]  # strip .rs
        lines.append(f'pub mod {modname};\n')
    return ''.join(lines)


def main():
    dry_run = '--dry-run' in sys.argv

    files = collect_files()
    print(f"Found {len(files)} files in {SRC_CRATE}/{FUNCTIONAL}, "
          f"total {sum(n for _, n in files)} lines")

    bins = bin_pack(files, TARGET_MAX)
    print(f"\nBin-packed into {len(bins)} sub-crates (target {TARGET_MAX} lines):")
    for i, b in enumerate(bins):
        total = sum(x[1] for x in b)
        suffix = SUFFIXES[i]
        print(f"  kernel-mgga-7{suffix}: {total:6d} lines, {len(b)} files")

    if dry_run:
        print("\nDry run -- no changes made.")
        return

    if len(bins) > len(SUFFIXES):
        print(f"ERROR: need more suffixes (have {len(SUFFIXES)}, need {len(bins)})")
        sys.exit(1)

    src_dir = os.path.join(REPO_ROOT, "crates", SRC_CRATE, "src", FUNCTIONAL)

    # Create new sub-crates
    new_crate_names = []
    for i, b in enumerate(bins):
        suffix = SUFFIXES[i]
        crate_name = f"kernel-mgga-7{suffix}"
        crate_dir = os.path.join(REPO_ROOT, "crates", crate_name)
        new_src = os.path.join(crate_dir, "src", FUNCTIONAL)
        os.makedirs(new_src, exist_ok=True)
        new_crate_names.append((suffix, crate_name))

        # Copy files
        for fname, _n in b:
            shutil.copy2(os.path.join(src_dir, fname), os.path.join(new_src, fname))

        # mod.rs for this functional
        with open(os.path.join(new_src, "mod.rs"), 'w') as f:
            f.write(make_mod_rs(b))

        # Cargo.toml
        with open(os.path.join(crate_dir, "Cargo.toml"), 'w') as f:
            f.write(CARGO_TOML_TEMPLATE.format(name=f"7{suffix}"))

        # lib.rs
        with open(os.path.join(crate_dir, "src", "lib.rs"), 'w') as f:
            f.write(make_lib_rs(suffix))

        print(f"  Created {crate_name}: {len(b)} files")

    # Update workspace Cargo.toml
    ws_cargo = os.path.join(REPO_ROOT, "Cargo.toml")
    with open(ws_cargo) as f:
        content = f.read()

    # Replace the dependency line
    new_dep_lines = '\n'.join(
        f'libxc-kernel-mgga-7{suf} = {{ path = "crates/{cname}" }}'
        for suf, cname in new_crate_names
    )
    content = content.replace(
        'libxc-kernel-mgga-7 = { path = "crates/kernel-mgga-7" }',
        new_dep_lines
    )

    # Replace workspace members entry
    new_member_lines = '\n'.join(
        f'    "crates/{cname}",' for _suf, cname in new_crate_names
    )
    content = content.replace(
        '    "crates/kernel-mgga-7",',
        new_member_lines
    )

    with open(ws_cargo, 'w') as f:
        f.write(content)
    print(f"  Updated {ws_cargo}")

    # Update crates/kernel-mgga/Cargo.toml
    mg_cargo = os.path.join(REPO_ROOT, "crates", "kernel-mgga", "Cargo.toml")
    with open(mg_cargo) as f:
        content = f.read()
    new_dep_lines = '\n'.join(
        f'libxc-kernel-mgga-7{suf} = {{ path = "../{cname}" }}'
        for suf, cname in new_crate_names
    )
    content = content.replace(
        'libxc-kernel-mgga-7 = { path = "../kernel-mgga-7" }',
        new_dep_lines
    )
    with open(mg_cargo, 'w') as f:
        f.write(content)
    print(f"  Updated {mg_cargo}")

    # Update crates/kernel-mgga/src/lib.rs
    mg_lib = os.path.join(REPO_ROOT, "crates", "kernel-mgga", "src", "lib.rs")
    with open(mg_lib) as f:
        content = f.read()
    new_export_lines = '\n'.join(
        f'pub use libxc_kernel_mgga_7{suf} as batch7{suf};'
        for suf, _cname in new_crate_names
    )
    content = content.replace(
        'pub use libxc_kernel_mgga_7 as batch7;',
        new_export_lines
    )
    with open(mg_lib, 'w') as f:
        f.write(content)
    print(f"  Updated {mg_lib}")

    # Remove old kernel-mgga-7
    old = os.path.join(REPO_ROOT, "crates", SRC_CRATE)
    shutil.rmtree(old)
    print(f"  Removed {old}")

    print(f"\nDone. {len(new_crate_names)} new sub-crates created.")


if __name__ == '__main__':
    main()
