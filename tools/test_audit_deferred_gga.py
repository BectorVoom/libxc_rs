#!/usr/bin/env python3
"""Tests for tools/audit_deferred_gga.py — Plan 09-05 audit script.

Tests cover the 7 behaviors documented in the plan:

  1. Canonical-list resolution — exactly 25 functionals; the 6 RESEARCH-named
     functionals are all present.
  2. Audit per-functional coverage (happy path) — every (level, spin) covered.
  3. Gap detection: orphan_file (file on disk not listed in mod.rs).
  4. Gap detection: commented_pub_mod (// pub mod entry).
  5. Gap detection: FORBIDDEN_GATE (#[cfg(feature = "order-…")]).
  6. Gap detection: missing_lib_export (functional dir exists but no lib.rs entry).
  7. Multi-sub-crate aggregation — UNION across letter-suffix sub-crates.

The tests build minimal fixture trees in tempfile.TemporaryDirectory and call
audit_functional with the fixture root. No real-codebase dependency in tests.
"""

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

# Make tools/ importable.
TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import audit_deferred_gga as agda  # noqa: E402


def _make_subcrate(
    root: Path,
    subcrate: str,
    functional: str,
    files: list[str],
    *,
    mod_rs_lines: list[str] | None = None,
    lib_rs_extra: list[str] | None = None,
) -> Path:
    """Create a sub-crate directory tree mirroring the live layout.

    crates/{subcrate}/src/{functional}/{files}
    crates/{subcrate}/src/{functional}/mod.rs (auto-generated unless mod_rs_lines given)
    crates/{subcrate}/src/lib.rs with `pub mod {functional};` plus lib_rs_extra
    """
    funcdir = root / "crates" / subcrate / "src" / functional
    funcdir.mkdir(parents=True, exist_ok=True)
    for fname in files:
        (funcdir / fname).write_text("// stub\n")
    if mod_rs_lines is None:
        mod_rs_lines = [f"pub mod {Path(f).stem};" for f in files if f.endswith(".rs")]
    (funcdir / "mod.rs").write_text("\n".join(mod_rs_lines) + "\n")
    libdir = root / "crates" / subcrate / "src"
    libfile = libdir / "lib.rs"
    existing = libfile.read_text() if libfile.exists() else ""
    extra = "\n".join(lib_rs_extra or [])
    if f"pub mod {functional};" not in existing:
        existing += f"\npub mod {functional};\n"
    if extra:
        existing += "\n" + extra + "\n"
    libfile.write_text(existing)
    return funcdir


def _make_maple2c_stub(root: Path, functional: str, all_levels: bool = True) -> None:
    """Create a stub maple2c .c source so the audit can read MAPLE2C_FLAGS.

    By default, all 5 levels (exc, vxc, fxc, kxc, lxc) are declared via
    XC_FLAGS_I_HAVE_*. all_levels=False writes only EXC.
    """
    maple2c = root / "libxc-master" / "src" / "maple2c" / "gga_exc"
    maple2c.mkdir(parents=True, exist_ok=True)
    flags = ("XC_FLAGS_I_HAVE_EXC | XC_FLAGS_I_HAVE_VXC | "
             "XC_FLAGS_I_HAVE_FXC | XC_FLAGS_I_HAVE_KXC | XC_FLAGS_I_HAVE_LXC"
             if all_levels else "XC_FLAGS_I_HAVE_EXC")
    body = f"""/* stub maple2c source for {functional} */
#define MAPLE2C_FLAGS ({flags})

#ifndef XC_DONT_COMPILE_EXC
/* exc body */
#endif

#ifndef XC_DONT_COMPILE_VXC
/* vxc body */
#endif

#ifndef XC_DONT_COMPILE_FXC
/* fxc body */
#endif

#ifndef XC_DONT_COMPILE_KXC
/* kxc body */
#endif

#ifndef XC_DONT_COMPILE_LXC
/* lxc unpolarized body */
#endif

/* polarized section: */
#ifndef XC_DONT_COMPILE_EXC
/* exc_pol body */
#endif

#ifndef XC_DONT_COMPILE_VXC
/* vxc_pol body */
#endif

#ifndef XC_DONT_COMPILE_FXC
/* fxc_pol body */
#endif

#ifndef XC_DONT_COMPILE_KXC
/* kxc_pol body */
#endif

#ifndef XC_DONT_COMPILE_LXC
/* lxc_pol body — pad to make this the largest block by padding lines below */
"""
    pad = "\n".join(f"/* lxc_pol pad line {i} */" for i in range(7000))
    body += pad + "\n#endif\n"
    (maple2c / f"{functional}.c").write_text(body)


class CanonicalListTests(unittest.TestCase):
    """Test 1 — canonical-list resolution returns 25 functionals incl. the 6 named in RESEARCH."""

    def test_canonical_list_resolves_25_functionals(self) -> None:
        """load_canonical_list() must return exactly 25 names with the 6 RESEARCH-named present."""
        canonical = agda.load_canonical_list()
        self.assertEqual(len(canonical), 25,
                         f"Expected 25 canonical functionals, got {len(canonical)}: {canonical}")
        named = {
            "gga_c_ft97", "gga_x_wpbeh", "gga_c_pbe_erf_gws",
            "gga_c_optc", "gga_c_q2d", "gga_c_revtca",
        }
        for n in named:
            self.assertIn(n, canonical, f"RESEARCH-named functional {n} missing from canonical list")


class AuditFunctionalTests(unittest.TestCase):
    """Tests 2-7 — audit_functional behavior across happy-path and gap fixtures."""

    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        # All fixture functionals get the maple2c stub with all 5 levels.
        self.functional = "gga_c_test1"
        _make_maple2c_stub(self.root, self.functional, all_levels=True)

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def _all_levels_files(self) -> list[str]:
        return [f"{lvl}_{spin}.rs"
                for lvl in agda.LEVELS
                for spin in agda.SPINS]

    def test_happy_path_all_levels_covered(self) -> None:
        """Test 2: all 10 (level, spin) tuples covered in a single sub-crate → status OK."""
        _make_subcrate(self.root, "kernel-gga-test", self.functional, self._all_levels_files())
        report = agda.audit_functional(self.functional, repo_root=self.root)
        self.assertEqual(report["status"], "OK", f"Expected OK, got {report}")
        covered = set(tuple(t) for t in report["covered"])
        expected = {(lvl, spin) for lvl in agda.LEVELS for spin in agda.SPINS}
        self.assertEqual(covered, expected,
                         f"Coverage mismatch: missing {expected - covered}, extra {covered - expected}")
        self.assertEqual(report["gaps"], [], f"Expected no gaps, got {report['gaps']}")

    def test_gap_orphan_file(self) -> None:
        """Test 3: a .rs file exists on disk but is NOT listed in mod.rs."""
        files = self._all_levels_files()
        # mod.rs lists everything except the lxc_pol_part0 file.
        funcdir = _make_subcrate(self.root, "kernel-gga-test", self.functional, files)
        # Now drop in an orphan file that isn't listed in mod.rs.
        (funcdir / "lxc_pol_part0_orphan.rs").write_text("// orphan stub\n")
        report = agda.audit_functional(self.functional, repo_root=self.root)
        self.assertEqual(report["status"], "GAP", f"Expected GAP, got {report}")
        kinds = [g["kind"] for g in report["gaps"]]
        self.assertIn("orphan_file", kinds, f"Expected orphan_file gap, got kinds={kinds}")
        orphan_gap = next(g for g in report["gaps"] if g["kind"] == "orphan_file")
        self.assertIn("lxc_pol_part0_orphan", orphan_gap["file"])

    def test_gap_commented_pub_mod(self) -> None:
        """Test 4: mod.rs contains `// pub mod kxc_pol;` → status GAP, kind commented_pub_mod."""
        files = self._all_levels_files()
        # Replace `pub mod kxc_pol;` with `// pub mod kxc_pol;`
        mod_lines = [f"pub mod {Path(f).stem};" for f in files]
        mod_lines = [(f"// {l}" if "kxc_pol" in l else l) for l in mod_lines]
        _make_subcrate(self.root, "kernel-gga-test", self.functional, files,
                       mod_rs_lines=mod_lines)
        report = agda.audit_functional(self.functional, repo_root=self.root)
        self.assertEqual(report["status"], "GAP", f"Expected GAP, got {report}")
        kinds = [g["kind"] for g in report["gaps"]]
        self.assertIn("commented_pub_mod", kinds, f"Expected commented_pub_mod gap, got kinds={kinds}")
        commented_gap = next(g for g in report["gaps"] if g["kind"] == "commented_pub_mod")
        self.assertEqual(commented_gap["module"], "kxc_pol")

    def test_gap_forbidden_gate(self) -> None:
        """Test 5: mod.rs contains `#[cfg(feature = "order-kxc")]` → status FORBIDDEN_GATE."""
        files = self._all_levels_files()
        mod_lines = []
        for f in files:
            stem = Path(f).stem
            if stem == "kxc_pol":
                mod_lines.append('#[cfg(feature = "order-kxc")]')
            mod_lines.append(f"pub mod {stem};")
        _make_subcrate(self.root, "kernel-gga-test", self.functional, files,
                       mod_rs_lines=mod_lines)
        report = agda.audit_functional(self.functional, repo_root=self.root)
        self.assertEqual(report["status"], "FORBIDDEN_GATE",
                         f"Expected FORBIDDEN_GATE, got {report}")
        kinds = [g["kind"] for g in report["gaps"]]
        self.assertIn("FORBIDDEN_GATE", kinds, f"Expected FORBIDDEN_GATE gap, got kinds={kinds}")
        gate_gap = next(g for g in report["gaps"] if g["kind"] == "FORBIDDEN_GATE")
        self.assertIn("order-kxc", gate_gap["line"])

    def test_gap_missing_lib_export(self) -> None:
        """Test 6: functional dir exists, but lib.rs has no `pub mod <functional>;` line."""
        files = self._all_levels_files()
        # Build the dir + mod.rs but DELIBERATELY do not export from lib.rs.
        subcrate = "kernel-gga-test"
        funcdir = self.root / "crates" / subcrate / "src" / self.functional
        funcdir.mkdir(parents=True, exist_ok=True)
        for f in files:
            (funcdir / f).write_text("// stub\n")
        mod_lines = [f"pub mod {Path(f).stem};" for f in files]
        (funcdir / "mod.rs").write_text("\n".join(mod_lines) + "\n")
        # Empty lib.rs (no pub mod entry):
        (self.root / "crates" / subcrate / "src" / "lib.rs").write_text("// no pub mod\n")
        report = agda.audit_functional(self.functional, repo_root=self.root)
        self.assertEqual(report["status"], "GAP", f"Expected GAP, got {report}")
        kinds = [g["kind"] for g in report["gaps"]]
        self.assertIn("missing_lib_export", kinds,
                      f"Expected missing_lib_export gap, got kinds={kinds}")
        export_gap = next(g for g in report["gaps"] if g["kind"] == "missing_lib_export")
        self.assertEqual(export_gap["functional"], self.functional)

    def test_multi_subcrate_aggregation(self) -> None:
        """Test 7: split coverage across two sub-crates is UNIONed → status OK."""
        # -1a contains exc_pol; -1b contains exc_unpol; rest in -1c.
        a_files = ["exc_pol.rs"]
        b_files = ["exc_unpol.rs"]
        c_files = [f"{lvl}_{spin}.rs" for lvl in agda.LEVELS for spin in agda.SPINS
                   if (lvl, spin) not in {("exc", "pol"), ("exc", "unpol")}]
        _make_subcrate(self.root, "kernel-gga-1a", self.functional, a_files)
        _make_subcrate(self.root, "kernel-gga-1b", self.functional, b_files)
        _make_subcrate(self.root, "kernel-gga-1c", self.functional, c_files)
        report = agda.audit_functional(self.functional, repo_root=self.root)
        self.assertEqual(report["status"], "OK", f"Expected OK, got {report}")
        covered = set(tuple(t) for t in report["covered"])
        expected = {(lvl, spin) for lvl in agda.LEVELS for spin in agda.SPINS}
        self.assertEqual(covered, expected, f"Coverage mismatch: missing {expected - covered}")
        self.assertGreaterEqual(len(report["subcrates"]), 3,
                                f"Expected aggregation across 3 sub-crates, got {report['subcrates']}")


if __name__ == "__main__":
    unittest.main(verbosity=2)
