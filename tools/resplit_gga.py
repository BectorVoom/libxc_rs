#!/usr/bin/env python3
"""Re-split all 131 GGA functionals into sub-crates using first-fit-decreasing bin packing.

Follows the proven MGGA sub-crate pattern. The 25 deferred functionals
(whose lxc_pol/kxc_pol files exceed the CubeCL proc macro memory limit)
are included as source but commented out in lib.rs.

Usage: python3 tools/resplit_gga.py [--bin-limit N]

--bin-limit N (default 500000):
    Maximum lines per sub-crate for bin packing. Smaller values produce more,
    smaller crates; larger values produce fewer, larger crates.

    Default raised from 50K (Phase 8 P08 OOM mitigation) to 500K once the
    project has memory headroom for heavier per-crate CubeCL proc-macro
    expansion. The build-time saving comes from reducing cargo coordination
    overhead — fewer crates means fewer manifest parses, fewer build-script
    runs, fewer link units.

    Build-time vs RAM trade-off:
      - 50K (legacy) → ~22 crates, ~6-12 GB RAM per crate at peak. Safe on
                        24 GB systems with jobs=3 cap. Forces ~22× cargo
                        coordination cost per workspace build.
      - 500K (default) → ~3-5 crates, ~30-50 GB RAM per crate at peak.
                        Suitable for 64+ GB systems. Cargo coordination drops
                        ~5-7×, dominant gain on multi-core builds.

    For memory-tight systems, opt back into the legacy ceiling explicitly:
        python3 tools/resplit_gga.py --bin-limit 50000

    Recommendation: always run with current --bin-limit value once before
    re-splitting (the script does delete + recreate sub-crates), and benchmark
    with `/usr/bin/time -v cargo check -p libxc-kernel-gga-1` to confirm peak
    RSS before committing the new layout.
"""

import os
import re
import shutil
import sys

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES_DIR = os.path.join(PROJECT_ROOT, "crates", "kernels")
GGA_SRC = os.path.join(CRATES_DIR, "gga", "src")
CARGO_TOML = os.path.join(PROJECT_ROOT, "Cargo.toml")
DEFAULT_BIN_LIMIT = 500_000  # Memory-permissive default; opt back to 50000 for tight RAM

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


SUBCRATE_RE = re.compile(r"^gga-\d+[a-p]?$")


def gather_into_aggregator(dry_run: bool) -> tuple[int, set[str]]:
    """Merge per-functional contents from every kernel-gga-* sub-crate's src/
    into kernel-gga/src/<func>/ on a per-file basis.

    `split_oversized_kernel.py` produces several `kernel-gga-N{a,b,c,d}/src/<func>/`
    directories that share the SAME functional name but contain DIFFERENT .rs
    files (each lettered sub-crate holds its share of the .rs files for that
    functional, with a per-share mod.rs declaring only that subset).

    To gather correctly we must:
      1. For each (sub-crate, functional) pair, copy every .rs file EXCEPT
         mod.rs into kernel-gga/src/<func>/ (skip-clobber: if a same-named
         .rs file already exists at the destination, abort with an error —
         indicates the sub-crate split logic is non-disjoint and we need
         human review rather than silent loss).
      2. After all .rs files are merged, regenerate each <func>/mod.rs from
         the union of file basenames in that <func>/ directory.

    Returns (num_files_moved, set_of_touched_funcs).
    """
    moved_files = 0
    touched: set[str] = set()
    for entry in sorted(os.listdir(CRATES_DIR)):
        if not SUBCRATE_RE.match(entry):
            continue
        sub_src = os.path.join(CRATES_DIR, entry, "src")
        if not os.path.isdir(sub_src):
            continue
        for item in sorted(os.listdir(sub_src)):
            item_path = os.path.join(sub_src, item)
            if not os.path.isdir(item_path):
                continue
            dst_dir = os.path.join(GGA_SRC, item)
            if not dry_run:
                os.makedirs(dst_dir, exist_ok=True)
            for fn in sorted(os.listdir(item_path)):
                if not fn.endswith(".rs"):
                    continue
                if fn == "mod.rs":
                    continue  # regenerated below
                src_file = os.path.join(item_path, fn)
                dst_file = os.path.join(dst_dir, fn)
                if os.path.exists(dst_file):
                    print(
                        f"FATAL: {dst_file} already exists when merging "
                        f"{src_file} — non-disjoint sub-crate split, refusing "
                        f"to clobber. Investigate manually.",
                        file=sys.stderr,
                    )
                    sys.exit(2)
                if dry_run:
                    print(f"  [dry-run] mv {src_file} -> {dst_file}")
                else:
                    shutil.move(src_file, dst_file)
                moved_files += 1
            touched.add(item)

    # Regenerate mod.rs for every touched functional from the merged file set.
    for func in sorted(touched):
        func_dir = os.path.join(GGA_SRC, func)
        if dry_run and not os.path.isdir(func_dir):
            continue  # dry-run did no actual moves; mod.rs synthesis skipped
        if not os.path.isdir(func_dir):
            continue
        modnames = sorted(
            fn[:-3] for fn in os.listdir(func_dir)
            if fn.endswith(".rs") and fn != "mod.rs"
        )
        mod_lines = [f"pub mod {n};" for n in modnames] + [""]
        mod_path = os.path.join(func_dir, "mod.rs")
        if dry_run:
            print(f"  [dry-run] regenerate {mod_path} ({len(modnames)} modules)")
        else:
            with open(mod_path, "w") as f:
                f.write("\n".join(mod_lines))
    return moved_files, touched


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


def get_functional_line_counts_repack_dry():
    """Variant for --repack --dry-run: scans kernel-gga/src/ AND every
    kernel-gga-* sub-crate, summing line counts across ALL lettered
    sub-crates that contribute to the same functional name (excluding
    duplicate mod.rs which gets regenerated at merge time). Mirrors the
    accounting that a live gather+merge run would produce.
    """
    functionals: dict[str, int] = {}
    for name in sorted(os.listdir(GGA_SRC)):
        path = os.path.join(GGA_SRC, name)
        if not os.path.isdir(path):
            continue
        total = 0
        for fname in os.listdir(path):
            if fname.endswith(".rs") and fname != "mod.rs":
                fpath = os.path.join(path, fname)
                with open(fpath, "r") as f:
                    total += sum(1 for _ in f)
        functionals[name] = total
    for entry in sorted(os.listdir(CRATES_DIR)):
        if not SUBCRATE_RE.match(entry):
            continue
        sub_src = os.path.join(CRATES_DIR, entry, "src")
        if not os.path.isdir(sub_src):
            continue
        for item in sorted(os.listdir(sub_src)):
            item_path = os.path.join(sub_src, item)
            if not os.path.isdir(item_path):
                continue
            for fname in os.listdir(item_path):
                if fname.endswith(".rs") and fname != "mod.rs":
                    fpath = os.path.join(item_path, fname)
                    with open(fpath, "r") as f:
                        functionals[item] = functionals.get(item, 0) + sum(
                            1 for _ in f
                        )
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


def delete_old_subcrates(dry_run: bool = False):
    """Remove old kernel-gga-N (numeric) AND kernel-gga-Nx (lettered) sub-crates.

    The lettered pattern is what `split_oversized_kernel.py` produces when a
    single-bin functional exceeds TARGET_MAX; on a repack we want to drop both.
    """
    removed = []
    for name in os.listdir(CRATES_DIR):
        if SUBCRATE_RE.match(name):
            path = os.path.join(CRATES_DIR, name)
            if dry_run:
                print(f"  [dry-run] rm -rf {path}")
            else:
                shutil.rmtree(path)
            removed.append(name)
    if removed:
        # Sort: numeric-only first, then lettered, both in numeric/letter order.
        def sort_key(s: str) -> tuple[int, str]:
            tail = s.split("-")[-1]
            m = re.match(r"^(\d+)([a-p])?$", tail)
            return (int(m.group(1)), m.group(2) or "") if m else (10**9, tail)
        print(f"Removed {len(removed)} old sub-crates: "
              f"{', '.join(sorted(removed, key=sort_key))}")


def create_subcrate(bin_index, functionals_in_bin):
    """Create a single kernel-gga-N sub-crate."""
    n = bin_index + 1
    crate_name = f"gga-{n}"
    crate_dir = os.path.join(CRATES_DIR, crate_name)
    src_dir = os.path.join(crate_dir, "src")
    os.makedirs(src_dir, exist_ok=True)

    # Move functional directories out of the aggregator into this sub-crate.
    # Originally `shutil.copytree`; that left duplicated trees behind in
    # kernel-gga/src/ when the aggregator was treated as gathered staging
    # (q05 commit 3af6b262 demonstrated the resulting 137 MB / 1810-file
    # orphan). Move semantics drains the aggregator naturally so a follow-up
    # `--repack` does not trip the merge guard.
    for name, _ in functionals_in_bin:
        src_path = os.path.join(GGA_SRC, name)
        dst_path = os.path.join(src_dir, name)
        if os.path.exists(dst_path):
            shutil.rmtree(dst_path)
        shutil.move(src_path, dst_path)

    # Generate Cargo.toml (no feature gates)
    cargo_content = f"""[package]
name = "libxc-kernel-gga-{n}"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = {{ version = "0.10.0", default-features = false, features = ["cpu"] }}
libxc-kernel-math = {{ path = "../math" }}
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
        'cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }',
        'libxc-kernel-math = { path = "../math" }',
    ]
    for i in range(1, num_subcrates + 1):
        deps.append(f'libxc-kernel-gga-{i} = {{ path = "../gga-{i}" }}')

    cargo_content = f"""[package]
name = "libxc-kernel-gga"
version = "0.1.0"
edition = "2024"

[dependencies]
{chr(10).join(deps)}
"""
    facade_toml = os.path.join(CRATES_DIR, "gga", "Cargo.toml")
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

    facade_lib = os.path.join(CRATES_DIR, "gga", "src", "lib.rs")
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
            # Skip old kernel-gga-N AND kernel-gga-Nx entries (but keep kernel-gga itself)
            stripped = line.strip().strip(',').strip('"').strip("'")
            if re.match(r'^crates/kernels/gga-\d+[a-p]?$', stripped):
                # Insert new entries right before the first old one (once)
                if not gga_entries_inserted:
                    for i in range(1, num_subcrates + 1):
                        new_lines.append(f'    "crates/kernels/gga-{i}",')
                    gga_entries_inserted = True
                continue

            # Detect end of members array
            if ']' in line:
                # If we never saw old entries, insert before closing bracket
                if not gga_entries_inserted:
                    for i in range(1, num_subcrates + 1):
                        new_lines.append(f'    "crates/kernels/gga-{i}",')
                    gga_entries_inserted = True
                in_members = False

        new_lines.append(line)

    with open(CARGO_TOML, "w") as f:
        f.write('\n'.join(new_lines))


def main():
    dry_run = '--dry-run' in sys.argv
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

    # Step 0: Auto-gather. If kernel-gga-* sub-crates exist (numeric or
    # lettered), move every per-functional dir back into kernel-gga/src/ so
    # the bin-packer sees the full set. No-op if the aggregator already holds
    # the source of truth.
    existing_subs = [e for e in os.listdir(CRATES_DIR) if SUBCRATE_RE.match(e)]
    if existing_subs:
        print(f"Pre-gather: {len(existing_subs)} existing kernel-gga-* sub-crate(s) "
              f"will be drained into {os.path.relpath(GGA_SRC, PROJECT_ROOT)}/")
        moved_files, touched = gather_into_aggregator(dry_run)
        print(f"  merged {moved_files} .rs file(s) across {len(touched)} functional(s).\n")

    # Step 1: Inventory. Use the dry-run-aware variant when applicable so the
    # bin-pack accounting matches a live run.
    if dry_run and existing_subs:
        functionals = get_functional_line_counts_repack_dry()
    else:
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

    if dry_run:
        print("Dry run -- no changes made.")
        return

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
