#!/usr/bin/env python3
"""
Batch translate all 92 MGGA maple2c C kernel files to Rust sub-crates.

Uses translate_mgga.py's translate_functional() for each functional.
Uses first-fit-decreasing bin packing by generated line count to keep
each sub-crate under ~50K lines, avoiding OOM during CubeCL proc macro
expansion.

Seven large functionals (mgga_c_rmggac, mgga_c_revtpss, mgga_c_tpssloc,
mgga_c_kcisk, mgga_c_kcis, mgga_x_br89, mgga_c_tpss) exceed 50K lines
individually and occupy solo sub-crates.

Usage:
    batch_translate_mgga.py [--target-max N] [--dry-run]
"""

import os
import sys

# Ensure translate_mgga.py is importable
sys.path.insert(0, os.path.join(os.path.dirname(__file__)))
from translate_mgga import translate_functional

MGGA_EXC_DIR = "libxc-master/src/maple2c/mgga_exc"
MGGA_VXC_DIR = "libxc-master/src/maple2c/mgga_vxc"
CRATES_DIR = "crates"
DEFAULT_TARGET_MAX = 500000


def collect_all_functionals():
    """Collect all MGGA functional (c_path, func_name, is_vxc) tuples, sorted."""
    all_funcs = []

    # Standard mgga_exc files
    for fname in sorted(os.listdir(MGGA_EXC_DIR)):
        if not fname.endswith('.c') or 'Zone' in fname or fname.startswith('Makefile'):
            continue
        func_name = fname[:-2]
        c_path = os.path.join(MGGA_EXC_DIR, fname)
        all_funcs.append((c_path, func_name, False))

    # Special mgga_vxc files (vxc-only mode)
    if os.path.isdir(MGGA_VXC_DIR):
        for fname in sorted(os.listdir(MGGA_VXC_DIR)):
            if not fname.endswith('.c') or 'Zone' in fname or fname.startswith('Makefile'):
                continue
            func_name = fname[:-2]
            c_path = os.path.join(MGGA_VXC_DIR, fname)
            all_funcs.append((c_path, func_name, True))

    # Sort all by func_name for deterministic assignment
    all_funcs.sort(key=lambda x: x[1])
    return all_funcs


def translate_and_measure(all_funcs, staging_dir):
    """Translate all functionals to a staging dir and measure line counts."""
    results = []  # (func_name, line_count, is_vxc)

    for c_path, func_name, is_vxc in all_funcs:
        suffix = " (vxc-only)" if is_vxc else ""
        print(f"  Translating {func_name}{suffix}...", end=" ", flush=True)

        try:
            written = translate_functional(c_path, func_name, staging_dir, is_vxc)

            # Count lines in generated files
            func_dir = os.path.join(staging_dir, func_name)
            total_lines = 0
            if os.path.isdir(func_dir):
                for rs_file in os.listdir(func_dir):
                    if rs_file.endswith('.rs'):
                        fpath = os.path.join(func_dir, rs_file)
                        total_lines += sum(1 for _ in open(fpath))
            else:
                # Single file module
                rs_path = os.path.join(staging_dir, func_name + '.rs')
                if os.path.exists(rs_path):
                    total_lines = sum(1 for _ in open(rs_path))

            results.append((func_name, total_lines, is_vxc))
            print(f"OK ({total_lines} lines, {len(written)} files)")
        except Exception as e:
            print(f"FAILED: {e}")
            results.append((func_name, 0, is_vxc))

    return results


def bin_pack_ffd(items, target_max):
    """First-fit-decreasing bin packing.

    items: list of (func_name, line_count, is_vxc)
    Returns: list of lists
    """
    sorted_items = sorted(items, key=lambda x: -x[1])
    batches = []
    batch_totals = []

    for item in sorted_items:
        placed = False
        for i, batch in enumerate(batches):
            if batch_totals[i] + item[1] <= target_max:
                batch.append(item)
                batch_totals[i] += item[1]
                placed = True
                break
        if not placed:
            batches.append([item])
            batch_totals.append(item[1])

    # Sort batches by first functional name for determinism
    batches.sort(key=lambda b: b[0][0])
    return batches


def create_subcrate_cargo_toml(crate_dir, crate_num):
    """Create Cargo.toml for a sub-crate."""
    cargo_path = os.path.join(crate_dir, "Cargo.toml")
    content = f"""[package]
name = "libxc-kernel-mgga-{crate_num}"
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
    with open(cargo_path, 'w') as f:
        f.write(content)
    return cargo_path


def create_subcrate_lib_rs(crate_dir, compiled_mods, deferred_mods, crate_num):
    """Create lib.rs for a sub-crate with module declarations."""
    lib_path = os.path.join(crate_dir, "src", "lib.rs")
    lines = [
        '#![allow(clippy::excessive_precision)]',
        '#![allow(clippy::needless_late_init)]',
        '#![allow(clippy::too_many_arguments)]',
        '',
        f'//! MGGA kernel translations batch {crate_num}.',
        '',
    ]

    for mod_name in sorted(compiled_mods):
        lines.append(f'pub mod {mod_name};')

    if deferred_mods:
        lines.append('')
        lines.append('// Deferred functionals -- source generated but too large to compile.')
        lines.append('// These contain #[cube(launch_unchecked)] functions exceeding 5K lines')
        lines.append('// which cause rustc OOM during CubeCL proc macro expansion.')
        for mod_name, reason in deferred_mods:
            lines.append(f'// pub mod {mod_name};  // deferred: {reason}')

    lines.append('')
    with open(lib_path, 'w') as f:
        f.write('\n'.join(lines))
    return lib_path


def main():
    import shutil

    target_max = DEFAULT_TARGET_MAX
    dry_run = False

    # Simple arg parsing
    args = sys.argv[1:]
    if '--dry-run' in args:
        dry_run = True
        args.remove('--dry-run')
    if '--target-max' in args:
        idx = args.index('--target-max')
        target_max = int(args[idx + 1])

    # Collect all functionals
    all_funcs = collect_all_functionals()
    print(f"Found {len(all_funcs)} MGGA functionals (target max {target_max} lines/crate)")

    # Translate to staging area
    staging = "/tmp/mgga_translate_staging"
    if os.path.exists(staging):
        shutil.rmtree(staging)
    os.makedirs(staging)

    print("\nPhase 1: Translating all functionals...")
    results = translate_and_measure(all_funcs, staging)

    # Filter out failures
    valid = [(name, lines, vxc) for name, lines, vxc in results if lines > 0]
    failed = [(name, lines, vxc) for name, lines, vxc in results if lines == 0]

    print(f"\nTranslated: {len(valid)}, Failed: {len(failed)}")
    if failed:
        for name, _, _ in failed:
            print(f"  FAILED: {name}")

    # Bin pack
    batches = bin_pack_ffd(valid, target_max)
    print(f"\nPhase 2: Bin packing into {len(batches)} sub-crates")

    for i, batch in enumerate(batches, 1):
        total = sum(lines for _, lines, _ in batch)
        names = ', '.join(name for name, _, _ in batch)
        print(f"  mgga-{i}: {total:6d} lines ({len(batch)} funcs): {names}")

    if dry_run:
        print("\nDry run -- no crates created.")
        shutil.rmtree(staging)
        return

    # Create sub-crates
    print(f"\nPhase 3: Creating {len(batches)} sub-crates...")
    for i, batch in enumerate(batches, 1):
        crate_name = f"kernel-mgga-{i}"
        crate_dir = os.path.join(CRATES_DIR, crate_name)
        src_dir = os.path.join(crate_dir, "src")
        os.makedirs(src_dir, exist_ok=True)

        create_subcrate_cargo_toml(crate_dir, i)

        compiled_mods = []
        for func_name, lines, is_vxc in batch:
            # Move from staging to crate src dir
            src_func = os.path.join(staging, func_name)
            dst_func = os.path.join(src_dir, func_name)
            if os.path.isdir(src_func):
                if os.path.exists(dst_func):
                    shutil.rmtree(dst_func)
                shutil.copytree(src_func, dst_func)
            else:
                src_file = os.path.join(staging, func_name + '.rs')
                if os.path.exists(src_file):
                    shutil.copy2(src_file, os.path.join(src_dir, func_name + '.rs'))
            compiled_mods.append(func_name)

        create_subcrate_lib_rs(crate_dir, compiled_mods, [], i)

        total = sum(lines for _, lines, _ in batch)
        print(f"  Created {crate_name}: {len(compiled_mods)} funcs, {total} lines")

    shutil.rmtree(staging)

    # Summary
    n = len(batches)
    print(f"\n{'='*60}")
    print(f"BATCH TRANSLATION COMPLETE")
    print(f"{'='*60}")
    print(f"Total functionals: {len(all_funcs)}")
    print(f"Translated: {len(valid)}")
    print(f"Failed: {len(failed)}")
    print(f"Sub-crates: {n}")

    # Print workspace integration hints
    print(f"\nWorkspace Cargo.toml members to add:")
    for i in range(1, n + 1):
        print(f'    "crates/kernel-mgga-{i}",')

    print(f"\nFacade dependencies:")
    for i in range(1, n + 1):
        print(f'libxc-kernel-mgga-{i} = {{ path = "../kernel-mgga-{i}" }}')


if __name__ == '__main__':
    main()
