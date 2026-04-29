#!/usr/bin/env python3
"""Re-split all 131 GGA functionals into sub-crates using first-fit-decreasing bin packing.

Follows the proven MGGA sub-crate pattern. The 25 deferred functionals
(whose lxc_pol/kxc_pol files exceed the CubeCL proc macro memory limit)
are included as source but commented out in lib.rs.

Usage: python3 tools/resplit_gga.py [--bin-limit N]

--bin-limit N (default 50000):
    Maximum lines per sub-crate for bin packing. Smaller values produce more,
    smaller crates; larger values produce fewer, larger crates.

    Build-time vs RAM trade-off:
      - Small limit  → many small crates → less RAM per crate (chosen at 50K
                       to dodge >20 GB RSS during CubeCL proc-macro expansion),
                       but more cargo coordination overhead per workspace build.
      - Large limit  → fewer larger crates → less cargo overhead, but each
                       crate uses more RAM during expansion.

    Recommendation: only raise this after benchmarking single-crate compile RAM
    with the new size. Phase 9 plans to instrument this trade-off systematically.
"""

import os
import re
import shutil
import sys

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GGA_SRC = os.path.join(PROJECT_ROOT, "crates", "kernel-gga", "src")
CRATES_DIR = os.path.join(PROJECT_ROOT, "crates")
CARGO_TOML = os.path.join(PROJECT_ROOT, "Cargo.toml")
DEFAULT_BIN_LIMIT = 50_000  # Phase 8 P08 OOM mitigation

# 25 deferred functionals: CubeCL proc macro SIGSEGV on their large
# lxc_pol/kxc_pol files. Source is distributed but commented out.
DEFERRED = {
    "gga_c_acgga", "gga_c_acggap", "gga_c_ft97", "gga_c_gapc",
    "gga_c_gaploc", "gga_c_hcth_a", "gga_c_optc", "gga_c_pbe_erf_gws",
    "gga_c_pbeloc", "gga_c_pw91", "gga_c_q2d", "gga_c_regtpss",
    "gga_c_revtca", "gga_c_sg4", "gga_c_sogga11", "gga_c_zpbeint",
    "gga_c_zvpbeint", "gga_c_zvpbeloc", "gga_x_ft97", "gga_x_hjs",
    "gga_x_hjs_b88_v2", "gga_x_lcgau", "gga_x_wpbeh", "gga_xc_b97",
    "hyb_gga_xc_wb97",
}


def get_functional_line_counts():
    """Walk kernel-gga/src/ and count lines per functional directory."""
    functionals = {}
    for name in sorted(os.listdir(GGA_SRC)):
        path = os.path.join(GGA_SRC, name)
        if not os.path.isdir(path):
            continue
        total = 0
        for fname in os.listdir(path):
            if fname.endswith(".rs"):
                fpath = os.path.join(path, fname)
                with open(fpath, "r") as f:
                    total += sum(1 for _ in f)
        functionals[name] = total
    return functionals


def bin_pack_ffd(functionals, limit):
    """First-fit-decreasing bin packing. Returns list of lists of (name, lines)."""
    items = sorted(functionals.items(), key=lambda x: x[1], reverse=True)
    bins = []       # list of lists of (name, lines)
    bin_sizes = []  # current total lines per bin

    for name, lines in items:
        placed = False
        # Solo crate for items that exceed the bin limit
        if lines > limit:
            bins.append([(name, lines)])
            bin_sizes.append(lines)
            placed = True
        else:
            for i, sz in enumerate(bin_sizes):
                if sz + lines <= limit:
                    bins[i].append((name, lines))
                    bin_sizes[i] += lines
                    placed = True
                    break
        if not placed:
            bins.append([(name, lines)])
            bin_sizes.append(lines)

    return bins, bin_sizes


def delete_old_subcrates():
    """Remove old kernel-gga-N directories."""
    removed = []
    for name in os.listdir(CRATES_DIR):
        if re.match(r"^kernel-gga-\d+$", name):
            path = os.path.join(CRATES_DIR, name)
            shutil.rmtree(path)
            removed.append(name)
    if removed:
        print(f"Removed {len(removed)} old sub-crates: {', '.join(sorted(removed, key=lambda x: int(x.split('-')[-1])))}")


def create_subcrate(bin_index, functionals_in_bin):
    """Create a single kernel-gga-N sub-crate."""
    n = bin_index + 1
    crate_name = f"kernel-gga-{n}"
    crate_dir = os.path.join(CRATES_DIR, crate_name)
    src_dir = os.path.join(crate_dir, "src")
    os.makedirs(src_dir, exist_ok=True)

    # Copy functional directories
    for name, _ in functionals_in_bin:
        src_path = os.path.join(GGA_SRC, name)
        dst_path = os.path.join(src_dir, name)
        if os.path.exists(dst_path):
            shutil.rmtree(dst_path)
        shutil.copytree(src_path, dst_path)

    # Generate Cargo.toml (no feature gates)
    cargo_content = f"""[package]
name = "libxc-kernel-gga-{n}"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = {{ version = "0.9.0", default-features = false, features = ["cpu"] }}
libxc-kernel-math = {{ path = "../kernel-math" }}
"""
    with open(os.path.join(crate_dir, "Cargo.toml"), "w") as f:
        f.write(cargo_content)

    # Generate lib.rs (deferred functionals are commented out)
    func_names = sorted([name for name, _ in functionals_in_bin])
    compiled = [n for n in func_names if n not in DEFERRED]
    deferred = [n for n in func_names if n in DEFERRED]

    lib_lines = [
        '#![allow(clippy::excessive_precision)]',
        '#![allow(clippy::needless_late_init)]',
        '#![allow(clippy::too_many_arguments)]',
        '',
        f'//! GGA kernel translations batch {n}.',
        '',
    ]
    for name in compiled:
        lib_lines.append(f'pub mod {name};')
    if deferred:
        lib_lines.append('')
        lib_lines.append('// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.')
        for name in deferred:
            lib_lines.append(f'// pub mod {name};')
    lib_lines.append('')

    with open(os.path.join(src_dir, "lib.rs"), "w") as f:
        f.write('\n'.join(lib_lines))

    return n


def update_facade(num_subcrates):
    """Rewrite kernel-gga/Cargo.toml and kernel-gga/src/lib.rs."""
    # Cargo.toml
    deps = [
        'cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }',
        'libxc-kernel-math = { path = "../kernel-math" }',
    ]
    for i in range(1, num_subcrates + 1):
        deps.append(f'libxc-kernel-gga-{i} = {{ path = "../kernel-gga-{i}" }}')

    cargo_content = f"""[package]
name = "libxc-kernel-gga"
version = "0.1.0"
edition = "2024"

[dependencies]
{chr(10).join(deps)}
"""
    facade_toml = os.path.join(CRATES_DIR, "kernel-gga", "Cargo.toml")
    with open(facade_toml, "w") as f:
        f.write(cargo_content)

    # lib.rs
    lib_lines = [
        '#![allow(clippy::excessive_precision)]',
        '#![allow(clippy::needless_late_init)]',
        '#![allow(clippy::too_many_arguments)]',
        '',
        '//! GGA kernel translations from maple2c.',
        '//!',
        f'//! 131 GGA functionals total across {num_subcrates} sub-crates.',
        '//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under',
        '//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.',
        '',
        '// Re-export sub-crates containing compiled GGA functionals.',
    ]
    for i in range(1, num_subcrates + 1):
        lib_lines.append(f'pub use libxc_kernel_gga_{i} as batch{i};')
    lib_lines.append('')

    facade_lib = os.path.join(CRATES_DIR, "kernel-gga", "src", "lib.rs")
    with open(facade_lib, "w") as f:
        f.write('\n'.join(lib_lines))


def update_workspace(num_subcrates):
    """Update workspace members in root Cargo.toml."""
    with open(CARGO_TOML, "r") as f:
        content = f.read()

    # Find the members array and replace old gga sub-crate entries
    # Remove old kernel-gga-N entries
    lines = content.split('\n')
    new_lines = []
    in_members = False
    gga_entries_inserted = False

    for line in lines:
        # Check if we're in the members array
        if 'members = [' in line:
            in_members = True
            new_lines.append(line)
            continue

        if in_members:
            # Skip old kernel-gga-N entries (but keep kernel-gga itself)
            stripped = line.strip().strip(',').strip('"').strip("'")
            if re.match(r'^crates/kernel-gga-\d+$', stripped):
                # Insert new entries right before the first old one (once)
                if not gga_entries_inserted:
                    for i in range(1, num_subcrates + 1):
                        new_lines.append(f'    "crates/kernel-gga-{i}",')
                    gga_entries_inserted = True
                continue

            # Detect end of members array
            if ']' in line:
                # If we never saw old entries, insert before closing bracket
                if not gga_entries_inserted:
                    for i in range(1, num_subcrates + 1):
                        new_lines.append(f'    "crates/kernel-gga-{i}",')
                    gga_entries_inserted = True
                in_members = False

        new_lines.append(line)

    with open(CARGO_TOML, "w") as f:
        f.write('\n'.join(new_lines))


def main():
    bin_limit = DEFAULT_BIN_LIMIT
    if '--bin-limit' in sys.argv:
        idx = sys.argv.index('--bin-limit')
        if idx + 1 >= len(sys.argv):
            print("--bin-limit requires a positive integer (lines per crate)")
            sys.exit(1)
        try:
            bin_limit = int(sys.argv[idx + 1])
        except ValueError:
            print(f"--bin-limit must be an integer, got {sys.argv[idx + 1]!r}")
            sys.exit(1)
        if bin_limit <= 0:
            print(f"--bin-limit must be positive, got {bin_limit}")
            sys.exit(1)

    print("=== GGA Sub-Crate Re-Split ===\n")

    # Step 1: Inventory
    functionals = get_functional_line_counts()
    total_lines = sum(functionals.values())
    print(f"Found {len(functionals)} GGA functionals, {total_lines:,} total lines\n")

    # Step 2: Bin pack
    bins, bin_sizes = bin_pack_ffd(functionals, bin_limit)
    print(f"Bin packing with limit={bin_limit:,} lines -> {len(bins)} bins\n")

    for i, (b, sz) in enumerate(zip(bins, bin_sizes)):
        names = [name for name, _ in b]
        print(f"  Bin {i+1:2d}: {sz:6,} lines, {len(b):2d} functionals")

    print()

    # Step 3: Delete old sub-crates
    delete_old_subcrates()

    # Step 4: Create new sub-crates
    for i, b in enumerate(bins):
        n = create_subcrate(i, b)
    print(f"Created {len(bins)} new sub-crates")

    # Step 5: Update facade
    update_facade(len(bins))
    print("Updated kernel-gga facade (Cargo.toml + lib.rs)")

    # Step 6: Update workspace
    update_workspace(len(bins))
    print("Updated workspace members in root Cargo.toml")

    # Summary
    print(f"\n=== Summary ===")
    print(f"Functionals: {len(functionals)}")
    print(f"Sub-crates:  {len(bins)}")
    print(f"Total lines: {total_lines:,}")
    print(f"Max bin:     {max(bin_sizes):,} lines")
    print(f"Min bin:     {min(bin_sizes):,} lines")
    solo = sum(1 for sz in bin_sizes if sz > bin_limit)
    if solo:
        print(f"Solo crates: {solo} (exceed {bin_limit:,} line limit)")


if __name__ == "__main__":
    main()
