#!/usr/bin/env python3
"""
Batch translate all 92 MGGA maple2c C kernel files to Rust sub-crates.

Uses translate_mgga.py's translate_functional() for each functional.
Splits functionals into sub-crates of ~15 each to avoid OOM during
CubeCL proc macro expansion.

Sub-crate 1 already exists with 4 functionals (mgga_xc_lp90, mgga_k_gea2,
mgga_x_lta, mgga_c_b88) from Plans 01-02. This script regenerates all
sub-crates from scratch for a clean, deterministic layout.

Usage:
    batch_translate_mgga.py [--batch-size N] [--dry-run]
"""

import os
import sys
import glob

# Ensure translate_mgga.py is importable
sys.path.insert(0, os.path.join(os.path.dirname(__file__)))
from translate_mgga import translate_functional

MGGA_EXC_DIR = "libxc-master/src/maple2c/mgga_exc"
MGGA_VXC_DIR = "libxc-master/src/maple2c/mgga_vxc"
CRATES_DIR = "crates"
DEFAULT_BATCH_SIZE = 15


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


def split_into_batches(items, batch_size):
    """Split list into batches of given size."""
    return [items[i:i + batch_size] for i in range(0, len(items), batch_size)]


def create_subcrate_cargo_toml(crate_dir, crate_num):
    """Create Cargo.toml for a sub-crate."""
    cargo_path = os.path.join(crate_dir, "Cargo.toml")
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

[profile.test]
debug = 0
codegen-units = 16
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

    for mod_name in compiled_mods:
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
    batch_size = DEFAULT_BATCH_SIZE
    dry_run = False

    # Simple arg parsing
    args = sys.argv[1:]
    if '--dry-run' in args:
        dry_run = True
        args.remove('--dry-run')
    if '--batch-size' in args:
        idx = args.index('--batch-size')
        batch_size = int(args[idx + 1])

    # Collect all functionals
    all_funcs = collect_all_functionals()
    print(f"Found {len(all_funcs)} MGGA functionals ({batch_size} per sub-crate)")

    # Split into batches
    batches = split_into_batches(all_funcs, batch_size)
    print(f"Will create {len(batches)} sub-crates")

    if dry_run:
        for i, batch in enumerate(batches, 1):
            print(f"\n  kernel-mgga-{i}: {len(batch)} functionals")
            for _, name, is_vxc in batch:
                suffix = " (vxc-only)" if is_vxc else ""
                print(f"    {name}{suffix}")
        return

    total_compiled = 0
    total_deferred = 0
    all_deferred = []

    for batch_idx, batch in enumerate(batches, 1):
        crate_name = f"kernel-mgga-{batch_idx}"
        crate_dir = os.path.join(CRATES_DIR, crate_name)
        src_dir = os.path.join(crate_dir, "src")

        print(f"\n{'='*60}")
        print(f"Sub-crate {batch_idx}/{len(batches)}: {crate_name} ({len(batch)} functionals)")
        print(f"{'='*60}")

        # Create directory structure
        os.makedirs(src_dir, exist_ok=True)

        # Create Cargo.toml
        create_subcrate_cargo_toml(crate_dir, batch_idx)

        compiled_mods = []
        deferred_mods = []

        for c_path, func_name, is_vxc in batch:
            suffix = " (vxc-only)" if is_vxc else ""
            print(f"  Translating {func_name}{suffix}...", end=" ", flush=True)

            try:
                written = translate_functional(c_path, func_name, src_dir, is_vxc)
                compiled_mods.append(func_name)
                print(f"OK ({len(written)} files)")
                total_compiled += 1
            except Exception as e:
                reason = str(e)
                deferred_mods.append((func_name, reason))
                all_deferred.append((func_name, reason, batch_idx))
                print(f"DEFERRED: {reason}")
                total_deferred += 1

        # Create lib.rs
        create_subcrate_lib_rs(crate_dir, compiled_mods, deferred_mods, batch_idx)
        print(f"  -> {len(compiled_mods)} compiled, {len(deferred_mods)} deferred")

    # Summary
    print(f"\n{'='*60}")
    print(f"BATCH TRANSLATION COMPLETE")
    print(f"{'='*60}")
    print(f"Total functionals: {len(all_funcs)}")
    print(f"Compiled: {total_compiled}")
    print(f"Deferred: {total_deferred}")
    print(f"Sub-crates: {len(batches)}")

    if all_deferred:
        print(f"\nDeferred functionals:")
        for name, reason, batch_num in all_deferred:
            print(f"  kernel-mgga-{batch_num}/{name}: {reason}")

    # Print workspace integration hints
    print(f"\nWorkspace Cargo.toml members to add:")
    for i in range(1, len(batches) + 1):
        print(f'    "crates/kernel-mgga-{i}",')

    print(f"\nProfile overrides to add:")
    for i in range(1, len(batches) + 1):
        print(f'[profile.dev.package.libxc-kernel-mgga-{i}]')
        print(f'debug = 0')
        print(f'codegen-units = 16')
        print()
        print(f'[profile.test.package.libxc-kernel-mgga-{i}]')
        print(f'debug = 0')
        print(f'codegen-units = 16')
        print()


if __name__ == '__main__':
    main()
