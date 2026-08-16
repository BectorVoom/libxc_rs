#!/usr/bin/env python3
"""
Unified Maple→kernel driver: one CLI that regenerates the complete CubeCL
kernel tree (LDA + GGA + MGGA) — per-functional translation plus the
oversized-functional shard recipes.

This is a THIN ORCHESTRATOR. It does not reimplement translation or
splitting logic. Under the Phase 11 D-10 per-functional-subcrate model the
`translate` subcommand calls each translator's `emit_per_functional` entry
point DIRECTLY (in-process) — see "Why drive the translators directly?"
below. The `split` subcommand runs the SHARD_RECIPES table through
tools/split_per_functional_subcrate.py (in-process), and `all` chains
translate → split: a single command that reproduces the committed tree
byte-for-byte.

# Splitting-criteria knobs (Phase 11 D-10 model)

The CubeCL `#[cube]` proc-macro fans out into very large generated Rust per
kernel (see `docs/manual/Cubecl/cubecl_macro_fanout_manual.md`). Under the
Phase 11 per-functional-subcrate model there is now exactly ONE knob:

  --split-threshold N   per-cube-fn line cap (default 4500)
                        The SPLIT_THRESHOLD module-level constant inside
                        translate_lda_v2.py / translate_gga.py /
                        translate_mgga.py. The `translate` subcommand calls
                        each translator's `emit_per_functional` directly and
                        forwards this value, so a non-default value IS honored
                        (no longer a warn-and-fall-back). Larger value ⇒ FEWER
                        per-cube-fn split parts; D-LOCK-B floor is 4500.

There is NO per-sub-crate line cap any more: each functional is its OWN crate
under `crates/kernels/{family}/<func>/` (the old `--target-max` knob is gone).
Five MGGA functionals additionally exceed the single-rustc-process memory
ceiling; their shard parameters live in the SHARD_RECIPES table below.

User-direction memo (see feedback_splitting_terminology.md):
  "fewer files / less aggressive splitting" ⇒ RAISE --split-threshold
  "more files / more aggressive splitting" ⇒ LOWER --split-threshold
The arithmetic and the natural-language phrasing are inverted; always
confirm the desired file COUNT direction before tweaking.

# Wrapped tools

  translate (Phase 11 D-10: drives the per-family translators DIRECTLY via
            their `emit_per_functional` entry point, iterating Maple sources
            under libxc-master/src/maple2c/{family}_{exc,vxc}/ and emitting
            one per-functional subcrate per source into
            crates/kernels/{family}/<func>/. Replaces the stale
            regen_phase09.py in-place-replacement pipeline, which assumed
            pre-existing subcrate dirs and the pre-q07 crates/kernel-* layout
            and so cannot drive a D-10a clean-slate regen):
    LDA, GGA, MGGA, all

  split: SHARD_RECIPES → tools/split_per_functional_subcrate.py (in-process).
    Facade + `_pK` shard crates for the 5 MGGA functionals over the
    single-rustc-process memory ceiling; idempotent on already-split trees.

  all = translate, then split, for the selected family.

# Build-time model (incremental regen + sync emitter)

Three mechanisms keep `cargo build` and regen time proportional to what
actually changed, while emitting byte-identical kernel code (calculation
results cannot shift — the translation logic itself is untouched):

  1. Write-if-unchanged sync (translate_v2/emit.py): a regenerated file whose
     bytes match the on-disk file is NOT rewritten, so its mtime survives and
     cargo's fingerprint for that subcrate stays valid — an unchanged
     functional costs zero rustc time on the next build. Stale files from a
     prior split layout are swept at finalize (preserves the 11.1-02-fix5
     E0761 guarantee without the old wipe-first full rewrite).
  2. --changed-only: an input-hash manifest (.cache/maple2kernels-manifest.json)
     keyed on the Maple .c source + effective threshold + all translator
     Python sources skips unchanged functionals without even parsing them.
  3. --jobs N: per-functional translations fan out over a process pool
     (default cpu_count-1). Each subcrate is written wholly by one worker, so
     emitted bytes are independent of N and scheduling order.
  4. Size-band thresholds (--thresholds-map, default auto): per-functional
     compile RSS is steeply super-linear in the largest #[cube] part's size,
     so functionals with an "in-band" decision in tools/adaptive_thresholds.json
     regenerate at the lowered threshold that lands their largest part in the
     low-RSS band (<= 2500 lines) WITHOUT crossing the part-count explosion
     cliff. Functionals that are already small (default-ok) or whose dominant
     part is CSE-irreducible (needs-sharding) stay at the global threshold —
     no splitting is added where it cannot pay for itself in memory.

Execution-speed posture (unchanged by the above): routed entry kernels carry
`#[cube(launch_unchecked)]` (no per-thread bounds checks), all temporaries are
register-resident scalars, and oversized outputs are CSE-chunked so shared
subexpressions are computed once and threaded through tuple returns instead of
recomputed. Floating-point operation order is preserved verbatim from the
maple2c source (bit-level parity with the libxc oracle).

# Usage

  tools/maple_to_kernels.py all       --family all -j 15     # full regen
  tools/maple_to_kernels.py all       --family all --changed-only
  tools/maple_to_kernels.py translate --family gga --func gga_c_pbe
  tools/maple_to_kernels.py split     --family mgga
  tools/maple_to_kernels.py --dry-run all --family mgga

# Why drive the translators directly? (Phase 11 D-10)

The previous `translate` path delegated to `regen_phase09.py`, which
discovers each functional's EXISTING sub-crate dir and replaces its
contents in place. That flow cannot drive the D-10a clean-slate regen:
plan 11-03 deletes the entire old layout first, so there are no
pre-existing dirs to discover — and `regen_phase09.py` also still scans
the pre-q07 `crates/kernel-*` path prefix. It is therefore bypassed.

Instead, each translator now exposes an `emit_per_functional(c_file,
func_name, family, is_vxc_only, split_threshold)` entry point that emits a
complete per-functional subcrate (Cargo.toml + lib.rs + nested-by-output
src tree) via `tools/translate_v2/emit.py`. This driver iterates the Maple
sources for a family and calls that entry point in-process for each — no
subprocess, no staging dir, no in-place replacement.
"""

import argparse
import hashlib
import importlib
import json
import multiprocessing
import os
import sys
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
TOOLS = REPO_ROOT / "tools"
MAPLE2C = REPO_ROOT / "libxc-master" / "src" / "maple2c"

# Phase 11 D-LOCK-B: per-cube-fn line cap. 4500 leaves headroom vs the 5000
# hard cap; the three translators carry the same SPLIT_THRESHOLD default.
DEFAULT_SPLIT_THRESHOLD = 4500

FAMILIES = ("lda", "gga", "mgga")

# tools/ on the path so the per-family translator modules import cleanly.
sys.path.insert(0, str(TOOLS))


def _hyb_family(func_name: str) -> str | None:
    """Maple `hyb_*` sources live under the base family's maple2c dir."""
    for fam in FAMILIES:
        if func_name.startswith(f"hyb_{fam}"):
            return fam
    return None


def discover_maple_sources(family: str) -> list[tuple[Path, str, bool]]:
    """Every Maple .c source for a family → (c_path, func_name, is_vxc_only).

    Scans libxc-master/src/maple2c/{family}_exc/ and {family}_vxc/. The _vxc
    sources are emitted with is_vxc_only=True. Skips Makefile.am and *Zone*
    template files (mirrors translate_*.py batch_translate)."""
    found = []
    for is_vxc, sub in ((False, f"{family}_exc"), (True, f"{family}_vxc")):
        d = MAPLE2C / sub
        if not d.is_dir():
            continue
        for c in sorted(d.glob("*.c")):
            name = c.name
            if name == "Makefile.am" or "Zone" in name:
                continue
            func_name = c.stem
            # hyb_* sources may sit under a different family's maple2c dir;
            # only take them for the family they actually belong to.
            hf = _hyb_family(func_name)
            if hf is not None and hf != family:
                continue
            found.append((c, func_name, is_vxc))
    return found


def load_thresholds_map(path):
    """Load the per-functional decision map produced by
    `tools/adaptive_split.py --all` (the size-band selector). Returns
    ``{"family/func": decision_dict}`` (each decision carries at least
    ``threshold`` and ``status``), or ``None`` when ``path`` is falsy.

    Wiring this in lets regen use each functional's own split_threshold instead
    of a single global one — capturing the super-linear per-part RSS win where a
    functional's cliff allows it, and falling back to the global value for any
    functional absent from the map. See memory
    project_compile_rss_model_chunk_sizing."""
    if not path:
        return None
    import json
    with open(path) as f:
        m = json.load(f)
    print(f"[thresholds] loaded {len(m)} per-functional decisions from {path}")
    return m


TRANSLATOR_MODS = {
    "lda": "translate_lda_v2",
    "gga": "translate_gga",
    "mgga": "translate_mgga",
}

# --- Oversized-functional shard recipes ---------------------------------------
# Five MGGA functionals exceed the 30 GB single-rustc-process ceiling and are
# post-processed by tools/split_per_functional_subcrate.py into `_pK` shard
# crates behind a facade that keeps the public package name (quick tasks
# 260520-eem/-k1q + 11-10 sweep; see .planning history). The two densest ones
# additionally regenerate with hierarchical CSE (LIBXC_RS_HIERARCHICAL_CSE=1)
# so no single `#[cube]` fn exceeds the proc-macro RSS cliff. These recipes
# reproduce the committed layout byte-for-byte; `translate` applies the `env`
# automatically and `split`/`all` run the splitter step.
SHARD_RECIPES = {
    "mgga/mgga_c_tpss": {
        "output": "lxc_pol", "budget": 40, "weight_mode": "files"},
    "mgga/mgga_c_kcisk": {
        "output": "lxc_pol", "budget": 40, "weight_mode": "files"},
    "mgga/mgga_c_rmggac": {
        "output": "lxc_pol", "budget": 40, "weight_mode": "files"},
    "mgga/mgga_c_tpssloc": {
        "output": "lxc_pol", "budget": 10000, "weight_mode": "files",
        "env": {"LIBXC_RS_HIERARCHICAL_CSE": "1",
                "LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER": "1"}},
    "mgga/mgga_c_revtpss": {
        "output": "lxc_pol", "budget": 10000, "weight_mode": "files",
        "env": {"LIBXC_RS_HIERARCHICAL_CSE": "1",
                "LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER": "1"}},
}

# --- Incremental regen manifest (build-time optimization) ---------------------
# Maps "family/func" -> input-state hash covering everything that determines
# the emitted bytes: the Maple .c source, the effective split threshold, and
# every Python source in the translation pipeline. With --changed-only, a
# functional whose hash matches (and whose subcrate exists) is skipped without
# even parsing — so an edit to one Maple source or one translator regenerates
# only what that edit can affect. Combined with the emit.py write-if-unchanged
# sync, an untouched functional's files keep their mtimes and cargo does not
# rebuild its subcrate.
MANIFEST_PATH = REPO_ROOT / ".cache" / "maple2kernels-manifest.json"

# The Python sources whose behavior feeds the emitted bytes.
_TOOLCHAIN_SOURCES = (
    "maple_to_kernels.py",
    "kernel_routing.py",
    "translate_lda_v2.py",
    "translate_gga.py",
    "translate_mgga.py",
    "translate_v2/__init__.py",
    "translate_v2/cse.py",
    "translate_v2/emit.py",
    "translate_v2/helpers_allowlist.py",
    "translate_v2/per_functional.py",
)

_toolchain_hash_cache = None


def toolchain_hash() -> str:
    """Combined sha256 of the translation pipeline's Python sources."""
    global _toolchain_hash_cache
    if _toolchain_hash_cache is None:
        h = hashlib.sha256()
        for rel in _TOOLCHAIN_SOURCES:
            p = TOOLS / rel
            h.update(rel.encode())
            h.update(p.read_bytes() if p.is_file() else b"<missing>")
        _toolchain_hash_cache = h.hexdigest()
    return _toolchain_hash_cache


_EMIT_ENV_VARS = ("LIBXC_RS_HIERARCHICAL_CSE",
                  "LIBXC_RS_ACCEPT_OVERSIZED_WRAPPER",
                  "LIBXC_RS_HIERARCHICAL_META_FANOUT",
                  "LIBXC_RS_CHUNK_FIRST")


def input_hash(c_path: Path, thr: int, env: dict | None = None) -> str:
    h = hashlib.sha256()
    h.update(toolchain_hash().encode())
    h.update(str(thr).encode())
    # Emit-shaping env knobs (hier-CSE pipeline for the oversized-sharded
    # functionals) change output bytes, so the EFFECTIVE value (per-task
    # recipe env over the ambient environment) is part of the input state.
    merged = dict(os.environ)
    merged.update(env or {})
    for var in _EMIT_ENV_VARS:
        h.update(f"{var}={merged.get(var, '')}".encode())
    h.update(c_path.read_bytes())
    return h.hexdigest()


def load_manifest() -> dict:
    try:
        with open(MANIFEST_PATH) as f:
            return json.load(f)
    except (OSError, json.JSONDecodeError):
        return {}


def save_manifest(m: dict) -> None:
    MANIFEST_PATH.parent.mkdir(parents=True, exist_ok=True)
    tmp = MANIFEST_PATH.with_suffix(".json.tmp")
    with open(tmp, "w") as f:
        json.dump(m, f, indent=1, sort_keys=True)
    tmp.replace(MANIFEST_PATH)


def _translate_one(task: tuple) -> tuple:
    """Worker for one functional (runs in-process or in a Pool worker).

    Returns (func_name, status, message, emit_stats) where status is one of
    "ok" | "skip" | "fail". The translator module import is cached per
    process, so a Pool worker pays the import once, not per functional.

    ``env`` (a SHARD_RECIPES entry's emit-shaping vars) is applied around the
    emit and restored afterwards — Pool workers are reused across tasks, so a
    leaked var would corrupt the next functional's emit."""
    family, mod_name, c_path, func_name, is_vxc, thr, note, env = task
    mod = importlib.import_module(mod_name)
    from translate_v2 import emit
    emit.reset_write_stats()
    saved = {k: os.environ.get(k) for k in (env or {})}
    os.environ.update(env or {})
    try:
        mods = mod.emit_per_functional(c_path, func_name, family, is_vxc, thr)
        return (func_name, "ok", f"{len(mods)} output module(s){note}",
                emit.write_stats())
    except RuntimeError as e:
        # check_unimplemented_math and similar expected skips.
        return (func_name, "skip", str(e), emit.write_stats())
    except Exception as e:  # noqa: BLE001 — surface, count, keep going
        return (func_name, "fail", f"{type(e).__name__}: {e}",
                emit.write_stats())
    finally:
        for k, v in saved.items():
            if v is None:
                os.environ.pop(k, None)
            else:
                os.environ[k] = v


def translate_family(family: str, split_threshold: int, dry_run: bool,
                     thresholds: dict | None = None,
                     func_filter: str | None = None,
                     jobs: int = 1,
                     changed_only: bool = False) -> int:
    """Drive `family`'s translator directly: iterate Maple sources, call the
    translator's `emit_per_functional` for each, emitting one per-functional
    subcrate per source into crates/kernels/{family}/<func>/.

    When ``thresholds`` (the adaptive_split decision map) is supplied, each
    functional regenerates at its OWN ``threshold`` (falling back to the global
    ``split_threshold`` when absent from the map).

    ``jobs > 1`` fans the per-functional translations out over a process pool
    (each functional's subcrate is written wholly by one worker, so outputs are
    independent of scheduling order). ``changed_only`` consults the input-hash
    manifest and skips functionals whose Maple source, threshold, and tool
    sources are all unchanged since the last successful regen."""
    sources = discover_maple_sources(family)
    src = "per-functional map" if thresholds else f"global={split_threshold}"
    print(f"[{family}] {len(sources)} Maple source(s) discovered "
          f"(split_threshold: {src}, jobs={jobs}"
          f"{', changed-only' if changed_only else ''})")
    t0 = time.monotonic()
    ok = skipped = failed = cached = 0
    needs_sharding = []
    manifest = load_manifest() if changed_only or not dry_run else {}
    tasks = []
    task_hashes = {}
    for c_path, func_name, is_vxc in sources:
        if func_filter and func_name != func_filter:
            continue
        dec = (thresholds or {}).get(f"{family}/{func_name}")
        thr = split_threshold
        note = ""
        if dec:
            status = dec.get("status", "?")
            # Threshold policy (build-memory optimization, 2026-08-15):
            # apply a lowered per-functional threshold ONLY for "in-band"
            # decisions — the ones where the size-band selector proved the
            # extra splitting lands the largest part in the low-RSS band
            # (<= SIZE_TARGET lines) without crossing the part-count
            # explosion cliff. "default-ok" functionals stay at the global
            # default (no splitting to avoid), and "needs-sharding" ones
            # stay too: their dominant part is CSE-irreducible, so a lower
            # threshold only fragments the REST of the functional without
            # shrinking the RSS driver — splitting with zero memory payoff.
            if status == "in-band":
                thr = dec.get("threshold", split_threshold)
                note = f" [thr={thr} in-band]"
            elif status == "needs-sharding":
                needs_sharding.append(func_name)
        if dry_run:
            print(f"  [dry-run] would emit {family}/{func_name} "
                  f"(vxc_only={is_vxc}) from {c_path.name}{note}")
            ok += 1
            continue
        key = f"{family}/{func_name}"
        recipe_env = SHARD_RECIPES.get(key, {}).get("env") or {}
        if recipe_env:
            note += " [hier-CSE]"
        ih = input_hash(c_path, thr, recipe_env)
        task_hashes[key] = ih
        if changed_only and manifest.get(key) == ih and \
                (REPO_ROOT / "crates" / "kernels" / family / func_name /
                 "Cargo.toml").is_file():
            cached += 1
            continue
        tasks.append((family, TRANSLATOR_MODS[family], str(c_path),
                      func_name, is_vxc, thr, note, recipe_env))

    agg = {"written": 0, "unchanged": 0, "deleted": 0}

    def _consume(res):
        nonlocal ok, skipped, failed
        func_name, status, msg, stats = res
        for k in agg:
            agg[k] += stats.get(k, 0)
        key = f"{family}/{func_name}"
        if status == "ok":
            print(f"  OK: {key} ({msg})")
            ok += 1
            manifest[key] = task_hashes[key]
        elif status == "skip":
            print(f"  SKIP: {key}: {msg}", file=sys.stderr)
            skipped += 1
            manifest.pop(key, None)
        else:
            print(f"  FAIL: {key}: {msg}", file=sys.stderr)
            failed += 1
            manifest.pop(key, None)

    if tasks:
        if jobs > 1:
            with multiprocessing.Pool(processes=min(jobs, len(tasks))) as pool:
                for res in pool.imap_unordered(_translate_one, tasks):
                    _consume(res)
        else:
            for task in tasks:
                _consume(_translate_one(task))

    if not dry_run:
        save_manifest(manifest)
    dt = time.monotonic() - t0
    print(f"[{family}] ok={ok} skipped={skipped} failed={failed} "
          f"cached={cached} in {dt:.1f}s | files: {agg['written']} written, "
          f"{agg['unchanged']} unchanged (mtime preserved), "
          f"{agg['deleted']} stale deleted")
    if needs_sharding:
        print(f"[{family}] {len(needs_sharding)} functional(s) flagged "
              f"needs-sharding (run `split` next): {', '.join(needs_sharding)}",
              file=sys.stderr)
    return 1 if failed else 0


def _split_families(family: str) -> list[str]:
    """Family list for split invocations (no 'all' passthrough)."""
    return list(FAMILIES) if family == "all" else [family]


def do_translate(args: argparse.Namespace) -> int:
    """Phase 11 D-10: drive the per-family translators directly (no subprocess,
    no regen_phase09.py). Each family's emit_per_functional writes complete
    per-functional subcrates under crates/kernels/{family}/<func>/."""
    map_arg = getattr(args, "thresholds_map", None) or "auto"
    if map_arg == "auto":
        # Default: pick up the checked-in size-band decision map when present.
        # Only "in-band" decisions are applied (see translate_family), so the
        # map can never ADD splitting beyond the size-band policy.
        auto = TOOLS / "adaptive_thresholds.json"
        map_arg = str(auto) if auto.is_file() else None
    elif map_arg == "none":
        map_arg = None
    thresholds = load_thresholds_map(map_arg)
    rc = 0
    families = list(FAMILIES) if args.family == "all" else [args.family]
    for fam in families:
        rc = translate_family(fam, args.split_threshold, args.dry_run,
                              thresholds, getattr(args, "only_func", None),
                              jobs=getattr(args, "jobs", 1),
                              changed_only=getattr(args, "changed_only", False),
                              ) or rc
        if rc and not args.dry_run:
            return rc
    return rc


def do_split(args: argparse.Namespace) -> int:
    """Run the oversized-functional shard recipes (SHARD_RECIPES) for the
    selected family/families via split_per_functional_subcrate.run_split.
    Idempotent: an already-split facade no-ops (the splitter's
    `_already_split` guard), so `all` can always run translate then split."""
    import split_per_functional_subcrate as sps
    rc = 0
    only = getattr(args, "only_func", None)
    for fam in _split_families(args.family):
        for key, recipe in SHARD_RECIPES.items():
            r_fam, r_func = key.split("/", 1)
            if r_fam != fam or (only and r_func != only):
                continue
            print(f"[split] {key} {recipe['output']} "
                  f"--budget {recipe['budget']} "
                  f"--weight-mode {recipe['weight_mode']}", flush=True)
            rc = sps.run_split(r_fam, r_func, recipe["output"],
                               recipe["budget"], args.dry_run,
                               recipe["weight_mode"]) or rc
            if rc and not args.dry_run:
                return rc
    return rc


def do_all(args: argparse.Namespace) -> int:
    rc = do_translate(args)
    if rc and not args.dry_run:
        return rc
    return do_split(args) or rc


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="maple_to_kernels.py",
        description=(
            "Unified Maple→kernel driver: translate + split per family with "
            "consistent splitting-criteria knobs."
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help=(
            "Forward --dry-run to each underlying tool. The tool runs but "
            "produces only a planning/diff summary, no on-disk changes."
        ),
    )

    sub = parser.add_subparsers(dest="cmd", required=True)

    def add_family(p: argparse.ArgumentParser) -> None:
        p.add_argument(
            "--family", choices=("lda", "gga", "mgga", "all"), default="all",
            help="Functional family to operate on (default: all).",
        )

    p_t = sub.add_parser("translate", help="Run Maple→Rust translators.")
    add_family(p_t)
    p_t.add_argument(
        "--split-threshold", type=int, default=DEFAULT_SPLIT_THRESHOLD,
        help=(
            f"Per-cube-fn line cap (default {DEFAULT_SPLIT_THRESHOLD}). "
            "Larger ⇒ fewer per-functional .rs files. LDA translator does "
            "not honor this; non-default value warns and falls back. "
            "Used as the FALLBACK for functionals absent from --thresholds-map."
        ),
    )
    p_t.add_argument(
        "--thresholds-map", default="auto", metavar="PATH|auto|none",
        help=(
            "JSON decision map from `adaptive_split.py --all` (per-functional "
            "size-band thresholds). Only 'in-band' decisions are applied; "
            "everything else uses --split-threshold. Default 'auto' picks up "
            "tools/adaptive_thresholds.json when present; 'none' disables."
        ),
    )
    p_t.add_argument(
        "--func", dest="only_func", default=None, metavar="NAME",
        help="Regenerate only this one functional (e.g. gga_c_pbe) — useful for "
             "spot-checks and incremental compile-gating.",
    )
    p_t.add_argument(
        "--jobs", "-j", type=int, default=max(1, (os.cpu_count() or 2) - 1),
        metavar="N",
        help="Translate N functionals in parallel (process pool; default: "
             "cpu_count-1). Each subcrate is written wholly by one worker, so "
             "output bytes are independent of N.",
    )
    p_t.add_argument(
        "--changed-only", action="store_true",
        help="Skip functionals whose Maple source, threshold, and translator "
             "sources are unchanged since the last successful regen (input-"
             "hash manifest at .cache/maple2kernels-manifest.json). Unchanged "
             "subcrates keep their mtimes, so cargo does not rebuild them.",
    )
    p_t.set_defaults(func=do_translate)

    p_s = sub.add_parser(
        "split",
        help="Run the oversized-functional shard recipes (SHARD_RECIPES) — "
             "facade + _pK shard crates for the 5 functionals that exceed "
             "the single-rustc-process memory ceiling.")
    add_family(p_s)
    p_s.add_argument(
        "--func", dest="only_func", default=None, metavar="NAME",
        help="Shard only this one functional's recipe.",
    )
    p_s.set_defaults(func=do_split)

    p_a = sub.add_parser(
        "all",
        help="translate then split — the one-command full regen that "
             "reproduces the committed kernel tree.")
    add_family(p_a)
    p_a.add_argument(
        "--split-threshold", type=int, default=DEFAULT_SPLIT_THRESHOLD,
    )
    p_a.add_argument(
        "--thresholds-map", default="auto", metavar="PATH|auto|none",
        help="Per-functional size-band map (only 'in-band' decisions applied; "
             "default 'auto' = tools/adaptive_thresholds.json when present).",
    )
    p_a.add_argument(
        "--func", dest="only_func", default=None, metavar="NAME",
        help="Regenerate (and re-shard, if recipe'd) only this functional.",
    )
    p_a.add_argument(
        "--jobs", "-j", type=int, default=max(1, (os.cpu_count() or 2) - 1),
        metavar="N",
    )
    p_a.add_argument("--changed-only", action="store_true")
    p_a.set_defaults(func=do_all)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
