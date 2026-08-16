#!/usr/bin/env python3
"""Generic POST-PROCESS sub-crate splitter for an oversized per-functional kernel.

Background (quick task 260520-eem, Option A of 260520-c91)
----------------------------------------------------------
The translator (tools/translate_v2/) emits one per-functional facade crate
``crates/kernels/<family>/<func>/`` whose nested-by-output ``src/<output>/``
layout (emit.py ``emit_output_dir``) can contain tens of thousands of ``.rs``
files for a single output (e.g. mgga_c_tpssloc/lxc_pol: 122 parts, the dense
4th-derivative parts each fanned out into thousands of meta/chunk files). One
rustc process holding parse + IR + monomorphization state across ~63K modules
of a SINGLE crate exceeds the 30 GB dev box (260520-c91 "Phase 2 result").

This tool is a PURE POST-PROCESS over an already-regenerated facade crate. It
does NOT call the translator and does NOT touch the proven hier emit path. It
bin-packs an output's part UNITS into N **shard** sub-crates (each compiling in
its own rustc process), MOVES the part units into the shards, and rewrites the
facade's output-wrapper imports so the facade's ``#[cube]`` body LINKS against
each shard's already-expanded part fn (cross-crate ``#[cube]`` linking, proven
in 260520-c91 verified_structural_fact 3). The facade keeps its exact package
name (D-10 public-interface invariant).

Design (locked, from 260520-eem PLAN):
  - Parts are the atomic shardable unit: a flat ``partN.rs`` FILE or a
    ``partN/`` DIRECTORY. NEVER split a part across shards.
  - Bin-packing weight = SIZE-AWARE (default): ``sum_file max(1,
    (lines/NORM)**EXP)``. Each file costs >= 1 (the original 260520-eem
    file-count weight — RSS scaled with module COUNT for the dense meta/chunk
    FAN-OUT regime), and a file LARGER than NORM lines is up-weighted
    super-linearly so a huge flat CSE-irreducible part (the D-LOCK-B regime —
    one big ``#[cube]`` fn drives per-process RSS ~lines^2.5, see memory
    project_compile_rss_model_chunk_sizing) SELF-ISOLATES instead of OOM-ing the
    shard it was packed into. ``--weight-mode files`` restores the legacy count.
  - CONTIGUOUS part ranges per shard (keeps the facade ``use`` list readable).
    A single part that alone exceeds budget gets its own solo shard.
  - Shard naming is a pure function of (part order, file counts, budget) so
    re-runs are byte-identical (D-LOCK-D determinism).
  - Workspace membership is by a ROOT [dependencies] path-dep (not an explicit
    members list); shards are intentionally absent from default-members.

Usage:
    python3 tools/split_per_functional_subcrate.py <family> <func> <output> \\
        --budget <weight-per-shard> [--weight-mode size|files] [--dry-run]
    python3 tools/split_per_functional_subcrate.py --selftest

Example:
    python3 tools/split_per_functional_subcrate.py \\
        mgga mgga_c_tpssloc lxc_pol --budget 10000 --dry-run
"""

import argparse
import os
import re
import shutil
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
# Repo `crates/kernels` root. Overridable for the selftest via set_kernels_root.
KERNELS_ROOT = REPO_ROOT / "crates" / "kernels"
# Manifest that carries the per-kernel path-deps. Post-Phase-10 this is
# crates/libxc-eval/Cargo.toml (optional deps + oracle-<family> features);
# rewrite_root_cargo also still understands the legacy root-style dep lines.
# Overridable for the selftest.
WORKSPACE_CARGO = REPO_ROOT / "crates" / "libxc-eval" / "Cargo.toml"

# cubecl dependency line — copied verbatim from emit.py (CUBECL_DEP), so the
# shard manifests match the proven in-tree per-functional manifest shape.
CUBECL_DEP = 'cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }'

# Shard crate-level allow-list — mirror emit.py CRATE_ALLOW so the moved part
# trees (which already carry file-level allows) compile cleanly.
CRATE_ALLOW = (
    "#![allow(unused_imports, unused_variables, non_snake_case, "
    "clippy::excessive_precision, clippy::too_many_arguments, "
    "clippy::needless_return)]"
)

_PART_RE = re.compile(r"^part(\d+)$")
_PUBFN_RE = re.compile(r"pub fn (\w+)\s*\(")
_MODPART_RE = re.compile(r"^\s*mod part\d+\s*;\s*$")


# --- root overrides (selftest) ----------------------------------------------
def set_kernels_root(path) -> None:
    """Override the crates/kernels root (used by the selftest to act on a tempdir)."""
    global KERNELS_ROOT
    KERNELS_ROOT = Path(path)


def set_workspace_cargo(path) -> None:
    """Override the workspace root Cargo.toml (used by the selftest)."""
    global WORKSPACE_CARGO
    WORKSPACE_CARGO = Path(path)


# --- deterministic write -----------------------------------------------------
def _write(path: Path, content: str) -> None:
    """Deterministic write (mirrors emit.py._write): create parents, strip
    per-line trailing whitespace, guarantee exactly one trailing newline. This
    is the idempotency contract surface (D-LOCK-D)."""
    path.parent.mkdir(parents=True, exist_ok=True)
    body = content.rstrip("\n") + "\n"
    body = "\n".join(line.rstrip() for line in body.split("\n"))
    path.write_text(body, encoding="utf-8")


# --- path helpers ------------------------------------------------------------
def facade_dir(family: str, func: str) -> Path:
    return KERNELS_ROOT / family / func


def output_dir(family: str, func: str, output: str) -> Path:
    return facade_dir(family, func) / "src" / output


def shard_name(func: str, k: int) -> str:
    """Shard short name, e.g. ``mgga_c_tpssloc_p0`` (contiguous-range index)."""
    return f"{func}_p{k}"


def shard_pkg(func: str, k: int) -> str:
    """Cargo package name, e.g. ``libxc-kernel-mgga_c_tpssloc_p0``."""
    return f"libxc-kernel-{shard_name(func, k)}"


def shard_ident(func: str, k: int) -> str:
    """Rust crate identifier, e.g. ``libxc_kernel_mgga_c_tpssloc_p0``."""
    return f"libxc_kernel_{shard_name(func, k)}".replace("-", "_")


def shard_dir(family: str, func: str, k: int) -> Path:
    return KERNELS_ROOT / family / shard_name(func, k)


# --- scanning ----------------------------------------------------------------
# Size-weight defaults. A `.rs` file of <= NORM lines costs 1 weight unit (so
# weight == file count for normal-sized files — backward-compatible with the
# legacy file-count budget and the meta/fan-out regime where RSS scaled with
# module COUNT). A file LARGER than NORM additionally costs (lines/NORM)**EXP, a
# super-linear penalty for the flat D-LOCK-B regime where ONE huge `#[cube]` fn
# drives per-process rustc RSS super-linearly (measured ~lines^2.5; see memory
# project_compile_rss_model_chunk_sizing). NORM ~= the per-cube-fn line cap.
DEFAULT_WEIGHT_NORM = 2500
DEFAULT_WEIGHT_EXP = 2.0


def _rs_count(path: Path) -> int:
    """A flat ``partN.rs`` is 1; a ``partN/`` dir is its recursive ``*.rs`` file
    count. (Kept for reporting; bin-packing now uses :func:`_part_weight`.)"""
    if path.is_file():
        return 1
    return sum(1 for _ in path.rglob("*.rs"))


def _line_count(path: Path) -> int:
    try:
        return path.read_text(encoding="utf-8").count("\n") + 1
    except OSError:
        return 0


def _part_weight(path: Path, norm: int, exp: float) -> int:
    """Size-aware shard weight: ``sum_file max(1, round((lines/norm)**exp))``
    over the part's recursive ``*.rs`` files. Each file costs >= 1 (legacy
    file-count floor); a file over ``norm`` lines is up-weighted super-linearly
    so a huge flat (CSE-irreducible) part self-isolates into its own shard
    rather than OOM-ing a shard it was packed into. Deterministic: a pure
    function of file line counts + (norm, exp)."""
    files = [path] if path.is_file() else sorted(path.rglob("*.rs"))
    return sum(max(1, int(round((_line_count(f) / norm) ** exp))) for f in files)


def _part_fn_name(unit_path: Path) -> str:
    """Discover the part's exported ``pub fn`` via regex (same approach as
    per_functional.py:359). For a dir unit the source is its ``mod.rs``."""
    src = unit_path if unit_path.is_file() else unit_path / "mod.rs"
    text = src.read_text(encoding="utf-8")
    m = _PUBFN_RE.search(text)
    if not m:
        raise ValueError(f"no `pub fn` found in part unit {src}")
    return m.group(1)


def scan_parts(out_dir: Path, norm: int = DEFAULT_WEIGHT_NORM,
               exp: float = DEFAULT_WEIGHT_EXP):
    """Enumerate part UNITS in ``out_dir`` ordered by part index.

    Returns a list of dicts ordered by part index:
      {"idx": int, "path": Path, "is_dir": bool, "files": int, "fn": str}
    Asserts no flat-file / dir collision for the same index (E0761 hazard).
    """
    by_idx = {}
    for entry in sorted(os.listdir(out_dir)):
        full = out_dir / entry
        if full.is_dir():
            m = _PART_RE.match(entry)
            if not m:
                continue
            idx = int(m.group(1))
        elif entry.endswith(".rs") and entry != "mod.rs":
            m = _PART_RE.match(entry[:-3])
            if not m:
                continue
            idx = int(m.group(1))
        else:
            continue
        if idx in by_idx:
            raise ValueError(
                f"part{idx} present as BOTH a file and a directory in {out_dir} "
                "(E0761 collision) — clean the stale form before splitting")
        by_idx[idx] = full

    parts = []
    for idx in sorted(by_idx):
        path = by_idx[idx]
        parts.append({
            "idx": idx,
            "path": path,
            "is_dir": path.is_dir(),
            "files": _rs_count(path),
            "weight": _part_weight(path, norm, exp),
            "fn": _part_fn_name(path),
        })
    return parts


# --- bin-packing -------------------------------------------------------------
def bin_pack(parts, budget: int, weight_key: str = "weight"):
    """Pack contiguous part ranges into shards under ``budget`` per shard.

    ``weight_key`` selects the unit weight: ``"weight"`` (size-aware, default —
    a huge flat part self-isolates) or ``"files"`` (legacy recursive file count).
    Walk parts in index order; start a new shard when adding the next unit would
    exceed budget. A unit that alone exceeds budget gets its OWN solo shard
    (a part is atomic — NEVER split it across shards). Pure function of
    (part order, weights, budget) -> deterministic (D-LOCK-D).

    Returns a list of shards; each shard is a list of part dicts (in order).
    """
    shards = []
    current = []
    current_files = 0
    for p in parts:
        w = p[weight_key]
        if not current:
            current = [p]
            current_files = w
            continue
        # Solo-shard rule: if the running shard plus this unit would exceed
        # budget, close the current shard and start a fresh one with this unit.
        if current_files + w > budget:
            shards.append(current)
            current = [p]
            current_files = w
        else:
            current.append(p)
            current_files += w
    if current:
        shards.append(current)
    return shards


# --- shard materialization ---------------------------------------------------
def make_shard_cargo_toml(func: str, k: int) -> str:
    return "\n".join([
        "[package]",
        f'name = "{shard_pkg(func, k)}"',
        'version = "0.1.0"',
        'edition = "2024"',
        "",
        "[dependencies]",
        CUBECL_DEP,
        'libxc-kernel-math = { path = "../../math" }',
    ])


def make_shard_lib_rs(func: str, k: int, output: str, shard_parts) -> str:
    """``src/lib.rs`` for a shard: crate-level allow, ``pub mod <output>;``, then
    one ``pub use <output>::<partfn>;`` per owned part (stable crate-root path)."""
    lines = [
        f"//! {func.upper()} shard sub-crate {shard_name(func, k)} "
        f"(parts {shard_parts[0]['idx']}..={shard_parts[-1]['idx']}).",
        "//!",
        "//! Auto-generated by tools/split_per_functional_subcrate.py — "
        "do not hand-edit.",
        "",
        CRATE_ALLOW,
        "",
        f"pub mod {output};",
        "",
    ]
    for p in shard_parts:
        lines.append(f"pub use {output}::{p['fn']};")
    return "\n".join(lines)


def make_shard_output_mod_rs(func: str, k: int, output: str, shard_parts) -> str:
    """``src/<output>/mod.rs`` for a shard: a THIN index — ``pub mod partN;`` and
    ``pub use partN::<fn>;`` per owned part. NO #[cube] wrapper (the wrapper
    stays in the facade)."""
    lines = [
        f"//! {func.upper()} {output} shard {shard_name(func, k)} — thin module "
        "index (no cube wrapper; the wrapper lives in the facade).",
        f"//! Parts {shard_parts[0]['idx']}..={shard_parts[-1]['idx']}.",
        "",
    ]
    for p in shard_parts:
        lines.append(f"pub mod part{p['idx']};")
    lines.append("")
    for p in shard_parts:
        lines.append(f"pub use part{p['idx']}::{p['fn']};")
    return "\n".join(lines)


def materialize_shard(family: str, func: str, output: str, k: int, shard_parts):
    """Create the shard crate skeleton and MOVE its owned part units in."""
    sd = shard_dir(family, func, k)
    _write(sd / "Cargo.toml", make_shard_cargo_toml(func, k))
    _write(sd / "src" / "lib.rs",
           make_shard_lib_rs(func, k, output, shard_parts))
    _write(sd / "src" / output / "mod.rs",
           make_shard_output_mod_rs(func, k, output, shard_parts))
    dst_out = sd / "src" / output
    for p in shard_parts:
        src = p["path"]
        dst = dst_out / src.name
        # Re-split over a pre-existing shard tree: shutil.move into an existing
        # directory NESTS the source inside it (partN/partN/), leaving stray
        # duplicate trees. Replace the destination instead.
        if dst.is_dir():
            shutil.rmtree(dst)
        elif dst.exists():
            dst.unlink()
        # shutil.move (not copy) so the facade carries no duplicate part trees.
        shutil.move(str(src), str(dst))


# --- facade rewrite ----------------------------------------------------------
def rewrite_facade_output_mod(out_mod_path: Path, parts, part_to_shard_ident):
    """Rewrite the facade ``<output>/mod.rs`` (verified_structural_fact 9):
      - DROP every ``mod partN;`` line.
      - REPLACE each ``use partN::{pfn};`` with ``use <shard_ident>::{pfn};``.
      - LEAVE the ``#[cube] pub fn`` header, ALL call statements, ``}``, and the
        math/cubecl ``use`` lines VERBATIM.
    """
    text = out_mod_path.read_text(encoding="utf-8")
    lines = text.split("\n")
    # Map part index -> owning shard ident, for the `use partN::...` rewrite.
    idx_to_ident = {p["idx"]: part_to_shard_ident[p["idx"]] for p in parts}

    use_re = re.compile(r"^(\s*)use part(\d+)::(.+;)\s*$")
    out_lines = []
    for ln in lines:
        if _MODPART_RE.match(ln):
            # DROP `mod partN;` declarations.
            continue
        m = use_re.match(ln)
        if m:
            indent, idx_s, tail = m.group(1), m.group(2), m.group(3)
            ident = idx_to_ident[int(idx_s)]
            out_lines.append(f"{indent}use {ident}::{tail}")
            continue
        out_lines.append(ln)
    _write(out_mod_path, "\n".join(out_lines))


def rewrite_facade_cargo(family: str, func: str, n_shards: int):
    """Add one ``[dependencies]`` line per shard to the facade Cargo.toml.
    Idempotent: a shard line already present is not duplicated."""
    cargo = facade_dir(family, func) / "Cargo.toml"
    text = cargo.read_text(encoding="utf-8")
    lines = text.split("\n")
    additions = []
    for k in range(n_shards):
        dep = f'{shard_pkg(func, k)} = {{ path = "../{shard_name(func, k)}" }}'
        if dep not in text:
            additions.append(dep)
    if not additions:
        return
    # Append shard deps to the end of the [dependencies] block. The facade
    # manifest is a fixed 8-line shape (emit.py): [dependencies] is the last
    # section, so appending at EOF keeps the deps under it.
    while lines and lines[-1].strip() == "":
        lines.pop()
    lines.extend(additions)
    _write(cargo, "\n".join(lines))


def rewrite_root_cargo(family: str, func: str, n_shards: int):
    """Add one ``[dependencies]`` path-dep per shard, inserted immediately
    after the facade's existing path-dep line. Makes each shard a workspace
    member (verified_structural_fact 7); shards are NOT added to default-members.
    Idempotent: a shard line already present is not re-inserted.

    Post-Phase-10, kernel workspace membership lives in
    ``crates/libxc-eval/Cargo.toml`` as ``optional = true`` path-deps (relative
    ``../kernels/...`` paths) gated behind the family's ``oracle-<family>``
    feature. Both manifest dialects are recognized: the eval-style optional dep
    (preferred) and the legacy root-style plain dep (kept for the selftest and
    any pre-Phase-10 checkout). Shard dep lines mirror whichever facade line is
    found; when the manifest also carries a ``"dep:libxc-kernel-<func>",``
    feature entry, matching ``dep:`` entries are inserted for the shards."""
    text = WORKSPACE_CARGO.read_text(encoding="utf-8")
    lines = text.split("\n")
    eval_facade_dep = (f'libxc-kernel-{func} = '
                       f'{{ path = "../kernels/{family}/{func}", optional = true }}')
    legacy_facade_dep = (f'libxc-kernel-{func} = '
                         f'{{ path = "crates/kernels/{family}/{func}" }}')
    anchor = None
    eval_style = None
    for i, ln in enumerate(lines):
        if ln.strip() == eval_facade_dep:
            anchor, eval_style = i, True
            break
        if ln.strip() == legacy_facade_dep:
            anchor, eval_style = i, False
            break
    if anchor is None:
        raise ValueError(
            f"facade path-dep not found in {WORKSPACE_CARGO}: "
            f"{eval_facade_dep!r} (eval-style) or {legacy_facade_dep!r} (legacy)")
    new_deps = []
    for k in range(n_shards):
        if eval_style:
            dep = (f'{shard_pkg(func, k)} = '
                   f'{{ path = "../kernels/{family}/{shard_name(func, k)}", '
                   f'optional = true }}')
        else:
            dep = (f'{shard_pkg(func, k)} = '
                   f'{{ path = "crates/kernels/{family}/{shard_name(func, k)}" }}')
        if dep not in text:
            new_deps.append(dep)
    if new_deps:
        lines[anchor + 1:anchor + 1] = new_deps

    # Feature gating (eval-style manifests): mirror the facade's
    # `"dep:libxc-kernel-<func>",` entry with one per shard.
    feat_anchor = None
    facade_feat = f'"dep:libxc-kernel-{func}",'
    for i, ln in enumerate(lines):
        if ln.strip() == facade_feat:
            feat_anchor = i
            break
    new_feats = []
    if feat_anchor is not None:
        indent = lines[feat_anchor][:len(lines[feat_anchor])
                                    - len(lines[feat_anchor].lstrip())]
        for k in range(n_shards):
            feat = f'{indent}"dep:{shard_pkg(func, k)}",'
            if feat.strip() not in {ln.strip() for ln in lines}:
                new_feats.append(feat)
        if new_feats:
            lines[feat_anchor + 1:feat_anchor + 1] = new_feats

    if new_deps or new_feats:
        _write(WORKSPACE_CARGO, "\n".join(lines))


# --- idempotency guard -------------------------------------------------------
def _already_split(out_mod_path: Path) -> bool:
    """True if the facade output ``mod.rs`` already has zero ``mod partN;``
    lines (treat as already-split)."""
    text = out_mod_path.read_text(encoding="utf-8")
    return not any(_MODPART_RE.match(ln) for ln in text.split("\n"))


# --- dry-run plan ------------------------------------------------------------
def plan_summary(func: str, shards) -> str:
    if not shards:
        return f"Shard plan for {func}: no part units found (nothing to shard)."
    out = [f"Shard plan for {func} ({len(shards)} shards):"]
    longest_k, longest_w = 0, -1
    for k, sp in enumerate(shards):
        files = sum(p["files"] for p in sp)
        weight = sum(p["weight"] for p in sp)
        if weight > longest_w:
            longest_k, longest_w = k, weight
        out.append(
            f"  {shard_name(func, k)}: parts {sp[0]['idx']}..={sp[-1]['idx']} "
            f"({len(sp)} parts, {files} files, weight {weight})")
    out.append(
        f"Largest shard: {shard_name(func, longest_k)} "
        f"(weight {longest_w}) — the compile-RSS canary.")
    return "\n".join(out)


# --- main split orchestration ------------------------------------------------
def run_split(family: str, func: str, output: str, budget: int,
              dry_run: bool, weight_mode: str = "size",
              norm: int = DEFAULT_WEIGHT_NORM,
              exp: float = DEFAULT_WEIGHT_EXP) -> int:
    out_dir = output_dir(family, func, output)
    if not out_dir.is_dir():
        print(f"ERROR: output dir not found: {out_dir}", file=sys.stderr)
        return 1
    out_mod = out_dir / "mod.rs"
    if not out_mod.is_file():
        print(f"ERROR: output wrapper not found: {out_mod}", file=sys.stderr)
        return 1

    # Idempotency guard: already-split facade -> no-op (do NOT double-move).
    if _already_split(out_mod):
        print(f"NOTICE: {func}/{output} already split "
              "(facade mod.rs has zero `mod partN;` lines) — nothing to do.")
        return 0

    parts = scan_parts(out_dir, norm, exp)
    if not parts:
        print(f"ERROR: no part units found in {out_dir}", file=sys.stderr)
        return 1
    shards = bin_pack(parts, budget, "weight" if weight_mode == "size" else "files")

    print(plan_summary(func, shards))

    if dry_run:
        print("\nDry run — no filesystem changes made.")
        return 0

    # part index -> owning shard ident (computed before any move).
    part_to_shard_ident = {}
    for k, sp in enumerate(shards):
        ident = shard_ident(func, k)
        for p in sp:
            part_to_shard_ident[p["idx"]] = ident

    # Materialize shards (MOVE part units in).
    for k, sp in enumerate(shards):
        materialize_shard(family, func, output, k, sp)

    # Rewrite the facade output wrapper imports (drop mod, re-source use).
    rewrite_facade_output_mod(out_mod, parts, part_to_shard_ident)
    # Patch the facade + root manifests.
    rewrite_facade_cargo(family, func, len(shards))
    rewrite_root_cargo(family, func, len(shards))

    print(f"\nDone. Sharded {func}/{output}: {len(parts)} parts -> "
          f"{len(shards)} shards. Facade keeps name libxc-kernel-{func}.")
    return 0


# --- selftest ----------------------------------------------------------------
def _selftest() -> int:
    """Build a tiny synthetic facade in a tempdir (an output with a few flat
    parts + one dir part), run the split with a small budget, and assert the
    locked invariants. Exit 0 on success, 1 on failure."""
    import tempfile

    tmp = Path(tempfile.mkdtemp(prefix="split_pf_selftest_"))
    ok = True
    fam, func, output = "mgga", "mgga_x_selftest", "lxc_pol"

    # --- synthetic facade ----------------------------------------------------
    def build_facade(root_kernels: Path, ws_cargo: Path):
        out = root_kernels / fam / func / "src" / output
        # 5 part units: part0..part3 flat, part4 a dir with 3 chunk files.
        flat_idxs = [0, 1, 2, 3]
        part_fns = {
            i: f"mgga_x_selftest_lxc_pol_part{i}_out{i}" for i in range(5)
        }
        for i in flat_idxs:
            _write(out / f"part{i}.rs",
                   f"//! synthetic flat part {i}.\n"
                   "use cubecl::prelude::*;\n"
                   "#[cube]\n"
                   f"pub fn {part_fns[i]}(rho: &Array<f64>, "
                   "o: &mut Array<f64>) {{}}\n")
        # part4 is a directory unit (mod.rs + 3 chunks) -> weight 4.
        p4 = out / "part4"
        _write(p4 / "mod.rs",
               "//! synthetic dir part 4.\n"
               "mod chunk0;\nmod chunk1;\nmod chunk2;\n"
               "use cubecl::prelude::*;\n"
               "#[cube]\n"
               f"pub fn {part_fns[4]}(rho: &Array<f64>, o: &mut Array<f64>) {{}}\n")
        for c in range(3):
            _write(p4 / f"chunk{c}.rs",
                   f"//! chunk {c}.\n#[cube]\nfn c{c}<F: Float>(x: F) -> F {{ x }}\n")
        # facade output wrapper mod.rs — the rewrite target.
        wrapper = [
            f"//! {func.upper()} {output} (nested-by-output, 5 parts).",
            CRATE_ALLOW,
            "",
        ]
        for i in range(5):
            wrapper.append(f"mod part{i};")
        wrapper += [
            "",
            "use cubecl::prelude::*;",
            "use libxc_kernel_math::constants::M_PI;",
            "",
        ]
        for i in range(5):
            wrapper.append(f"use part{i}::{part_fns[i]};")
        wrapper += [
            "",
            "#[allow(unused_variables, non_snake_case)]",
            "#[cube]",
            f"pub fn {func}_{output}(rho: &Array<f64>, o: &mut Array<f64>) {{",
        ]
        for i in range(5):
            wrapper.append(f"    {part_fns[i]}(rho, o);")
        wrapper.append("}")
        _write(out / "mod.rs", "\n".join(wrapper))
        # facade lib.rs + Cargo.toml
        _write(root_kernels / fam / func / "src" / "lib.rs",
               f"{CRATE_ALLOW}\npub mod {output};\n")
        _write(root_kernels / fam / func / "Cargo.toml",
               "[package]\n"
               f'name = "libxc-kernel-{func}"\n'
               'version = "0.1.0"\n'
               'edition = "2024"\n'
               "\n[dependencies]\n"
               f"{CUBECL_DEP}\n"
               'libxc-kernel-math = { path = "../../math" }\n')
        # synthetic workspace root Cargo.toml with the facade path-dep.
        _write(ws_cargo,
               "[workspace]\n"
               'members = ["xtask"]\n'
               "\n[dependencies]\n"
               "libxc-kernel-something = { path = \"crates/kernels/mgga/something\" }\n"
               f'libxc-kernel-{func} = '
               f'{{ path = "crates/kernels/{fam}/{func}" }}\n'
               "libxc-kernel-other = { path = \"crates/kernels/mgga/other\" }\n")

    def snapshot(root: Path):
        snap = {}
        for p in sorted(root.rglob("*")):
            if p.is_file():
                snap[str(p.relative_to(root))] = p.read_text(encoding="utf-8")
        return snap

    try:
        kroot = tmp / "crates" / "kernels"
        ws = tmp / "Cargo.toml"
        build_facade(kroot, ws)
        set_kernels_root(kroot)
        set_workspace_cargo(ws)

        out_dir = output_dir(fam, func, output)
        out_mod = out_dir / "mod.rs"
        wrapper_before = out_mod.read_text(encoding="utf-8")
        # Capture the #[cube] call body (header through `}`) for the
        # "body unchanged" assertion.
        body_before = wrapper_before[wrapper_before.index("#[allow(unused_variables"):]

        # Budget 3: part0..part2 (weight 1 each) fill shard p0 exactly; part3
        # starts p1; part4 (weight 4 > budget) becomes its own solo shard p2.
        rc = run_split(fam, func, output, budget=3, dry_run=False)
        if rc != 0:
            print(f"SELFTEST FAIL: run_split returned {rc}")
            ok = False

        # (a) every part unit lands in exactly one shard.
        shard_dirs = sorted(
            d for d in (kroot / fam).iterdir()
            if d.is_dir() and re.match(rf"{func}_p\d+$", d.name))
        seen = {}
        for sd in shard_dirs:
            sout = sd / "src" / output
            for entry in sout.iterdir():
                m = _PART_RE.match(entry.name[:-3] if entry.name.endswith(".rs")
                                   else entry.name)
                if m:
                    idx = int(m.group(1))
                    seen.setdefault(idx, []).append(sd.name)
        if sorted(seen) != [0, 1, 2, 3, 4]:
            print(f"SELFTEST FAIL: parts distributed = {sorted(seen)}, "
                  "expected 0..4")
            ok = False
        dupes = {i: s for i, s in seen.items() if len(s) > 1}
        if dupes:
            print(f"SELFTEST FAIL: parts in multiple shards: {dupes}")
            ok = False
        # part4 (the dir unit, weight 4 > budget 3) must be a solo shard.
        if len(shard_dirs) < 2:
            print(f"SELFTEST FAIL: expected >1 shard, got {len(shard_dirs)}")
            ok = False
        # the dir part must still be a directory in its shard.
        p4_shard = seen.get(4, [None])[0]
        if p4_shard:
            p4_dir = kroot / fam / p4_shard / "src" / output / "part4"
            if not p4_dir.is_dir():
                print("SELFTEST FAIL: dir part4 did not land as a directory")
                ok = False
            if not (p4_dir / "chunk0.rs").is_file():
                print("SELFTEST FAIL: part4 chunk files not moved with the dir")
                ok = False

        # (b) facade mod.rs has zero `mod partN;` and use lines point at shards.
        wrapper_after = out_mod.read_text(encoding="utf-8")
        if any(_MODPART_RE.match(ln) for ln in wrapper_after.split("\n")):
            print("SELFTEST FAIL: facade mod.rs still has `mod partN;` lines")
            ok = False
        n_shard_use = wrapper_after.count(f"use libxc_kernel_{func}_p")
        if n_shard_use != 5:
            print(f"SELFTEST FAIL: expected 5 shard-sourced use lines, "
                  f"got {n_shard_use}")
            ok = False
        if "use part0::" in wrapper_after:
            print("SELFTEST FAIL: a `use partN::` line survived the rewrite")
            ok = False
        # the wrapper's own math/cubecl imports survive.
        if "use cubecl::prelude::*;" not in wrapper_after or \
                "use libxc_kernel_math::constants::M_PI;" not in wrapper_after:
            print("SELFTEST FAIL: facade math/cubecl imports were dropped")
            ok = False

        # (c) facade #[cube] call body is unchanged (header..`}` verbatim).
        body_after = wrapper_after[wrapper_after.index("#[allow(unused_variables"):]
        if body_after.rstrip("\n") != body_before.rstrip("\n"):
            print("SELFTEST FAIL: facade #[cube] call body changed")
            ok = False

        # (d) shard lib.rs re-exports resolve (string check).
        for sd in shard_dirs:
            lib = (sd / "src" / "lib.rs").read_text(encoding="utf-8")
            if f"pub mod {output};" not in lib:
                print(f"SELFTEST FAIL: {sd.name}/lib.rs missing pub mod {output}")
                ok = False
            smod = (sd / "src" / output / "mod.rs").read_text(encoding="utf-8")
            # each owned part must be re-exported in both lib.rs and the
            # output mod.rs index.
            for entry in (sd / "src" / output).iterdir():
                m = _PART_RE.match(entry.name[:-3] if entry.name.endswith(".rs")
                                   else entry.name)
                if not m:
                    continue
                idx = int(m.group(1))
                fn = f"mgga_x_selftest_lxc_pol_part{idx}_out{idx}"
                if f"pub use {output}::{fn};" not in lib:
                    print(f"SELFTEST FAIL: {sd.name}/lib.rs missing re-export {fn}")
                    ok = False
                if f"pub mod part{idx};" not in smod:
                    print(f"SELFTEST FAIL: {sd.name}/{output}/mod.rs missing "
                          f"pub mod part{idx}")
                    ok = False
            # An actual cube wrapper would be an attribute line `#[cube]`, not a
            # prose mention in a doc-comment — check non-comment lines only.
            has_cube_attr = any(
                ln.strip().startswith("#[cube")
                for ln in smod.split("\n"))
            if has_cube_attr:
                print(f"SELFTEST FAIL: shard {sd.name} output mod.rs has a "
                      "cube wrapper attribute (should be a thin index)")
                ok = False

        # facade + root Cargo.toml gained one dep line per shard.
        facade_cargo = (kroot / fam / func / "Cargo.toml").read_text()
        root_cargo = ws.read_text()
        for k in range(len(shard_dirs)):
            if shard_pkg(func, k) not in facade_cargo:
                print(f"SELFTEST FAIL: facade Cargo.toml missing {shard_pkg(func, k)}")
                ok = False
            if shard_pkg(func, k) not in root_cargo:
                print(f"SELFTEST FAIL: root Cargo.toml missing {shard_pkg(func, k)}")
                ok = False
        if "default-members" in root_cargo:
            print("SELFTEST FAIL: synthetic root unexpectedly mentions "
                  "default-members")
            ok = False

        # (e) double-run is byte-identical (idempotency guard short-circuits).
        snap1 = snapshot(tmp)
        rc2 = run_split(fam, func, output, budget=3, dry_run=False)
        if rc2 != 0:
            print(f"SELFTEST FAIL: second run_split returned {rc2}")
            ok = False
        snap2 = snapshot(tmp)
        if snap1 != snap2:
            diff = sorted(set(snap1) ^ set(snap2)) or [
                k for k in snap1 if snap1.get(k) != snap2.get(k)]
            print(f"SELFTEST FAIL: double-run not byte-identical: {diff}")
            ok = False
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    if ok:
        print("SELFTEST PASS: synthetic facade sharded; each part in exactly "
              "one shard; facade mod.rs re-sourced (zero `mod partN;`); "
              "#[cube] body unchanged; shard re-exports resolve; "
              "double-run byte-identical.")
        return 0
    return 1


# --- CLI ---------------------------------------------------------------------
def main() -> int:
    if "--selftest" in sys.argv:
        return _selftest()

    ap = argparse.ArgumentParser(
        description="Post-process sub-crate splitter for an oversized "
                    "per-functional kernel output.")
    ap.add_argument("family", help="kernel family (e.g. mgga)")
    ap.add_argument("func", help="functional id (e.g. mgga_c_tpssloc)")
    ap.add_argument("output", help="output module to shard (e.g. lxc_pol)")
    ap.add_argument("--budget", type=int, required=True,
                    help="max shard WEIGHT per shard. In the default 'size' "
                         "mode weight == file count for normal files but a huge "
                         "flat part counts (lines/--weight-norm)**--weight-exp "
                         "files; in 'files' mode it is the raw recursive *.rs "
                         "count.")
    ap.add_argument("--weight-mode", choices=("size", "files"), default="size",
                    help="shard weight metric: 'size' (per-file, super-linear "
                         "over --weight-norm; default) or 'files' (legacy).")
    ap.add_argument("--weight-norm", type=int, default=DEFAULT_WEIGHT_NORM,
                    help=f"lines at which a file costs 1 weight unit "
                         f"(size mode; default {DEFAULT_WEIGHT_NORM}).")
    ap.add_argument("--weight-exp", type=float, default=DEFAULT_WEIGHT_EXP,
                    help=f"super-linear exponent for files over --weight-norm "
                         f"(size mode; default {DEFAULT_WEIGHT_EXP}).")
    ap.add_argument("--dry-run", action="store_true",
                    help="print the shard plan; make NO filesystem changes")
    ap.add_argument("--selftest", action="store_true",
                    help="run the synthetic-facade selftest and exit")
    args = ap.parse_args()

    if args.budget <= 0:
        print("ERROR: --budget must be positive", file=sys.stderr)
        return 1

    return run_split(args.family, args.func, args.output,
                     args.budget, args.dry_run, args.weight_mode,
                     args.weight_norm, args.weight_exp)


if __name__ == "__main__":
    sys.exit(main())
