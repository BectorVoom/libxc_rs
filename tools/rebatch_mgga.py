#!/usr/bin/env python3
"""
Rebatch MGGA sub-crates with configurable per-crate line ceiling.

Reads per-functional sizes from existing sub-crates, applies first-fit-decreasing
bin packing with a configurable line target, moves functional directories into new
sub-crate layout, and generates Cargo.toml + lib.rs for each.

Usage:
    python3 tools/rebatch_mgga.py [--dry-run] [--target-max N]

--target-max N (default 500000):
    Maximum lines per sub-crate. Smaller values produce more, smaller crates;
    larger values produce fewer, larger crates.

    Default raised from 50K (Phase 8 P08 OOM mitigation) to 500K once the
    project has memory headroom for the heavier per-crate CubeCL proc-macro
    expansion. The build-time saving comes from reducing cargo coordination
    overhead — fewer crates means fewer manifest parses, fewer build-script
    runs, fewer link units, less dependency-graph bookkeeping.

    Build-time vs RAM trade-off:
      - 50K (legacy) → ~37 crates, ~6-12 GB RAM per crate at peak. Safe on
                        24 GB systems with jobs=3 cap. Forces ~37× cargo
                        coordination cost per workspace build.
      - 500K (default) → ~4-6 crates, ~30-50 GB RAM per crate at peak.
                        Suitable for 64+ GB systems. Cargo coordination drops
                        ~7-9×, dominant gain on multi-core builds.
      - Larger values → keep going if RAM allows. Diminishing returns past
                        the point where cargo's coordination cost is below
                        the longest single-crate compile time.

    For memory-tight systems, opt back into the legacy ceiling explicitly:
        python3 tools/rebatch_mgga.py --target-max 50000

    Recommendation: always pass `--dry-run` first to inspect the resulting
    bin count and the single-largest-bin size, then benchmark with
    `/usr/bin/time -v cargo check -p libxc-kernel-mgga-1` to confirm peak
    RSS before committing the new layout.
"""

import os
import sys
import shutil
import json

CRATES_DIR = "crates"
DEFAULT_TARGET_MAX = 500_000  # Memory-permissive default; opt back to 50000 for tight RAM


def get_functional_sizes():
    """Scan existing sub-crates and measure per-functional line counts."""
    funcs = []  # list of (func_name, line_count, source_crate_dir)

    for entry in sorted(os.listdir(CRATES_DIR)):
        if not entry.startswith("kernel-mgga-") or entry == "kernel-mgga":
            continue
        crate_dir = os.path.join(CRATES_DIR, entry)
        src_dir = os.path.join(crate_dir, "src")
        if not os.path.isdir(src_dir):
            continue

        for item in sorted(os.listdir(src_dir)):
            item_path = os.path.join(src_dir, item)
            if item == "lib.rs" or not os.path.isdir(item_path):
                # Check if it's a .rs file (non-directory module)
                if item.endswith('.rs') and item != 'lib.rs':
                    func_name = item[:-3]
                    lines = sum(1 for _ in open(item_path))
                    funcs.append((func_name, lines, crate_dir, False))
                continue

            # Directory-based module
            func_name = item
            total_lines = 0
            for rs_file in os.listdir(item_path):
                if rs_file.endswith('.rs'):
                    fpath = os.path.join(item_path, rs_file)
                    total_lines += sum(1 for _ in open(fpath))
            funcs.append((func_name, total_lines, crate_dir, True))

    return funcs


def bin_pack(funcs, target_max):
    """First-fit decreasing bin packing."""
    # Sort by size descending
    sorted_funcs = sorted(funcs, key=lambda x: -x[1])

    batches = []  # list of lists of (func_name, lines, source_dir, is_dir)
    batch_totals = []

    for func in sorted_funcs:
        placed = False
        for i, batch in enumerate(batches):
            if batch_totals[i] + func[1] <= target_max:
                batch.append(func)
                batch_totals[i] += func[1]
                placed = True
                break
        if not placed:
            batches.append([func])
            batch_totals.append(func[1])

    # Sort batches by first functional name for determinism
    batches.sort(key=lambda b: b[0][0])
    return batches


def create_cargo_toml(crate_dir, crate_num):
    """Create Cargo.toml for a sub-crate."""
    content = f"""[package]
name = "libxc-kernel-mgga-{crate_num}"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = {{ version = "0.9.0", default-features = false, features = ["cpu"] }}
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
    with open(os.path.join(crate_dir, "Cargo.toml"), 'w') as f:
        f.write(content)


def create_lib_rs(crate_dir, func_names, crate_num):
    """Create lib.rs for a sub-crate."""
    lines = [
        '#![allow(clippy::excessive_precision)]',
        '#![allow(clippy::needless_late_init)]',
        '#![allow(clippy::too_many_arguments)]',
        '',
        f'//! MGGA kernel translations batch {crate_num}.',
        '',
    ]
    for name in sorted(func_names):
        lines.append(f'pub mod {name};')
    lines.append('')

    with open(os.path.join(crate_dir, "src", "lib.rs"), 'w') as f:
        f.write('\n'.join(lines))


def main():
    dry_run = '--dry-run' in sys.argv

    target_max = DEFAULT_TARGET_MAX
    if '--target-max' in sys.argv:
        idx = sys.argv.index('--target-max')
        if idx + 1 >= len(sys.argv):
            print("--target-max requires a positive integer (lines per crate)")
            sys.exit(1)
        try:
            target_max = int(sys.argv[idx + 1])
        except ValueError:
            print(f"--target-max must be an integer, got {sys.argv[idx + 1]!r}")
            sys.exit(1)
        if target_max <= 0:
            print(f"--target-max must be positive, got {target_max}")
            sys.exit(1)

    # Step 1: Measure all functionals
    print("Scanning existing sub-crates...")
    funcs = get_functional_sizes()
    print(f"Found {len(funcs)} functionals")

    # Step 2: Bin pack
    batches = bin_pack(funcs, target_max)
    print(f"Planned {len(batches)} sub-crates (target max {target_max} lines)")

    for i, batch in enumerate(batches, 1):
        total = sum(f[1] for f in batch)
        names = ', '.join(f[0] for f in batch)
        print(f"  mgga-{i}: {total:6d} lines ({len(batch)} funcs): {names}")

    if dry_run:
        print("\nDry run -- no changes made.")
        return

    # Step 3: Create staging area
    staging = "/tmp/mgga_rebatch_staging"
    if os.path.exists(staging):
        shutil.rmtree(staging)
    os.makedirs(staging)

    # Copy all functional dirs/files to staging
    print("\nCopying functionals to staging area...")
    for func_name, lines, source_crate, is_dir in funcs:
        if is_dir:
            src = os.path.join(source_crate, "src", func_name)
            dst = os.path.join(staging, func_name)
            shutil.copytree(src, dst)
        else:
            src = os.path.join(source_crate, "src", func_name + ".rs")
            dst = os.path.join(staging, func_name + ".rs")
            shutil.copy2(src, dst)

    # Step 4: Remove old sub-crates
    print("Removing old sub-crates...")
    for entry in os.listdir(CRATES_DIR):
        if entry.startswith("kernel-mgga-") and entry != "kernel-mgga":
            path = os.path.join(CRATES_DIR, entry)
            shutil.rmtree(path)
            print(f"  Removed {entry}")

    # Step 5: Create new sub-crates
    print(f"\nCreating {len(batches)} new sub-crates...")
    for i, batch in enumerate(batches, 1):
        crate_name = f"kernel-mgga-{i}"
        crate_dir = os.path.join(CRATES_DIR, crate_name)
        src_dir = os.path.join(crate_dir, "src")
        os.makedirs(src_dir, exist_ok=True)

        # Create Cargo.toml
        create_cargo_toml(crate_dir, i)

        # Move functional files from staging
        func_names = []
        for func_name, lines, _, is_dir in batch:
            if is_dir:
                src = os.path.join(staging, func_name)
                dst = os.path.join(src_dir, func_name)
                shutil.copytree(src, dst)
            else:
                src = os.path.join(staging, func_name + ".rs")
                dst = os.path.join(src_dir, func_name + ".rs")
                shutil.copy2(src, dst)
            func_names.append(func_name)

        # Create lib.rs
        create_lib_rs(crate_dir, func_names, i)

        total = sum(f[1] for f in batch)
        print(f"  Created {crate_name}: {len(func_names)} funcs, {total} lines")

    # Cleanup staging
    shutil.rmtree(staging)

    # Step 6: Print what needs updating
    n = len(batches)
    print(f"\n{'='*60}")
    print(f"REBATCHING COMPLETE: {n} sub-crates created")
    print(f"{'='*60}")

    print(f"\nUpdate kernel-mgga/Cargo.toml dependencies:")
    for i in range(1, n + 1):
        print(f'libxc-kernel-mgga-{i} = {{ path = "../kernel-mgga-{i}" }}')

    print(f"\nUpdate kernel-mgga/src/lib.rs re-exports:")
    for i in range(1, n + 1):
        print(f'pub use libxc_kernel_mgga_{i} as batch{i};')

    print(f"\nWorkspace members to add:")
    for i in range(1, n + 1):
        print(f'    "crates/kernel-mgga-{i}",')

    # Output batch count for scripting
    with open("/tmp/mgga_batch_count.txt", "w") as f:
        f.write(str(n))


if __name__ == '__main__':
    main()
