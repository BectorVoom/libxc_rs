#!/usr/bin/env python3
"""Emit plain-Rust (rayon backend) kernel subcrates from the CubeCL tree.

Usage:
    python3 tools/translate_rayon/translate.py --func gga_x_pbe
    python3 tools/translate_rayon/translate.py --family gga
    python3 tools/translate_rayon/translate.py --all

Reads  archive/kernels-cubecl/<family>/<func>/src/*.rs  (CubeCL, archived)
Writes crates/kernels-rayon/<family>/<func>/src/*.rs    (plain Rust)

The CubeCL tree is never modified, so the two can be diffed and the emitted
kernels verified bit-for-bit against it.

Parallelism deliberately lives in the *caller*, not here. Every emitted kernel
takes whole slices and sweeps one iteration per grid point (`0..guard.len() / D`
for a guard buffer with D elements per point), so a caller can split each array
at its own per-point stride and hand the sub-slices to a rayon worker. That
keeps the generated tree free of any `unsafe`, and confines the stride-aware
splitting to one reviewed place in the eval layer.

The CubeCL tree's chunk/meta helper fan-out (253,961 files, kept only to bound
cubecl-macros memory) is flattened back into the part functions on the way
through — see flatten.py. The emitted tree is ~5.9k files.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from flatten import flatten_helper_dir  # noqa: E402
from vnmerge import MergeError, merge_texts  # noqa: E402
from xform import UnsupportedKernel, transform_kernel  # noqa: E402

REPO = Path(__file__).resolve().parents[2]
# The CubeCL tree moved to archive/ in ADR 0001; it remains the transform's
# input so the emitted kernels stay bit-checkable against it.
SRC_ROOT = REPO / "archive" / "kernels-cubecl"
DST_ROOT = REPO / "crates" / "kernels-rayon"
FAMILIES = ("lda", "gga", "mgga")


def _write_if_changed(path: Path, text: str) -> None:
    """Write `text` to `path` unless it is already byte-identical.

    Preserving mtimes on unchanged files keeps cargo from rebuilding kernel
    crates a regen did not actually touch (same idea as the sync emitter in
    tools/maple_to_kernels.py).
    """
    try:
        if path.read_text() == text:
            return
    except FileNotFoundError:
        pass
    path.write_text(text)


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


def _has_struct_interface(src_dir: Path) -> bool:
    """True for chunk-first struct-interface outputs (gga_c_pbe-style).

    Those chunks return structs, not tuples, and are already CSE-optimal
    (a handful of chunks, no cross-chunk recompute), so they are copied 1:1
    rather than flattened.
    """
    return any(
        "pub struct " in p.read_text()
        for p in src_dir.rglob("chunk*.rs")
    )


def _collect_verbatim(src_dir: Path, rel: str) -> tuple[dict[str, str], list[str]]:
    """Old-style 1:1 recursive transform of a module tree.

    Returns ({relpath: text}, errors). A directory converts only if every
    file beneath it does, so a partial module tree can never be emitted.
    """
    files = sorted(p for p in src_dir.glob("*.rs"))
    subdirs = sorted(p for p in src_dir.iterdir() if p.is_dir())
    if not any(p.name == "mod.rs" for p in files):
        return {}, [f"{src_dir.name}/: no mod.rs"]

    out: dict[str, str] = {}
    errors: list[str] = []
    for p in files:
        try:
            out[f"{rel}/{p.name}"] = transform_kernel(p.read_text())
        except UnsupportedKernel as exc:
            errors.append(f"{src_dir.name}/{p.name}: {exc}")
    if errors:
        return {}, errors

    for sd in subdirs:
        sub, errs = _collect_verbatim(sd, f"{rel}/{sd.name}")
        if errs:
            return {}, errs
        out.update(sub)
    return out, []


_SHARD_USE = re.compile(r"^use libxc_kernel_(\w+)::(\w+);$", re.M)


def _collect_shard_parts(src_dir: Path, outname: str
                         ) -> tuple[dict[str, str], list[str]]:
    """Resolve part bodies a wrapper imports from sibling shard crates.

    A handful of functionals (mgga_c_tpssloc, revtpss, rmggac, tpss, kcisk)
    were split across companion crates `<func>_p0..pN` purely to get under
    cubecl-macros' memory ceiling: the parent holds only the wrapper, and its
    parts live next door. Post-CubeCL there is no reason to keep them apart,
    and keeping them apart blocks the value-numbering merge, which is where
    these functionals have the most to gain. So the parts are pulled in here
    and emitted as if they had always been local; the shard crates then have
    no dependents and are dropped.

    Returns ({synthetic partN.rs name: text}, errors).
    """
    mod_path = src_dir / "mod.rs"
    if not mod_path.is_file():
        return {}, []
    refs = _SHARD_USE.findall(mod_path.read_text())
    if not refs:
        return {}, []

    out: dict[str, str] = {}
    errors: list[str] = []
    for idx, (crate, fn) in enumerate(refs):
        fam = find_family(crate)
        if fam is None:
            errors.append(f"{outname}: no such shard crate `{crate}`")
            continue
        shard_out = SRC_ROOT / fam / crate / "src" / outname
        flat = shard_out / f"{_part_stem(fn)}.rs"
        nested = shard_out / _part_stem(fn)
        try:
            if flat.is_file():
                text = transform_kernel(flat.read_text())
            elif nested.is_dir():
                text = flatten_helper_dir(nested, transform_kernel)
            else:
                errors.append(f"{outname}: `{fn}` not found in {crate}")
                continue
        except UnsupportedKernel as exc:
            errors.append(f"{outname}: {crate}/{fn}: {exc}")
            continue
        if f"pub fn {fn}(" not in text:
            errors.append(f"{outname}: {crate} part does not define `{fn}`")
            continue
        out[f"part{idx}.rs"] = text
    return out, errors


def shard_crates() -> set[str]:
    """Every companion crate whose parts a parent wrapper imports."""
    found: set[str] = set()
    for fam in FAMILIES:
        fam_dir = SRC_ROOT / fam
        if not fam_dir.is_dir():
            continue
        for mod in fam_dir.glob("*/src/*/mod.rs"):
            found.update(c for c, _fn in _SHARD_USE.findall(mod.read_text()))
    return found


def _part_stem(fn: str) -> str:
    """`..._lxc_pol_part17_v3rho...` -> `part17` (the source file's stem)."""
    m = re.search(r"_(part\d+)_", fn)
    if not m:
        raise UnsupportedKernel(f"cannot derive a part stem from `{fn}`")
    return m.group(1)


def _collect_output_dir(src_dir: Path, rel: str, *, inline_shards: bool = False
                        ) -> tuple[dict[str, str], list[str]]:
    """Convert one split-output directory to {relpath: text}.

    Flat `partN.rs` files transform 1:1; `partN/` chunk/meta fan-out
    directories are flattened to a single `partN.rs` (see flatten.py) — the
    fan-out only ever existed to bound cubecl-macros memory, which plain
    rustc does not need. Chunk-first struct-interface outputs are the one
    exception and are copied verbatim.
    """
    if _has_struct_interface(src_dir):
        return _collect_verbatim(src_dir, rel)

    files = sorted(p for p in src_dir.glob("*.rs"))
    subdirs = sorted(p for p in src_dir.iterdir() if p.is_dir())
    if not any(p.name == "mod.rs" for p in files):
        return {}, [f"{src_dir.name}/: no mod.rs"]

    out: dict[str, str] = {}
    errors: list[str] = []
    for p in files:
        if re.fullmatch(r"chunk\d+\.rs", p.name):
            errors.append(
                f"{src_dir.name}/{p.name}: tuple-return chunks directly under "
                "an output dir are not expected; teach _collect_output_dir "
                "to flatten the whole dir if this shape appears"
            )
            continue
        try:
            out[f"{rel}/{p.name}"] = transform_kernel(p.read_text())
        except UnsupportedKernel as exc:
            errors.append(f"{src_dir.name}/{p.name}: {exc}")
    for sd in subdirs:
        try:
            out[f"{rel}/{sd.name}.rs"] = flatten_helper_dir(sd, transform_kernel)
        except UnsupportedKernel as exc:
            errors.append(f"{src_dir.name}/{sd.name}/: {exc}")

    if inline_shards:
        shard, errs = _collect_shard_parts(src_dir, src_dir.name)
        errors.extend(errs)
        for name, text in shard.items():
            out[f"{rel}/{name}"] = text

    if errors:
        return {}, errors
    return out, []


def translate_functional(family: str, func: str, *, dry_run: bool = False,
                         cap: int = 0, inline_shards: bool = False,
                         seg_target: int = 0,
                         max_width: int = 1200) -> dict:
    src_dir = SRC_ROOT / family / func / "src"
    if not src_dir.is_dir():
        raise FileNotFoundError(src_dir)

    dst_dir = DST_ROOT / family / func / "src"
    modules: list[str] = []
    skipped: list[str] = []
    emitted: dict[str, str] = {}  # relpath under src/ -> text

    flat = sorted(p for p in src_dir.glob("*.rs") if p.name != "lib.rs")
    nested = sorted(p for p in src_dir.iterdir() if p.is_dir())

    for path in flat:
        text = path.read_text()
        try:
            out = transform_kernel(text)
        except UnsupportedKernel as exc:
            skipped.append(f"{path.name}: {exc}")
            continue
        modules.append(path.stem)
        emitted[path.name] = out

    # Split outputs nested up to four levels in the CubeCL tree — the emitter
    # fanned large outputs out to work around cubecl-macros memory blowup:
    #
    #   src/<output>.rs                              flat
    #   src/<output>/{mod,partN}.rs                  split
    #   src/<output>/partN/{mod,chunkK}.rs           chunked (up to ~1240 chunks)
    #   src/<output>/partN/metaM/{mod,chunkK}.rs     meta-wrapped
    #
    # Plain rustc needs none of that, so the chunk/meta fan-out is flattened
    # back into its part function (see flatten.py) and the emitted tree is at
    # most `src/<output>/{mod,partN}.rs`. The one exception is the chunk-first
    # struct-interface outputs, which _collect_output_dir copies verbatim.
    for d in nested:
        sub, errs = _collect_output_dir(d, d.name, inline_shards=inline_shards)
        if errs:
            skipped.extend(f"{d.name}/{e}" for e in errs)
            continue
        modules.append(d.name)
        emitted.update(sub)

    # Value-number each split output into a single function (see vnmerge.py).
    # maple2c re-derived every shared intermediate in each part, so the parts
    # of one output recompute the same values 2-16x over; merging computes
    # each distinct value once. An output the merge does not cover stays in
    # its split `<output>/{mod,partN}.rs` form, which is still correct.
    unmerged: list[str] = []
    for m in list(modules):
        group = {
            k.split("/", 1)[1]: v for k, v in emitted.items()
            if k.startswith(f"{m}/")
        }
        if not group:
            continue
        try:
            if any("/" in k for k in group):
                raise MergeError(f"{m}: nested directories survived flattening")
            text, _stats = merge_texts(m, group, cap=cap,
                                       seg_target=seg_target,
                                       max_width=max_width)
        except MergeError as exc:
            unmerged.append(str(exc))
            continue
        for k in [k for k in emitted if k.startswith(f"{m}/")]:
            del emitted[k]
        emitted[f"{m}.rs"] = text

    written = len(emitted)
    if not dry_run and modules:
        all_text = "\n".join(emitted.values())
        needs_libm = "libm::" in all_text
        siblings = {
            n for n in re.findall(r"libxc_rkernel_([A-Za-z0-9_]+)", all_text)
            if n != "math" and n != func
        }
        # Create the crate dir first: a regen that follows an --inline-shards
        # run finds the shard crate dirs deleted, and Cargo.toml is written
        # before any src/ file.
        dst_dir.mkdir(parents=True, exist_ok=True)
        _write_if_changed(
            dst_dir.parent / "Cargo.toml",
            emit_cargo_toml(dst_dir.parent, func, needs_libm, siblings),
        )
        # Prefer transforming the source lib.rs over synthesising one: the
        # shard crates (mgga_c_tpssloc_p0 etc.) carry `pub use <mod>::<fn>;`
        # re-exports that their sibling's wrapper imports by name, and a
        # synthesised lib.rs silently drops them.
        src_lib = src_dir / "lib.rs"
        if src_lib.is_file():
            lib_text = transform_kernel(src_lib.read_text())
            declared = set(re.findall(r"^pub mod (\w+);", lib_text, re.M))
            missing = declared - set(modules)
            if missing:
                raise UnsupportedKernel(
                    f"lib.rs declares modules that were not emitted: {sorted(missing)}"
                )
            emitted["lib.rs"] = lib_text
        else:
            emitted["lib.rs"] = emit_lib_rs(func, modules)

        dst_dir.mkdir(parents=True, exist_ok=True)
        for rel, text in emitted.items():
            path = dst_dir / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            _write_if_changed(path, text)

        # Flattening replaces `partN/` fan-out directories with `partN.rs`,
        # so anything the previous layout left behind must go, or cargo
        # would still compile the stale chunk files alongside the flat part.
        # Skipped outputs leave their old files untouched: deleting on a
        # partial emit would destroy modules that last converted cleanly.
        if not skipped:
            keep = {dst_dir / rel for rel in emitted}
            for p in sorted(dst_dir.rglob("*.rs"), reverse=True):
                if p not in keep:
                    p.unlink()
            for p in sorted((d for d in dst_dir.rglob("*") if d.is_dir()), reverse=True):
                if not any(p.iterdir()):
                    p.rmdir()

    return {"func": func, "written": written, "skipped": skipped,
            "modules": modules, "unmerged": unmerged}


def main() -> int:
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--func")
    g.add_argument("--family", choices=FAMILIES)
    g.add_argument("--all", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--inline-shards", action="store_true",
                    help="pull `<func>_pN` companion-crate parts into their "
                         "parent so the merge can dedup across them. OFF by "
                         "default: it wins ~5x on operation count but loses "
                         "far more on build wall-clock, because 39 crates that "
                         "compiled in parallel collapse into 5 serial ones "
                         "(mgga_c_rmggac alone: 26s -> 959s).")
    ap.add_argument("--seg-target", type=int, default=0,
                    help="split a merged output into contiguous segment "
                         "functions of about this many definitions each, "
                         "threading cut-crossing values through a scratch "
                         "array. Unlike --cap this changes only the emitted "
                         "shape, not the value numbering, so the operation "
                         "count stays at the uncapped optimum while each "
                         "segment gets its own module and therefore its own "
                         "codegen unit.")
    ap.add_argument("--seg-max-width", type=int, default=1200,
                    help="refuse a segment cut with more than this many live "
                         "values; each costs a store and a reload per grid "
                         "point. An output with no cheap cut stays whole.")
    ap.add_argument("--cap", type=int, default=0,
                    help="max defs per merged function (0 = unlimited); "
                         "splits an output into several functions so they "
                         "can land in different codegen units")
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

    # Shard companion crates (`<func>_p0..pN`) are now inlined into their
    # parent by _collect_shard_parts, so emitting them would leave dead source
    # that nothing depends on. Drop them from the target list and delete any
    # tree a previous run left behind.
    shards = shard_crates() if args.inline_shards else set()
    dropped = [t for t in targets if t[1] in shards]
    targets = [t for t in targets if t[1] not in shards]
    if not args.dry_run:
        for fam, func in dropped:
            stale = DST_ROOT / fam / func
            if stale.is_dir():
                for p in sorted(stale.rglob("*"), reverse=True):
                    p.unlink() if p.is_file() else p.rmdir()
                stale.rmdir()
    if dropped:
        print(f"inlined {len(dropped)} shard crate(s) into their parents")

    total_written = 0
    total_skipped = 0
    total_unmerged = 0
    failures = 0
    for fam, func in targets:
        try:
            r = translate_functional(fam, func, dry_run=args.dry_run,
                                     cap=args.cap,
                                     inline_shards=args.inline_shards,
                                     seg_target=args.seg_target,
                                     max_width=args.seg_max_width)
        except Exception as exc:  # noqa: BLE001
            print(f"FAIL {fam}/{func}: {exc}", file=sys.stderr)
            failures += 1
            continue
        total_written += r["written"]
        total_skipped += len(r["skipped"])
        total_unmerged += len(r["unmerged"])
        if len(targets) == 1 or r["skipped"]:
            for s in r["skipped"]:
                print(f"  skip {fam}/{func}/{s}")
        if len(targets) == 1:
            for u in r["unmerged"]:
                print(f"  unmerged {fam}/{func}/{u}")
            print(f"{fam}/{func}: {r['written']} modules -> {DST_ROOT / fam / func}")

    print(
        f"\ntranslated {total_written} kernel modules across {len(targets)} "
        f"functional(s); {total_skipped} skipped, {failures} failed, "
        f"{total_unmerged} output(s) left unmerged"
    )
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
