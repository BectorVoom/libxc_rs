#!/usr/bin/env python3
"""Split crates/kernel-lda mono-crate into bin-packed sub-crates.

Currently `crates/kernel-lda` is a single ~11 MB Rust crate containing 41
LDA functionals × 5 derivative orders × 2 spin modes = ~410 launch kernels
that compile in one rustc invocation. Per
`docs/manual/Cubecl/cubecl_macro_fanout_manual.md` §16 ("Split Crates to
Reduce Recompilation Scope") and matching the GGA/MGGA structure
(22/37 sub-crates respectively), bin-pack the LDA functionals into
~5 sub-crates so that workspace builds parallelise better and per-rustc
peak RAM during proc-macro expansion drops.

Strategy:
1. Scan `crates/kernel-lda/src/*/` for per-functional line counts.
2. First-fit-decreasing bin pack into ≤TARGET_MAX-line bins.
3. For each bin produce `crates/kernel-lda-<N>/` with Cargo.toml + lib.rs
   re-declaring `pub mod <func>;` for each functional in the bin.
4. Move (`mv`) each functional directory from kernel-lda/src/ into the
   new sub-crate's src/.
5. Rewrite the kernel-lda aggregator's lib.rs to `pub use libxc_kernel_lda_<N>::<func>;`
   for each functional, so existing imports
   (`use libxc_kernel_lda::lda_x::*`) keep working unchanged.
6. Update kernel-lda/Cargo.toml to depend on each new sub-crate.
7. Update workspace root Cargo.toml: add new members + new path deps.

`deferred.rs` and any non-functional metadata files in
`crates/kernel-lda/src/` stay in the aggregator (they're shared
metadata, not generated kernel code).

Usage:
    python3 tools/split_lda_subcrates.py [--dry-run] [--target-max LINES]

`--dry-run` prints the bin assignment + would-be edits without touching
the filesystem.

This script is idempotent: a second run with no changes since the last
emits "already split" and exits cleanly.
"""

import re
import shutil
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = REPO_ROOT / "crates"
LDA_AGG = CRATES_DIR / "kernel-lda"
WORKSPACE_CARGO = REPO_ROOT / "Cargo.toml"

TARGET_MAX = 100_000  # lines per sub-crate (matches GGA/MGGA)
SUFFIXES = list("abcdefghijklmnop")  # only used if a single functional > TARGET_MAX


def per_func_lines(src_dir: Path) -> list[tuple[str, int]]:
    """Return [(funcname, total_lines)] for each functional sub-directory."""
    out: list[tuple[str, int]] = []
    for entry in sorted(src_dir.iterdir()):
        if not entry.is_dir():
            continue
        n = 0
        for rs in entry.rglob("*.rs"):
            n += sum(1 for _ in rs.open(errors="replace"))
        out.append((entry.name, n))
    return out


def bin_pack(items: list[tuple[str, int]], cap: int) -> list[list[tuple[str, int]]]:
    bins: list[list[tuple[str, int]]] = []
    for name, n in sorted(items, key=lambda x: -x[1]):
        for b in bins:
            if sum(x[1] for x in b) + n <= cap:
                b.append((name, n))
                break
        else:
            bins.append([(name, n)])
    bins.sort(key=lambda b: sorted(x[0] for x in b)[0])
    return bins


def make_subcrate_cargo(sub_idx: int) -> str:
    return (
        "[package]\n"
        f'name = "libxc-kernel-lda-{sub_idx}"\n'
        'version = "0.1.0"\n'
        'edition = "2024"\n\n'
        "[dependencies]\n"
        'cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }\n'
        'libxc-kernel-math = { path = "../kernel-math" }\n'
    )


def make_subcrate_libfile(sub_idx: int, funcs: list[str]) -> str:
    out = [
        "#![allow(clippy::excessive_precision)]\n",
        "#![allow(clippy::needless_late_init)]\n",
        "#![allow(clippy::too_many_arguments)]\n",
        "\n",
        f"//! LDA kernel translations: kernel-lda-{sub_idx} "
        f"({len(funcs)} functional{'s' if len(funcs) != 1 else ''}).\n",
        "\n",
    ]
    for f in sorted(funcs):
        out.append(f"pub mod {f};\n")
    return "".join(out)


def make_aggregator_libfile(func_to_sub: dict[str, int]) -> str:
    out = [
        "#![allow(clippy::excessive_precision)]\n",
        "#![allow(clippy::needless_late_init)]\n",
        "#![allow(clippy::too_many_arguments)]\n",
        "\n",
        "//! LDA kernel implementations (incremental derivative structure).\n",
        f"//! Auto-generated: {len(func_to_sub)} functionals re-exported from "
        f"{len(set(func_to_sub.values()))} sub-crates.\n",
        "//! Bin-packed by tools/split_lda_subcrates.py\n",
        "\n",
        "// `deferred` is in-aggregator metadata (not a kernel module).\n",
        "pub mod deferred;\n",
        "\n",
        "// Functional re-exports (paths preserved so that\n",
        "// `use libxc_kernel_lda::<func>::*;` keeps working).\n",
    ]
    for func in sorted(func_to_sub):
        sub = func_to_sub[func]
        out.append(f"pub use libxc_kernel_lda_{sub}::{func};\n")
    return "".join(out)


def make_aggregator_cargo(num_subs: int, original_aggregator_cargo: str) -> str:
    """Rewrite aggregator Cargo.toml to depend on the sub-crates.

    Keeps the existing top-level shape (package, edition, dev-dependencies)
    and inserts/updates the [dependencies] section.
    """
    # Find the [dependencies] block and replace.
    lines = original_aggregator_cargo.splitlines(keepends=False)
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.strip() == "[dependencies]":
            out.append("[dependencies]")
            for n in range(1, num_subs + 1):
                out.append(
                    f'libxc-kernel-lda-{n} = {{ path = "../kernel-lda-{n}" }}'
                )
            # skip until next [section] or EOF
            i += 1
            while i < len(lines) and not lines[i].lstrip().startswith("["):
                i += 1
            continue
        out.append(line)
        i += 1
    return "\n".join(out) + "\n"


WORKSPACE_DEP_RE = re.compile(
    r'^libxc-kernel-lda = \{ path = "crates/kernel-lda" \}\s*$',
    re.MULTILINE,
)
WORKSPACE_MEMBER_RE = re.compile(
    r'^\s*"crates/kernel-lda",\s*$',
    re.MULTILINE,
)


def update_workspace_cargo(num_subs: int, content: str) -> str:
    """Insert the new sub-crate members and path-deps into workspace Cargo.toml."""
    new_deps = "\n".join(
        f'libxc-kernel-lda = {{ path = "crates/kernel-lda" }}' if n == 0
        else f'libxc-kernel-lda-{n} = {{ path = "crates/kernel-lda-{n}" }}'
        for n in range(0, num_subs + 1)
    )
    content = WORKSPACE_DEP_RE.sub(new_deps, content, count=1)

    new_members = "\n".join(
        f'    "crates/kernel-lda",' if n == 0
        else f'    "crates/kernel-lda-{n}",'
        for n in range(0, num_subs + 1)
    )
    content = WORKSPACE_MEMBER_RE.sub(new_members, content, count=1)
    return content


def already_split() -> bool:
    """True if any kernel-lda-N sub-crate already exists."""
    for entry in CRATES_DIR.iterdir():
        if entry.is_dir() and re.match(r"^kernel-lda-\d+", entry.name):
            return True
    return False


def main() -> int:
    args = sys.argv[1:]
    dry_run = "--dry-run" in args
    target = TARGET_MAX
    for a in args:
        if a.startswith("--target-max="):
            target = int(a.split("=", 1)[1])

    if not LDA_AGG.exists():
        print(f"FATAL: {LDA_AGG} does not exist", file=sys.stderr)
        return 2

    if already_split() and not dry_run:
        print("Already split (kernel-lda-<N> sub-crate exists). Refusing to redo.")
        return 0

    src = LDA_AGG / "src"
    items = per_func_lines(src)
    bins = bin_pack(items, target)

    print(f"Found {len(items)} LDA functionals; total "
          f"{sum(n for _, n in items):,} lines.")
    print(f"Target max per sub-crate: {target:,} lines.")
    print(f"Bin-packed into {len(bins)} sub-crates:\n")
    for idx, b in enumerate(bins, start=1):
        total = sum(n for _, n in b)
        names = ", ".join(sorted(name for name, _ in b))
        print(f"  kernel-lda-{idx}: {total:>6,}L  [{len(b)} func(s)]  {names}")

    func_to_sub = {name: idx for idx, b in enumerate(bins, start=1)
                   for name, _ in b}

    if dry_run:
        print("\nDry run -- no changes made.")
        return 0

    # 1. Materialise sub-crates and move functional dirs.
    for idx, b in enumerate(bins, start=1):
        sub_dir = CRATES_DIR / f"kernel-lda-{idx}"
        (sub_dir / "src").mkdir(parents=True, exist_ok=True)
        # Cargo.toml
        (sub_dir / "Cargo.toml").write_text(make_subcrate_cargo(idx))
        # lib.rs declares all functionals in this bin
        (sub_dir / "src" / "lib.rs").write_text(
            make_subcrate_libfile(idx, [name for name, _ in b])
        )
        # Move each functional directory
        for name, _ in b:
            src_dir = LDA_AGG / "src" / name
            dst_dir = sub_dir / "src" / name
            if not src_dir.exists():
                print(f"WARN: {src_dir} missing, skipping move")
                continue
            shutil.move(str(src_dir), str(dst_dir))

    # 2. Rewrite aggregator lib.rs to re-export from sub-crates.
    (LDA_AGG / "src" / "lib.rs").write_text(make_aggregator_libfile(func_to_sub))

    # 3. Rewrite aggregator Cargo.toml deps.
    agg_cargo = LDA_AGG / "Cargo.toml"
    original = agg_cargo.read_text()
    agg_cargo.write_text(make_aggregator_cargo(len(bins), original))

    # 4. Update workspace Cargo.toml.
    ws = WORKSPACE_CARGO.read_text()
    ws = update_workspace_cargo(len(bins), ws)
    WORKSPACE_CARGO.write_text(ws)

    print(f"\nDone. Created {len(bins)} sub-crates under crates/kernel-lda-1..{len(bins)}/.")
    print("Aggregator crates/kernel-lda/ keeps `pub mod deferred` and re-exports each functional.")
    print("Run `cargo metadata` to verify workspace + dependency resolution.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
