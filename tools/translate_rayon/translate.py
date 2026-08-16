#!/usr/bin/env python3
"""Emit plain-Rust (rayon backend) kernel subcrates from the CubeCL tree.

Usage:
    python3 tools/translate_rayon/translate.py --func gga_x_pbe
    python3 tools/translate_rayon/translate.py --family gga
    python3 tools/translate_rayon/translate.py --all

Reads  crates/kernels/<family>/<func>/src/*.rs        (CubeCL, current tree)
Writes crates/kernels-rayon/<family>/<func>/src/*.rs  (plain Rust)

The CubeCL tree is never modified, so the two can be diffed and the emitted
kernels verified bit-for-bit against it.

Parallelism deliberately lives in the *caller*, not here. Every emitted kernel
takes whole slices and sweeps `0..guard.len()`, so a caller can split each array
at its own per-point stride and hand the sub-slices to a rayon worker. That
keeps the 253,961 generated files free of any `unsafe`, and confines the
stride-aware splitting to one reviewed place in the eval layer.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from xform import UnsupportedKernel, transform_kernel  # noqa: E402

REPO = Path(__file__).resolve().parents[2]
SRC_ROOT = REPO / "crates" / "kernels"
DST_ROOT = REPO / "crates" / "kernels-rayon"
FAMILIES = ("lda", "gga", "mgga")


def crate_name(func: str) -> str:
    return f"libxc-rkernel-{func}"


def find_family(func: str) -> str | None:
    for fam in FAMILIES:
        if (SRC_ROOT / fam / func).is_dir():
            return fam
    return None


def emit_cargo_toml(dst: Path, func: str, needs_libm: bool,
                    siblings: set[str] | None = None) -> str:
    deps = ['libxc-rkernel-math = { path = "../../math" }']
    for sib in sorted(siblings or ()):
        fam = find_family(sib)
        if fam is None:
            continue
        deps.append(f'libxc-rkernel-{sib} = {{ path = "../../{fam}/{sib}" }}')
    if needs_libm:
        deps.append('libm = "0.2"')
    return (
        "[package]\n"
        f'name = "{crate_name(func)}"\n'
        'version = "0.1.0"\n'
        'edition = "2024"\n'
        "\n"
        "[dependencies]\n" + "\n".join(deps) + "\n"
    )


def emit_lib_rs(func: str, modules: list[str]) -> str:
    head = (
        f"//! {func.upper()} kernels - plain Rust, rayon backend.\n"
        "//!\n"
        "//! Auto-emitted by tools/translate_rayon/translate.py - do not hand-edit.\n"
        "//!\n"
        "//! Each kernel sweeps the whole slice it is given. Parallelism is the\n"
        "//! caller's job: split every array at its own per-point stride and call\n"
        "//! this on the sub-slices from a rayon worker.\n"
        "\n"
        "#![allow(unused_imports, unused_variables, non_snake_case, "
        "clippy::excessive_precision, clippy::too_many_arguments, "
        "clippy::needless_return)]\n\n"
    )
    return head + "\n".join(f"pub mod {m};" for m in modules) + "\n"


def _translate_tree(
    src_dir: Path, dst_dir: Path, dry_run: bool
) -> tuple[int, list[str]]:
    """Recursively transform every `.rs` under `src_dir`.

    Returns (files_written, errors). A directory is emitted only if every file
    beneath it converts, so a partially-written module tree can never be left
    behind.
    """
    files = sorted(p for p in src_dir.glob("*.rs"))
    subdirs = sorted(p for p in src_dir.iterdir() if p.is_dir())
    if not any(p.name == "mod.rs" for p in files):
        return 0, [f"{src_dir.name}/: no mod.rs"]

    converted: list[tuple[str, str]] = []
    errors: list[str] = []
    for p in files:
        try:
            converted.append((p.name, transform_kernel(p.read_text())))
        except UnsupportedKernel as exc:
            errors.append(f"{src_dir.name}/{p.name}: {exc}")
    if errors:
        return 0, errors

    written = 0
    if not dry_run:
        dst_dir.mkdir(parents=True, exist_ok=True)
        for name, text in converted:
            (dst_dir / name).write_text(text)
    written += len(converted)

    for sd in subdirs:
        n, errs = _translate_tree(sd, dst_dir / sd.name, dry_run)
        if errs:
            return 0, errs
        written += n
    return written, []


def translate_functional(family: str, func: str, *, dry_run: bool = False) -> dict:
    src_dir = SRC_ROOT / family / func / "src"
    if not src_dir.is_dir():
        raise FileNotFoundError(src_dir)

    dst_dir = DST_ROOT / family / func / "src"
    modules: list[str] = []
    skipped: list[str] = []
    written = 0

    flat = sorted(p for p in src_dir.glob("*.rs") if p.name != "lib.rs")
    nested = sorted(p for p in src_dir.iterdir() if p.is_dir())

    if not dry_run:
        dst_dir.mkdir(parents=True, exist_ok=True)

    for path in flat:
        text = path.read_text()
        try:
            out = transform_kernel(text)
        except UnsupportedKernel as exc:
            skipped.append(f"{path.name}: {exc}")
            continue
        modules.append(path.stem)
        if not dry_run:
            (dst_dir / path.name).write_text(out)
        written += 1

    # Split outputs nest arbitrarily deep. The CubeCL emitter fanned large
    # outputs out to work around cubecl-macros memory blowup, giving up to four
    # levels:
    #
    #   src/<output>.rs                              flat
    #   src/<output>/{mod,partN}.rs                  split
    #   src/<output>/partN/{mod,chunkK}.rs           chunked (up to ~1240 chunks)
    #   src/<output>/partN/metaM/{mod,chunkK}.rs     meta-wrapped
    #
    # Every file in that tree is either a guarded kernel body or a guard-free
    # wrapper that calls its children, and both go through the same transform.
    # So the walk just has to recurse; it must not stop at the first level, or
    # it emits `mod.rs` files referencing modules that were never written.
    for d in nested:
        n, errs = _translate_tree(d, dst_dir / d.name, dry_run)
        if errs:
            skipped.extend(f"{d.name}/{e}" for e in errs)
            continue
        modules.append(d.name)
        written += n

    if not dry_run and modules:
        def _module_text(m: str) -> str:
            f = dst_dir / f"{m}.rs"
            if f.is_file():
                return f.read_text()
            return "\n".join(p.read_text() for p in (dst_dir / m).glob("*.rs"))

        all_text = "\n".join(_module_text(m) for m in modules)
        needs_libm = "libm::" in all_text
        siblings = {
            n for n in re.findall(r"libxc_rkernel_([A-Za-z0-9_]+)", all_text)
            if n != "math" and n != func
        }
        (dst_dir.parent / "Cargo.toml").write_text(
            emit_cargo_toml(dst_dir.parent, func, needs_libm, siblings)
        )
        # Prefer transforming the source lib.rs over synthesising one: the
        # shard crates (mgga_c_tpssloc_p0 etc.) carry `pub use <mod>::<fn>;`
        # re-exports that their sibling's wrapper imports by name, and a
        # synthesised lib.rs silently drops them.
        src_lib = src_dir / "lib.rs"
        if src_lib.is_file():
            lib_text = transform_kernel(src_lib.read_text())
            emitted = set(modules)
            declared = set(re.findall(r"^pub mod (\w+);", lib_text, re.M))
            missing = declared - emitted
            if missing:
                raise UnsupportedKernel(
                    f"lib.rs declares modules that were not emitted: {sorted(missing)}"
                )
            (dst_dir / "lib.rs").write_text(lib_text)
        else:
            (dst_dir / "lib.rs").write_text(emit_lib_rs(func, modules))

    return {"func": func, "written": written, "skipped": skipped, "modules": modules}


def main() -> int:
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--func")
    g.add_argument("--family", choices=FAMILIES)
    g.add_argument("--all", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    targets: list[tuple[str, str]] = []
    if args.func:
        for fam in FAMILIES:
            if (SRC_ROOT / fam / args.func).is_dir():
                targets.append((fam, args.func))
                break
        else:
            print(f"error: no such functional: {args.func}", file=sys.stderr)
            return 2
    else:
        fams = [args.family] if args.family else list(FAMILIES)
        for fam in fams:
            fam_dir = SRC_ROOT / fam
            if fam_dir.is_dir():
                targets += [(fam, d.name) for d in sorted(fam_dir.iterdir()) if d.is_dir()]

    total_written = 0
    total_skipped = 0
    failures = 0
    for fam, func in targets:
        try:
            r = translate_functional(fam, func, dry_run=args.dry_run)
        except Exception as exc:  # noqa: BLE001
            print(f"FAIL {fam}/{func}: {exc}", file=sys.stderr)
            failures += 1
            continue
        total_written += r["written"]
        total_skipped += len(r["skipped"])
        if len(targets) == 1 or r["skipped"]:
            for s in r["skipped"]:
                print(f"  skip {fam}/{func}/{s}")
        if len(targets) == 1:
            print(f"{fam}/{func}: {r['written']} modules -> {DST_ROOT / fam / func}")

    print(
        f"\ntranslated {total_written} kernel modules across {len(targets)} "
        f"functional(s); {total_skipped} skipped, {failures} failed"
    )
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
