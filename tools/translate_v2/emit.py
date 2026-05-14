#!/usr/bin/env python3
"""Per-functional subcrate emitter for splitter v2 (Phase 11 D-04, D-10).

Writes ``crates/kernels/{family}/<func>/`` as a complete Cargo crate with the
nested-by-output ``src`` layout that matches the q02 ``mgga_c_b94`` golden spec
(commit 504d8560):

    crates/kernels/{family}/<func>/
      Cargo.toml                     name = "libxc-kernel-<func>"
      src/lib.rs                     pub mod <output>;  enumeration
      src/<output>.rs                single-file output (<= SPLIT_THRESHOLD)
      src/<output>/mod.rs            private `mod partNN;` + #[cube] wrapper
      src/<output>/partNN.rs         per-output-component part OR D-02 CSE chunk

This module owns the FILESYSTEM LAYOUT (directory structure, file naming,
partNN indexing, deterministic writes). It does NOT generate kernel bodies —
the translators (``translate_lda_v2.py`` / ``translate_gga.py`` /
``translate_mgga.py``) build the file-content strings (via the reused
``generate_function`` emitter + the CSE pass) and hand them in.

Routing: a routed functional's per-(level,spin) entry wrapper carries
``#[cube(launch_unchecked)]``; an unrouted/deferred functional's entry wrapper,
and every ``_part`` helper / D-02 CSE chunk, carries plain ``#[cube]``. That
decision (``tools/kernel_routing.py`` ``cached_routed_funcnames``) is the
caller's — it is already baked into the ``wrapper_src`` / ``part_srcs`` strings
passed here; emit.py never synthesizes a ``#[cube]`` attribute itself.

Determinism (D-LOCK-D): every write goes through ``_write`` — single trailing
newline, no per-line trailing whitespace, parents auto-created. partNN indices
are 0-based sequence order, reset per output. Emitting the same functional
twice produces byte-identical files.

Build env source of truth: .cargo/config.toml (do not duplicate values here).
"""

from pathlib import Path

# Repo `crates/kernels` root. Overridable for the selftest via set_kernels_root.
KERNELS_ROOT = Path(__file__).resolve().parent.parent.parent / "crates" / "kernels"

# q02 golden-spec crate-level allow-list (wider than the old numbered-subcrate set).
CRATE_ALLOW = (
    "#![allow(unused_imports, unused_variables, non_snake_case, "
    "clippy::excessive_precision, clippy::too_many_arguments, "
    "clippy::needless_return)]"
)

# cubecl dependency line — copied verbatim from the numbered-subcrate manifests.
# No cuda/wgpu/hip features here; GPU backends are feature-gated at the workspace root.
CUBECL_DEP = 'cubecl = { version = "0.10.0", default-features = false, features = ["cpu"] }'

# libm dependency line — emitted only when a functional references libm primitives.
LIBM_DEP = 'libm = "0.2"'


def set_kernels_root(path) -> None:
    """Override the crates/kernels root (used by the selftest to emit into a tempdir)."""
    global KERNELS_ROOT
    KERNELS_ROOT = Path(path)


def subcrate_dir(family: str, func: str) -> Path:
    """``crates/kernels/{family}/<func>/`` — the family level is a plain directory."""
    return KERNELS_ROOT / family / func


def _write(path: Path, content: str) -> None:
    """Deterministic file write: create parents, strip per-line trailing whitespace,
    guarantee exactly one trailing newline. This is the idempotency contract surface."""
    path.parent.mkdir(parents=True, exist_ok=True)
    body = content.rstrip("\n") + "\n"
    body = "\n".join(line.rstrip() for line in body.split("\n"))
    path.write_text(body, encoding="utf-8")


def emit_cargo_toml(family: str, func: str, needs_libm: bool) -> None:
    """Write ``<func>/Cargo.toml``.

    ``name = "libxc-kernel-<func>"`` with the func id underscores verbatim (cargo
    permits underscores in package names; the lib path becomes
    ``libxc_kernel_<func>``). Deps: ``cubecl`` (verbatim line),
    ``libxc-kernel-math = { path = "../../math" }`` (note the ``../../`` — per-
    functional subcrates sit one level deeper than the old numbered subcrates),
    and ``libm`` IFF ``needs_libm``. No ``[dev-dependencies]`` (a kernel
    subcrate dev-dep would re-create the verify/ OOM cycle)."""
    lines = [
        "[package]",
        f'name = "libxc-kernel-{func}"',
        'version = "0.1.0"',
        'edition = "2024"',
        "",
        "[dependencies]",
        CUBECL_DEP,
        'libxc-kernel-math = { path = "../../math" }',
    ]
    if needs_libm:
        lines.append(LIBM_DEP)
    _write(subcrate_dir(family, func) / "Cargo.toml", "\n".join(lines))


def emit_lib_rs(family: str, func: str, output_modules: list) -> None:
    """Write ``<func>/src/lib.rs``: crate-level ``#![allow(...)]`` (q02 wide list),
    a module doc-comment, then one ``pub mod <output>;`` per entry in
    ``output_modules``. The caller passes ``output_modules`` already ordered
    (unpol levels then pol levels, lowest derivative first). The ``pub mod``
    line is identical whether ``<output>`` resolves to a ``.rs`` file or an
    ``<output>/`` directory — Rust handles both transparently."""
    d = subcrate_dir(family, func)
    lines = [
        f"//! {func.upper()} kernel — per-functional subcrate (Phase 11 D-10).",
        "//!",
        "//! Output modules are enumerated below; each is either a single",
        "//! `src/<output>.rs` file or a `src/<output>/` directory with a",
        "//! nested-by-output partNN layout (D-04).",
        "//!",
        "//! Auto-emitted by tools/translate_v2/emit.py — do not hand-edit.",
        "",
        CRATE_ALLOW,
        "",
    ]
    for om in output_modules:
        lines.append(f"pub mod {om};")
    _write(d / "src" / "lib.rs", "\n".join(lines))


def emit_single_output(family: str, func: str, output: str, cube_fn_src: str) -> None:
    """Write ``<func>/src/<output>.rs`` — a single-file output (the functional fit
    under SPLIT_THRESHOLD at this output, so no CSE chunking was needed).
    ``cube_fn_src`` is the complete file content the caller assembled (doc-comment,
    file-level ``#![allow]``, import block, and the ``#[cube]`` /
    ``#[cube(launch_unchecked)]`` fn) — emit.py writes it deterministically."""
    _write(subcrate_dir(family, func) / "src" / f"{output}.rs", cube_fn_src)


def emit_chunked_output(family: str, func: str, output: str,
                        wrapper_src: str, part_srcs: list) -> None:
    """Write a chunked output: ``<func>/src/<output>/mod.rs`` plus one
    ``<func>/src/<output>/partNN.rs`` per entry in ``part_srcs``.

    ``wrapper_src`` is the complete ``mod.rs`` content (doc-comment, file-level
    ``#![allow]``, the private ``mod partNN;`` declarations, the
    ``use partNN::...;`` lines, and the ``#[cube]`` / ``#[cube(launch_unchecked)]``
    entry wrapper) — the caller owns it because the caller knows the part count
    and the routing verdict. ``part_srcs`` are the complete ``partNN.rs`` file
    contents (each its own doc-comment + ``#![allow]`` + imports + ``#[cube]``
    helper). partNN indices are 0-based sequence order, **reset per output**
    (NOT a global counter)."""
    out_dir = subcrate_dir(family, func) / "src" / output
    _write(out_dir / "mod.rs", wrapper_src)
    for nn, part_src in enumerate(part_srcs):
        _write(out_dir / f"part{nn}.rs", part_src)


# --- self-test ---------------------------------------------------------------
def _selftest() -> int:
    """Emit a tiny synthetic functional into a tempdir, assert the expected files
    and strings exist, emit a second time and diff (must be byte-identical),
    then clean up. Exit 0 on success, 1 on failure."""
    import shutil
    import tempfile

    tmp = tempfile.mkdtemp(prefix="emit_selftest_")
    ok = True
    fam, func = "mgga", "mgga_x_selftest"

    single_src = (
        "//! MGGA_X_SELFTEST exc unpol kernel.\n"
        f"{CRATE_ALLOW}\n"
        "use cubecl::prelude::*;\n"
        "#[cube(launch_unchecked)]\n"
        "pub fn mgga_x_selftest_exc_unpol(rho: &Array<f64>) {}\n"
    )
    wrapper_src = (
        "//! MGGA_X_SELFTEST polarized kxc-level kernel.\n"
        f"{CRATE_ALLOW}\n"
        "mod part0;\nmod part1;\n"
        "use cubecl::prelude::*;\n"
        "#[cube(launch_unchecked)]\n"
        "pub fn mgga_x_selftest_kxc_pol(rho: &Array<f64>) {}\n"
    )
    part_srcs = [
        "//! part 0/2.\n#[cube]\nfn p0<F: Float>(x: F) -> (F, F) { (x, x) }\n",
        "//! part 1/2.\n#[cube]\nfn p1<F: Float>(x: F) -> (F,) { (x,) }\n",
    ]

    def _do_emit():
        emit_cargo_toml(fam, func, needs_libm=True)
        emit_lib_rs(fam, func, ["exc_unpol", "kxc_pol"])
        emit_single_output(fam, func, "exc_unpol", single_src)
        emit_chunked_output(fam, func, "kxc_pol", wrapper_src, part_srcs)

    def _snapshot(root):
        snap = {}
        for p in sorted(Path(root).rglob("*")):
            if p.is_file():
                snap[str(p.relative_to(root))] = p.read_text(encoding="utf-8")
        return snap

    try:
        set_kernels_root(tmp)
        _do_emit()
        snap1 = _snapshot(tmp)

        d = subcrate_dir(fam, func)
        cargo = (d / "Cargo.toml").read_text()
        for needle in ('name = "libxc-kernel-mgga_x_selftest"',
                       'path = "../../math"', 'libm = "0.2"'):
            if needle not in cargo:
                print(f"SELFTEST FAIL: Cargo.toml missing {needle!r}")
                ok = False
        if "[dev-dependencies]" in cargo:
            print("SELFTEST FAIL: Cargo.toml has a [dev-dependencies] section")
            ok = False

        lib = (d / "src" / "lib.rs").read_text()
        for needle in ("pub mod exc_unpol;", "pub mod kxc_pol;",
                       "clippy::needless_return"):
            if needle not in lib:
                print(f"SELFTEST FAIL: lib.rs missing {needle!r}")
                ok = False

        expected_files = [
            d / "src" / "exc_unpol.rs",
            d / "src" / "kxc_pol" / "mod.rs",
            d / "src" / "kxc_pol" / "part0.rs",
            d / "src" / "kxc_pol" / "part1.rs",
        ]
        for f in expected_files:
            if not f.exists():
                print(f"SELFTEST FAIL: expected file missing: {f}")
                ok = False

        wrapper = (d / "src" / "kxc_pol" / "mod.rs").read_text()
        if "mod part0;" not in wrapper or "launch_unchecked" not in wrapper:
            print("SELFTEST FAIL: kxc_pol/mod.rs missing mod decl or routing attr")
            ok = False

        # Double-emit idempotency: re-run the exact same emit, diff the tree.
        _do_emit()
        snap2 = _snapshot(tmp)
        if snap1 != snap2:
            diff_keys = sorted(set(snap1) ^ set(snap2)) or [
                k for k in snap1 if snap1.get(k) != snap2.get(k)
            ]
            print(f"SELFTEST FAIL: double-emit not byte-identical: {diff_keys}")
            ok = False
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    if ok:
        print("SELFTEST PASS: per-functional subcrate emitted, double-emit "
              "byte-identical, layout matches q02 golden spec")
        return 0
    return 1


if __name__ == "__main__":
    import sys
    if "--selftest" in sys.argv:
        sys.exit(_selftest())
    print("usage: python3 tools/translate_v2/emit.py --selftest")
    sys.exit(2)
