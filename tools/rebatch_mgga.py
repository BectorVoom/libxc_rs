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
import re
import sys
import shutil

CRATES_DIR = "crates"
DEFAULT_TARGET_MAX = 500_000  # Memory-permissive default; opt back to 50000 for tight RAM
SUBCRATE_RE = re.compile(r"^kernel-mgga-\d+[a-p]?$")


def get_functional_sizes():
    """Scan existing kernel-mgga-* sub-crates and produce a per-functional
    summary, MERGING shares of the same functional across lettered sub-crates.

    `split_oversized_kernel.py` / `split_oversized_mgga.py` produce
    `kernel-mgga-N{a,b,c,d}/src/<func>/` directories that share the SAME
    functional name but contain DIFFERENT .rs files (each lettered crate
    holds its share of the .rs files for that functional, with a per-share
    mod.rs declaring only that subset).

    Returns a list of dicts:
      [{
         "name": "mgga_x_br89",
         "lines": <total .rs lines across all shares, excluding mod.rs>,
         "is_dir": True/False,         # True when the functional is a
                                        # directory module; False for a
                                        # bare .rs file.
         "sources": [(<crate_dir>, <path_inside_src>), ...],
       }, ...]
    """
    by_name: dict[str, dict] = {}

    for entry in sorted(os.listdir(CRATES_DIR)):
        if not SUBCRATE_RE.match(entry):
            continue
        crate_dir = os.path.join(CRATES_DIR, entry)
        src_dir = os.path.join(crate_dir, "src")
        if not os.path.isdir(src_dir):
            continue

        for item in sorted(os.listdir(src_dir)):
            item_path = os.path.join(src_dir, item)
            if not os.path.isdir(item_path):
                # Bare .rs file module (rare for MGGA but supported).
                if item.endswith('.rs') and item != 'lib.rs':
                    func_name = item[:-3]
                    lines = sum(1 for _ in open(item_path))
                    rec = by_name.setdefault(func_name, {
                        "name": func_name, "lines": 0,
                        "is_dir": False, "sources": [],
                    })
                    if rec["is_dir"]:
                        print(f"FATAL: functional {func_name!r} appears as both "
                              f"a directory module AND a bare .rs file across "
                              f"sub-crates. Refusing to merge.", file=sys.stderr)
                        sys.exit(2)
                    rec["lines"] += lines
                    rec["sources"].append((crate_dir, item))
                continue

            # Directory-based module.
            func_name = item
            sub_lines = 0
            for rs_file in os.listdir(item_path):
                if rs_file.endswith('.rs') and rs_file != 'mod.rs':
                    sub_lines += sum(1 for _ in open(os.path.join(item_path, rs_file)))
            rec = by_name.setdefault(func_name, {
                "name": func_name, "lines": 0,
                "is_dir": True, "sources": [],
            })
            if not rec["is_dir"]:
                print(f"FATAL: functional {func_name!r} appears as both a "
                      f"directory module AND a bare .rs file across sub-crates. "
                      f"Refusing to merge.", file=sys.stderr)
                sys.exit(2)
            rec["lines"] += sub_lines
            rec["sources"].append((crate_dir, item))

    return sorted(by_name.values(), key=lambda r: r["name"])


def bin_pack(funcs, target_max):
    """First-fit decreasing bin packing over per-functional records.

    Each record is the dict shape returned by get_functional_sizes(). Bins
    are lists of those records. Solo-oversized functionals (lines > target)
    get their own bin even though they exceed the target — caller's choice
    to enforce; this matches resplit_gga.py's behavior.
    """
    sorted_funcs = sorted(funcs, key=lambda r: -r["lines"])

    batches: list[list[dict]] = []
    batch_totals: list[int] = []

    for rec in sorted_funcs:
        placed = False
        for i, batch in enumerate(batches):
            if batch_totals[i] + rec["lines"] <= target_max:
                batch.append(rec)
                batch_totals[i] += rec["lines"]
                placed = True
                break
        if not placed:
            batches.append([rec])
            batch_totals.append(rec["lines"])

    batches.sort(key=lambda b: b[0]["name"])
    return batches


def create_cargo_toml(crate_dir, crate_num):
    """Create Cargo.toml for a sub-crate.

    [profile.dev]/[profile.test] sections were removed project-wide
    (commit 4be5c995 'drop dead [profile.*] sections'); they're inherited
    from the workspace root so per-crate copies are dead code.
    """
    content = f"""[package]
name = "libxc-kernel-mgga-{crate_num}"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = {{ version = "0.10.0", default-features = false, features = ["cpu"] }}
libxc-kernel-math = {{ path = "../kernel-math" }}
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
    print(f"Found {len(funcs)} unique functionals "
          f"(merged across {sum(len(r['sources']) for r in funcs)} (sub-crate, "
          f"functional) pairs).")

    # Step 2: Bin pack
    batches = bin_pack(funcs, target_max)
    print(f"Planned {len(batches)} sub-crates (target max {target_max} lines)")

    for i, batch in enumerate(batches, 1):
        total = sum(r["lines"] for r in batch)
        names = ', '.join(r["name"] for r in batch)
        print(f"  mgga-{i}: {total:6d} lines ({len(batch)} funcs): {names}")

    if dry_run:
        print("\nDry run -- no changes made.")
        return

    # Step 3: Stage merged per-functional contents.
    staging = "/tmp/mgga_rebatch_staging"
    if os.path.exists(staging):
        shutil.rmtree(staging)
    os.makedirs(staging)

    print("\nMerging functional shares into staging...")
    for rec in funcs:
        if rec["is_dir"]:
            stage_dir = os.path.join(staging, rec["name"])
            os.makedirs(stage_dir, exist_ok=True)
            for crate_dir, src_relname in rec["sources"]:
                src_dir = os.path.join(crate_dir, "src", src_relname)
                for fn in os.listdir(src_dir):
                    if not fn.endswith(".rs") or fn == "mod.rs":
                        continue
                    src_file = os.path.join(src_dir, fn)
                    dst_file = os.path.join(stage_dir, fn)
                    if os.path.exists(dst_file):
                        print(f"FATAL: {dst_file} already in staging when "
                              f"merging {src_file} — non-disjoint sub-crate "
                              f"split. Refusing to clobber.", file=sys.stderr)
                        sys.exit(2)
                    shutil.copy2(src_file, dst_file)
            # Synthesise mod.rs from the merged file set.
            modnames = sorted(
                fn[:-3] for fn in os.listdir(stage_dir)
                if fn.endswith(".rs") and fn != "mod.rs"
            )
            with open(os.path.join(stage_dir, "mod.rs"), "w") as f:
                f.write("\n".join(f"pub mod {n};" for n in modnames) + "\n")
        else:
            # Bare-file functional. Take the first source as canonical.
            crate_dir, src_relname = rec["sources"][0]
            src = os.path.join(crate_dir, "src", src_relname)
            dst = os.path.join(staging, src_relname)
            shutil.copy2(src, dst)

    # Step 4: Remove old sub-crates (numeric AND lettered).
    print("Removing old sub-crates...")
    for entry in sorted(os.listdir(CRATES_DIR)):
        if SUBCRATE_RE.match(entry):
            shutil.rmtree(os.path.join(CRATES_DIR, entry))
            print(f"  Removed {entry}")

    # Step 5: Create new sub-crates.
    print(f"\nCreating {len(batches)} new sub-crates...")
    for i, batch in enumerate(batches, 1):
        crate_name = f"kernel-mgga-{i}"
        crate_dir = os.path.join(CRATES_DIR, crate_name)
        src_dir = os.path.join(crate_dir, "src")
        os.makedirs(src_dir, exist_ok=True)

        create_cargo_toml(crate_dir, i)

        func_names = []
        for rec in batch:
            if rec["is_dir"]:
                src = os.path.join(staging, rec["name"])
                dst = os.path.join(src_dir, rec["name"])
                shutil.copytree(src, dst)
            else:
                src = os.path.join(staging, rec["name"] + ".rs")
                dst = os.path.join(src_dir, rec["name"] + ".rs")
                shutil.copy2(src, dst)
            func_names.append(rec["name"])

        create_lib_rs(crate_dir, func_names, i)

        total = sum(r["lines"] for r in batch)
        print(f"  Created {crate_name}: {len(func_names)} funcs, {total} lines")

    shutil.rmtree(staging)

    # Step 6: Update kernel-mgga aggregator (Cargo.toml + lib.rs).
    n = len(batches)
    update_facade(n)
    print("Updated kernel-mgga facade (Cargo.toml + lib.rs)")

    # Step 7: Update workspace root Cargo.toml.
    update_workspace(n)
    print("Updated workspace members in root Cargo.toml")

    print(f"\n{'='*60}")
    print(f"REBATCHING COMPLETE: {n} sub-crates created")
    print(f"{'='*60}")

    with open("/tmp/mgga_batch_count.txt", "w") as f:
        f.write(str(n))


def update_facade(num_subcrates):
    """Rewrite kernel-mgga/Cargo.toml and kernel-mgga/src/lib.rs."""
    facade_dir = os.path.join(CRATES_DIR, "kernel-mgga")
    deps = [
        'cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }',
        'libxc-kernel-math = { path = "../kernel-math" }',
    ]
    for i in range(1, num_subcrates + 1):
        deps.append(f'libxc-kernel-mgga-{i} = {{ path = "../kernel-mgga-{i}" }}')
    cargo_content = (
        '[package]\n'
        'name = "libxc-kernel-mgga"\n'
        'version = "0.1.0"\n'
        'edition = "2024"\n'
        '\n'
        '[dependencies]\n'
        + '\n'.join(deps) + '\n'
    )
    with open(os.path.join(facade_dir, "Cargo.toml"), "w") as f:
        f.write(cargo_content)

    # lib.rs preserves any non-batch-export prefix (e.g. `pub mod deferred;`)
    # if present, then re-emits the numeric `batch{i}` aliases.
    lib_path = os.path.join(facade_dir, "src", "lib.rs")
    preserved_prefix: list[str] = []
    if os.path.exists(lib_path):
        for line in open(lib_path):
            stripped = line.strip()
            if stripped.startswith("pub use libxc_kernel_mgga_"):
                break
            preserved_prefix.append(line.rstrip("\n"))
    if not preserved_prefix or not any(s.strip() for s in preserved_prefix):
        preserved_prefix = [
            "#![allow(clippy::excessive_precision)]",
            "#![allow(clippy::needless_late_init)]",
            "#![allow(clippy::too_many_arguments)]",
            "",
            "//! MGGA kernel translations from maple2c.",
            "",
            "// Re-export sub-crates containing compiled MGGA functionals.",
        ]
    out_lines = list(preserved_prefix)
    for i in range(1, num_subcrates + 1):
        out_lines.append(f"pub use libxc_kernel_mgga_{i} as batch{i};")
    out_lines.append("")
    with open(lib_path, "w") as f:
        f.write("\n".join(out_lines))


def update_workspace(num_subcrates):
    """Update workspace root Cargo.toml: strip stale kernel-mgga-* entries
    (numeric or lettered) and emit fresh kernel-mgga-1..N entries."""
    cargo_path = os.path.join(os.path.dirname(CRATES_DIR), "Cargo.toml") \
        if os.path.dirname(CRATES_DIR) else "Cargo.toml"
    content = open(cargo_path).read()

    # Strip every libxc-kernel-mgga-<id> dep line under [workspace.dependencies].
    content = re.sub(
        r'^libxc-kernel-mgga-\d+[a-p]?\s*=\s*\{\s*path\s*=\s*'
        r'"crates/kernel-mgga-\d+[a-p]?"\s*\}\s*\n',
        "",
        content, flags=re.MULTILINE,
    )
    # Strip every "crates/kernel-mgga-<id>", member line.
    content = re.sub(
        r'^\s*"crates/kernel-mgga-\d+[a-p]?",\s*\n',
        "",
        content, flags=re.MULTILINE,
    )
    # Insert fresh deps right after the kernel-mgga aggregator dep line.
    new_deps = "\n".join(
        f'libxc-kernel-mgga-{i} = {{ path = "crates/kernel-mgga-{i}" }}'
        for i in range(1, num_subcrates + 1)
    ) + "\n"
    content = re.sub(
        r'(^libxc-kernel-mgga\s*=\s*\{\s*path\s*=\s*"crates/kernel-mgga"\s*\}\s*\n)',
        r'\1' + new_deps,
        content, count=1, flags=re.MULTILINE,
    )
    # Insert fresh members right after the kernel-mgga member line.
    new_members = "\n".join(
        f'    "crates/kernel-mgga-{i}",'
        for i in range(1, num_subcrates + 1)
    ) + "\n"
    content = re.sub(
        r'(^\s*"crates/kernel-mgga",\s*\n)',
        r'\1' + new_members,
        content, count=1, flags=re.MULTILINE,
    )

    with open(cargo_path, "w") as f:
        f.write(content)


if __name__ == '__main__':
    main()
