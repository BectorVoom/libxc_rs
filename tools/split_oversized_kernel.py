#!/usr/bin/env python3
"""Split every oversized kernel sub-crate into per-file bin-packed sub-crates.

Background: rebatch_*.py bin-packs at FUNCTIONAL granularity (≤50K lines/crate).
When a single functional exceeds 50K it goes in a solo crate; those solo crates
OOM rustc during compile. This script does PER-FILE bin-packing within each
oversized functional, producing kernel-{family}-Na, Nb, Nc, ... sub-crates each
≤50K lines.

Each new crate keeps the same module path `<functional>::<file>` so call sites
are unaffected.

Usage: python3 tools/split_oversized_kernel.py <family> [--dry-run]
       family ∈ {lda, gga, mgga}
"""

import os
import sys
import shutil

WORKSPACE = "/workspace"
CRATES_DIR = os.path.join(WORKSPACE, "crates")
TARGET_MAX = 50000
SUFFIXES = list("abcdefghijklmnop")


def scan_family(family):
    """Return {crate: {functional: [(file, lines), ...]}} for the family,
    skipping already-split crates and the aggregator."""
    prefix = f"kernel-{family}-"
    aggregator = f"kernel-{family}"
    out = {}
    for entry in sorted(os.listdir(CRATES_DIR)):
        if not entry.startswith(prefix) or entry == aggregator:
            continue
        stripped = entry[len(prefix):]
        if stripped and stripped[-1].isalpha() and stripped[:-1].isdigit():
            continue
        src_dir = os.path.join(CRATES_DIR, entry, "src")
        if not os.path.isdir(src_dir):
            continue
        crate_funcs = {}
        for item in sorted(os.listdir(src_dir)):
            path = os.path.join(src_dir, item)
            if not os.path.isdir(path):
                continue
            files = []
            for rs in os.listdir(path):
                if rs.endswith(".rs") and rs != "mod.rs":
                    n = sum(1 for _ in open(os.path.join(path, rs)))
                    files.append((rs, n))
            crate_funcs[item] = files
        if crate_funcs:
            out[entry] = crate_funcs
    return out


def find_oversized(family):
    """Return list of (crate, functional, [(file, lines), ...]) for oversized funcs."""
    out = []
    for crate, funcs in scan_family(family).items():
        for fname, files in funcs.items():
            if sum(n for _, n in files) > TARGET_MAX:
                out.append((crate, fname, files))
    return out


def find_non_oversized_siblings(family, source_crates):
    """Return {crate: {functional: [(file, lines)]}} for siblings of oversized
    functionals in source_crates that themselves are NOT oversized."""
    out = {}
    all_data = scan_family(family)
    for crate in source_crates:
        if crate not in all_data:
            continue
        for fname, files in all_data[crate].items():
            if sum(n for _, n in files) > TARGET_MAX:
                continue  # oversized — handled separately
            out.setdefault(crate, {})[fname] = files
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
name = "libxc-kernel-{family}-{name}"
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


def make_lib_rs(family, crate_name, functionals):
    """functionals: list of functional module names to declare."""
    if isinstance(functionals, str):
        functionals = [functionals]
    label = (functionals[0] + " subset" if len(functionals) == 1
             else f"{len(functionals)} sibling functionals")
    lines = [
        '#![allow(clippy::excessive_precision)]\n',
        '#![allow(clippy::needless_late_init)]\n',
        '#![allow(clippy::too_many_arguments)]\n',
        '\n',
        f'//! {family.upper()} kernel translations: {crate_name} ({label}).\n',
        '\n',
    ]
    for f in functionals:
        lines.append(f'pub mod {f};\n')
    return ''.join(lines)


def make_mod_rs(file_subset, functional):
    lines = [f'//! {functional.upper()} kernel — split into per-function files.\n', '\n']
    for fname, _ in sorted(file_subset, key=lambda x: x[0]):
        modname = fname[:-3]
        lines.append(f'pub mod {modname};\n')
    return ''.join(lines)


def update_workspace_and_aggregator(family, crate_renames):
    prefix = f"kernel-{family}-"
    aggregator = f"kernel-{family}"
    crate_id_prefix = f"libxc-kernel-{family}-"
    rust_id_prefix = f"libxc_kernel_{family}_"

    ws_cargo = os.path.join(WORKSPACE, "Cargo.toml")
    ag_cargo = os.path.join(CRATES_DIR, aggregator, "Cargo.toml")
    ag_lib = os.path.join(CRATES_DIR, aggregator, "src", "lib.rs")

    with open(ws_cargo) as f: ws_content = f.read()
    with open(ag_cargo) as f: ag_content = f.read()
    with open(ag_lib) as f:   lib_content = f.read()

    for old, news in crate_renames:
        n_old = old.replace(prefix, "")  # e.g. "4"
        # workspace dep line
        old_dep_ws = f'{crate_id_prefix}{n_old} = {{ path = "crates/{old}" }}'
        new_dep_ws = '\n'.join(
            f'{crate_id_prefix}{nc.replace(prefix, "")} = '
            f'{{ path = "crates/{nc}" }}' for nc in news
        )
        ws_content = ws_content.replace(old_dep_ws, new_dep_ws)
        old_member = f'    "crates/{old}",'
        new_members = '\n'.join(f'    "crates/{nc}",' for nc in news)
        ws_content = ws_content.replace(old_member, new_members)

        # aggregator Cargo dep
        old_dep_ag = f'{crate_id_prefix}{n_old} = {{ path = "../{old}" }}'
        new_dep_ag = '\n'.join(
            f'{crate_id_prefix}{nc.replace(prefix, "")} = '
            f'{{ path = "../{nc}" }}' for nc in news
        )
        ag_content = ag_content.replace(old_dep_ag, new_dep_ag)

        # aggregator lib.rs re-export
        old_export = f'pub use {rust_id_prefix}{n_old} as batch{n_old};'
        new_exports = '\n'.join(
            f'pub use {rust_id_prefix}{nc.replace(prefix, "")} '
            f'as batch{nc.replace(prefix, "")};' for nc in news
        )
        lib_content = lib_content.replace(old_export, new_exports)

    with open(ws_cargo, 'w') as f: f.write(ws_content)
    with open(ag_cargo, 'w') as f: f.write(ag_content)
    with open(ag_lib, 'w') as f:   f.write(lib_content)


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    dry_run = '--dry-run' in sys.argv
    if not args or args[0] not in ("lda", "gga", "mgga"):
        print("Usage: split_oversized_kernel.py <lda|gga|mgga> [--dry-run]")
        sys.exit(1)
    family = args[0]
    prefix = f"kernel-{family}-"

    oversized = find_oversized(family)
    print(f"Found {len(oversized)} oversized {family.upper()} functionals (>{TARGET_MAX} lines):\n")

    plan = []  # list of (crate, func, bins, new_crates) for oversized splits
    suffix_offset = {}  # source crate -> next free suffix index
    for crate, func, files in oversized:
        bins = bin_pack(files, TARGET_MAX)
        n_old = crate.replace(prefix, "")
        if not n_old.isdigit():
            print(f"  SKIP {crate} (non-pure-numeric suffix)")
            continue
        offset = suffix_offset.get(crate, 0)
        new_crates = [f"{prefix}{n_old}{SUFFIXES[offset + i]}"
                      for i in range(len(bins))]
        suffix_offset[crate] = offset + len(bins)
        total = sum(n for _, n in files)
        print(f"  {crate}/{func}: {total:,}L → {len(bins)} sub-crates "
              f"({', '.join(c.replace(prefix, '') for c in new_crates)})")
        plan.append((crate, func, bins, new_crates))

    # Sibling functionals (same source crate, not themselves oversized) need a
    # home too — one extra sub-crate per source crate that has them.
    source_crates = {p[0] for p in plan}
    siblings = find_non_oversized_siblings(family, source_crates)
    sibling_plan = []  # list of (crate, {func: files}, new_crate)
    for crate, func_map in siblings.items():
        if not func_map:
            continue
        n_old = crate.replace(prefix, "")
        offset = suffix_offset.get(crate, 0)
        new_crate = f"{prefix}{n_old}{SUFFIXES[offset]}"
        suffix_offset[crate] = offset + 1
        total = sum(sum(n for _, n in fs) for fs in func_map.values())
        names = ", ".join(sorted(func_map.keys()))
        print(f"  {crate} siblings: {total:,}L ({names}) → {new_crate.replace(prefix, '')}")
        sibling_plan.append((crate, func_map, new_crate))

    if dry_run:
        total_new = sum(len(p[3]) for p in plan) + len(sibling_plan)
        all_old = {p[0] for p in plan} | {p[0] for p in sibling_plan}
        print(f"\nTotal new crates: {total_new}")
        print(f"Total deletions: {len(all_old)}")
        print("\nDry run -- no changes made.")
        return

    # Materialize oversized-functional sub-crates
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
            n_new = new_crate.replace(prefix, "")
            with open(os.path.join(new_dir, "Cargo.toml"), 'w') as f:
                f.write(CARGO_TOML_TEMPLATE.format(family=family, name=n_new))
            with open(os.path.join(new_dir, "src", "lib.rs"), 'w') as f:
                f.write(make_lib_rs(family, new_crate, [func]))

    # Materialize sibling sub-crates (multiple functionals per crate possible)
    for crate, func_map, new_crate in sibling_plan:
        new_dir = os.path.join(CRATES_DIR, new_crate)
        os.makedirs(os.path.join(new_dir, "src"), exist_ok=True)
        for func, files in func_map.items():
            src_dir = os.path.join(CRATES_DIR, crate, "src", func)
            new_func = os.path.join(new_dir, "src", func)
            os.makedirs(new_func, exist_ok=True)
            for fname, _n in files:
                shutil.copy2(os.path.join(src_dir, fname),
                             os.path.join(new_func, fname))
            with open(os.path.join(new_func, "mod.rs"), 'w') as f:
                f.write(make_mod_rs(files, func))
        n_new = new_crate.replace(prefix, "")
        with open(os.path.join(new_dir, "Cargo.toml"), 'w') as f:
            f.write(CARGO_TOML_TEMPLATE.format(family=family, name=n_new))
        with open(os.path.join(new_dir, "src", "lib.rs"), 'w') as f:
            f.write(make_lib_rs(family, new_crate, sorted(func_map.keys())))

    # Aggregate crate -> all new sub-crates for workspace + aggregator updates
    by_crate = {}
    for crate, _f, _b, news in plan:
        by_crate.setdefault(crate, []).extend(news)
    for crate, _fm, new_crate in sibling_plan:
        by_crate.setdefault(crate, []).append(new_crate)
    update_workspace_and_aggregator(family, list(by_crate.items()))

    for crate in by_crate:
        shutil.rmtree(os.path.join(CRATES_DIR, crate))

    print(f"\nDone. Created {sum(len(p[3]) for p in plan)} new sub-crates, "
          f"removed {len(by_crate)} old ones.")


if __name__ == '__main__':
    main()
