#!/usr/bin/env python3
"""Audit per-functional derivative-order coverage for the 25 historically-deferred GGA functionals.

Phase 09 Plan 09-05 — script-driven audit per CONTEXT D-12.

For each of the 25 functionals whose maple2c-source `lxc_pol` body exceeds the
historical CubeCL proc-macro OOM threshold (~5,000 Rust lines, equivalent to
~6,000 maple2c source lines), this script:

  1. Walks every `crates/kernel-gga*/src/<functional>/` directory.
  2. Aggregates the (level, spin) tuples covered across all sub-crates.
  3. Detects gaps:
       - orphan_file: a `.rs` file on disk not listed in mod.rs.
       - commented_pub_mod: a `// pub mod ...;` line in mod.rs.
       - FORBIDDEN_GATE: a `#[cfg(feature = "order-...")]` attribute.
       - missing_lib_export: functional dir exists but no lib.rs `pub mod` entry.
       - missing_level_spin: maple2c source supports a (level, spin) tuple
         but no file (or _partN... file) exists for it on disk.
  4. Emits a JSON report (machine-readable) and a markdown report (human-readable).
  5. Exits non-zero under --strict if any gap remains.

The canonical list is regenerated dynamically from
`libxc-master/src/maple2c/gga_exc/<functional>.c`: a functional is canonical
iff the polarized LXC block (the second `#ifndef XC_DONT_COMPILE_LXC` block,
which contains the largest derivative computation) has a body of at least
6,000 lines. This rule reproduces the exact 25 functionals from
RESEARCH.md §"Why the 25 GGA Functionals Are Deferred".

Usage:
    python3 tools/audit_deferred_gga.py [--strict] \\
        [--json-out PATH] [--md-out PATH]

Exits:
    0 — all canonical functionals pass; no gaps.
    1 — at least one functional has status != "OK" (only with --strict).

Public API (used by tests):
    load_canonical_list(maple2c_root=...) -> list[str]
    audit_functional(name, repo_root=...) -> dict
    main() -> int
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
LEVELS: tuple[str, ...] = ("exc", "vxc", "fxc", "kxc", "lxc")
SPINS: tuple[str, ...] = ("unpol", "pol")

# (level)_(spin)[_partN_subname?].rs — anchor on full filename.
_FILE_RE = re.compile(
    r"^(?P<level>exc|vxc|fxc|kxc|lxc)_(?P<spin>unpol|pol)"
    r"(?:_part\d+(?:_[A-Za-z0-9]+)*)?\.rs$"
)
_PUB_MOD_RE = re.compile(r"^\s*pub\s+mod\s+(\w+)\s*;")
_COMMENTED_PUB_MOD_RE = re.compile(r"^\s*//\s*pub\s+mod\s+(\w+)\s*;")
_FORBIDDEN_GATE_RE = re.compile(r'#\[cfg\(\s*feature\s*=\s*"order-')
_MAPLE2C_FLAGS_RE = re.compile(
    r"#define\s+MAPLE2C_FLAGS\s*\((?P<body>[^)]*)\)"
)

# Threshold (in maple2c source lines) for the polarized LXC block.
# Empirically calibrated to reproduce the canonical 25-functional list
# documented in RESEARCH.md §"Why the 25 GGA Functionals Are Deferred":
#   thresh 5500: 28 functionals
#   thresh 5800: 26 functionals
#   thresh 6000: 25 functionals  ← canonical 25
#   thresh 6200: 25 functionals
#   thresh 6500: 25 functionals
#   thresh 6800: 23 functionals
LXC_POL_BODY_LINE_THRESHOLD = 6000


# ---------------------------------------------------------------------------
# Canonical-list resolution
# ---------------------------------------------------------------------------


def _measure_lxc_pol_body_lines(c_file: Path) -> int:
    """Return the line count of the polarized `XC_DONT_COMPILE_LXC` block in a maple2c .c file.

    Each maple2c GGA file has TWO `#ifndef XC_DONT_COMPILE_LXC` blocks: the
    first is unpolarized (smaller), the second is polarized (larger). We
    measure the second and return its body line count (excluding the #ifndef
    and matching #endif lines).

    Returns 0 if fewer than two such blocks exist (e.g., functional doesn't
    expose LXC at all).
    """
    try:
        text = c_file.read_text(errors="replace").splitlines()
    except OSError:
        return 0

    lxc_starts: list[int] = []
    for i, line in enumerate(text):
        stripped = line.lstrip()
        if "XC_DONT_COMPILE_LXC" in line and stripped.startswith("#"):
            # Match `#ifndef XC_DONT_COMPILE_LXC` (skip plain comments).
            if "#ifndef" in line or "#if" in line:
                lxc_starts.append(i)
    if len(lxc_starts) < 2:
        return 0

    pol_start = lxc_starts[1]
    depth = 1
    end = pol_start
    for i in range(pol_start + 1, len(text)):
        s = text[i].lstrip()
        if s.startswith("#if"):
            depth += 1
        elif s.startswith("#endif"):
            depth -= 1
            if depth == 0:
                end = i
                break
    return max(0, end - pol_start - 1)


def load_canonical_list(maple2c_root: Path | None = None) -> list[str]:
    """Return the canonical 25 deferred GGA functional names (sorted).

    Algorithm:
      For every `<name>.c` in `libxc-master/src/maple2c/gga_exc/`, compute
      the polarized-LXC block body line count. Functionals whose body
      exceeds LXC_POL_BODY_LINE_THRESHOLD (6000 maple2c lines) are deferred.

    Asserts cardinality 25 to catch translator-source drift early.
    """
    if maple2c_root is None:
        maple2c_root = REPO_ROOT / "libxc-master" / "src" / "maple2c"
    gga_exc = maple2c_root / "gga_exc"
    if not gga_exc.is_dir():
        raise FileNotFoundError(f"maple2c gga_exc directory not found: {gga_exc}")

    deferred: list[tuple[int, str]] = []
    for c_file in sorted(gga_exc.glob("*.c")):
        if "Zone.Identifier" in c_file.name:
            continue
        body = _measure_lxc_pol_body_lines(c_file)
        if body >= LXC_POL_BODY_LINE_THRESHOLD:
            deferred.append((body, c_file.stem))
    names = sorted(name for _, name in deferred)
    if len(names) != 25:
        raise AssertionError(
            f"Canonical list size drift: expected 25, got {len(names)}. "
            f"Names: {names}. Adjust LXC_POL_BODY_LINE_THRESHOLD if the "
            f"maple2c source has changed."
        )
    # RESEARCH.md sanity check.
    must_include = {
        "gga_c_ft97", "gga_x_wpbeh", "gga_c_pbe_erf_gws",
        "gga_c_optc", "gga_c_q2d", "gga_c_revtca",
    }
    missing = must_include - set(names)
    if missing:
        raise AssertionError(
            f"Canonical list missing RESEARCH-named functionals: {missing}"
        )
    return names


# ---------------------------------------------------------------------------
# Per-functional audit
# ---------------------------------------------------------------------------


def _classify_filename(filename: str) -> tuple[str, str] | None:
    """Return (level, spin) if filename matches a translator-emitted kernel file, else None."""
    m = _FILE_RE.match(filename)
    if not m:
        return None
    return (m.group("level"), m.group("spin"))


def _parse_mod_rs(mod_rs_path: Path) -> tuple[set[str], list[str], list[tuple[int, str]]]:
    """Parse mod.rs and return (uncommented_modules, commented_modules, forbidden_gate_lines).

    forbidden_gate_lines is a list of (line_number, line_text) tuples.
    """
    uncommented: set[str] = set()
    commented: list[str] = []
    forbidden: list[tuple[int, str]] = []
    if not mod_rs_path.is_file():
        return uncommented, commented, forbidden
    for lineno, line in enumerate(mod_rs_path.read_text(errors="replace").splitlines(), 1):
        if _FORBIDDEN_GATE_RE.search(line):
            forbidden.append((lineno, line.rstrip()))
        m = _PUB_MOD_RE.match(line)
        if m:
            uncommented.add(m.group(1))
            continue
        cm = _COMMENTED_PUB_MOD_RE.match(line)
        if cm:
            commented.append(cm.group(1))
    return uncommented, commented, forbidden


def _parse_lib_rs_modules(lib_rs_path: Path) -> tuple[set[str], list[str]]:
    """Parse a sub-crate lib.rs and return (uncommented_top_modules, commented_modules)."""
    uncommented: set[str] = set()
    commented: list[str] = []
    if not lib_rs_path.is_file():
        return uncommented, commented
    for line in lib_rs_path.read_text(errors="replace").splitlines():
        m = _PUB_MOD_RE.match(line)
        if m:
            uncommented.add(m.group(1))
            continue
        cm = _COMMENTED_PUB_MOD_RE.match(line)
        if cm:
            commented.append(cm.group(1))
    return uncommented, commented


def _maple2c_supported_levels(maple2c_root: Path, functional: str) -> set[str]:
    """Return the set of derivative levels declared by `MAPLE2C_FLAGS` in the functional's .c file.

    Looks for `#define MAPLE2C_FLAGS (XC_FLAGS_I_HAVE_EXC | ...)` and maps
    HAVE_<LEVEL> to the lowercase level name.
    """
    candidates = [
        maple2c_root / "gga_exc" / f"{functional}.c",
        maple2c_root / "gga_vxc" / f"{functional}.c",
    ]
    supported: set[str] = set()
    for c_file in candidates:
        if not c_file.is_file():
            continue
        text = c_file.read_text(errors="replace")
        m = _MAPLE2C_FLAGS_RE.search(text)
        if not m:
            continue
        body = m.group("body")
        for lvl in LEVELS:
            if f"XC_FLAGS_I_HAVE_{lvl.upper()}" in body or f"XC_FLAGS_HAVE_{lvl.upper()}" in body:
                supported.add(lvl)
        break
    if not supported:
        # Default: assume all 5 levels supported (safe — surfaces gaps, never hides them).
        return set(LEVELS)
    return supported


def _find_subcrate_dirs(repo_root: Path, functional: str) -> list[Path]:
    """Find every `crates/kernel-gga*/src/<functional>/` directory under repo_root."""
    crates_dir = repo_root / "crates"
    if not crates_dir.is_dir():
        return []
    matches: list[Path] = []
    for child in sorted(crates_dir.iterdir()):
        if not child.is_dir():
            continue
        if not (child.name.startswith("kernel-gga") or child.name.startswith("kernel-mgga")
                or child.name.startswith("kernel-lda")):
            # Guard against non-kernel crates; only kernel-gga* matters for this audit
            # but we tolerate subdirs from other families if they exist.
            pass
        # The audit's canonical scope is GGA — but we accept any kernel-* family
        # (the deferred list happens to be GGA-only).
        funcdir = child / "src" / functional
        if funcdir.is_dir():
            matches.append(funcdir)
    return matches


def audit_functional(name: str, repo_root: Path | None = None) -> dict:
    """Audit a single functional. Return a report dict.

    Report shape:
        {
            "name": str,
            "status": "OK" | "GAP" | "FORBIDDEN_GATE",
            "subcrates": list[str],         # sub-crate names containing the functional dir
            "covered": list[(level, spin)], # union of (level, spin) tuples observed
            "supported": list[(level, spin)],  # tuples supported by the maple2c source
            "gaps": [
                {"kind": "orphan_file", "file": str, "missing_from": str},
                {"kind": "commented_pub_mod", "module": str, "file": str},
                {"kind": "FORBIDDEN_GATE", "line": str, "file": str, "lineno": int},
                {"kind": "missing_lib_export", "functional": str, "subcrate": str},
                {"kind": "missing_level_spin", "level": str, "spin": str},
            ],
        }
    """
    if repo_root is None:
        repo_root = REPO_ROOT
    maple2c_root = repo_root / "libxc-master" / "src" / "maple2c"

    funcdirs = _find_subcrate_dirs(repo_root, name)
    subcrate_names = sorted(d.parents[1].name for d in funcdirs)

    covered: set[tuple[str, str]] = set()
    gaps: list[dict] = []
    has_forbidden_gate = False

    # 1. mod.rs / file analysis per sub-crate.
    for funcdir in funcdirs:
        mod_rs = funcdir / "mod.rs"
        uncommented, commented, forbidden = _parse_mod_rs(mod_rs)
        for lineno, line in forbidden:
            has_forbidden_gate = True
            gaps.append({
                "kind": "FORBIDDEN_GATE",
                "line": line,
                "file": str(mod_rs.relative_to(repo_root)),
                "lineno": lineno,
            })
        for cm in commented:
            gaps.append({
                "kind": "commented_pub_mod",
                "module": cm,
                "file": str(mod_rs.relative_to(repo_root)),
            })
        # Walk .rs files in funcdir (excluding mod.rs).
        for rs in sorted(funcdir.glob("*.rs")):
            if rs.name == "mod.rs":
                continue
            stem = rs.stem
            cls = _classify_filename(rs.name)
            if cls is None:
                # Unrecognized filename — treat as orphan if not listed.
                if stem not in uncommented:
                    gaps.append({
                        "kind": "orphan_file",
                        "file": str(rs.relative_to(repo_root)),
                        "missing_from": str(mod_rs.relative_to(repo_root)),
                    })
                continue
            level, spin = cls
            covered.add((level, spin))
            if stem not in uncommented:
                gaps.append({
                    "kind": "orphan_file",
                    "file": str(rs.relative_to(repo_root)),
                    "missing_from": str(mod_rs.relative_to(repo_root)),
                })

    # 2. lib.rs export check across the SAME sub-crates that contain a directory
    #    for this functional.
    found_in_lib = False
    for funcdir in funcdirs:
        lib_rs = funcdir.parents[0] / "lib.rs"
        uncommented, _ = _parse_lib_rs_modules(lib_rs)
        if name in uncommented:
            found_in_lib = True
            break
    if funcdirs and not found_in_lib:
        gaps.append({
            "kind": "missing_lib_export",
            "functional": name,
            "subcrate": funcdirs[0].parents[1].name,
        })

    # 3. supported levels per maple2c — flag missing (level, spin) tuples.
    supported_levels = _maple2c_supported_levels(maple2c_root, name)
    supported_tuples: set[tuple[str, str]] = set()
    for lvl in LEVELS:
        if lvl in supported_levels:
            for spin in SPINS:
                supported_tuples.add((lvl, spin))
    missing_tuples = supported_tuples - covered
    for lvl, spin in sorted(missing_tuples):
        gaps.append({
            "kind": "missing_level_spin",
            "level": lvl,
            "spin": spin,
        })

    # 4. Status precedence: FORBIDDEN_GATE > GAP > OK.
    if has_forbidden_gate:
        status = "FORBIDDEN_GATE"
    elif gaps:
        status = "GAP"
    else:
        status = "OK"

    return {
        "name": name,
        "status": status,
        "subcrates": subcrate_names,
        "covered": sorted([list(t) for t in covered]),
        "supported": sorted([list(t) for t in supported_tuples]),
        "gaps": gaps,
    }


# ---------------------------------------------------------------------------
# Report rendering
# ---------------------------------------------------------------------------


def _render_markdown(reports: list[dict], canonical: list[str]) -> str:
    counts: dict[str, int] = defaultdict(int)
    for r in reports:
        counts[r["status"]] += 1
    lines: list[str] = []
    lines.append("# Phase 09 Plan 09-05 — Deferred-GGA Coverage Audit")
    lines.append("")
    lines.append(
        "This report is **auto-generated** by `tools/audit_deferred_gga.py` per "
        "CONTEXT D-12 (script-driven audit). Do not edit by hand. Re-run the script "
        "after any translator/regen pass to refresh."
    )
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    lines.append(f"- Canonical functional count: **{len(canonical)}** (must be 25)")
    lines.append(f"- OK: **{counts.get('OK', 0)}**")
    lines.append(f"- GAP: **{counts.get('GAP', 0)}**")
    lines.append(f"- FORBIDDEN_GATE: **{counts.get('FORBIDDEN_GATE', 0)}**")
    lines.append("")
    lines.append("## Coverage Status")
    lines.append("")
    lines.append("| Functional | Status | Sub-crates | Covered tuples | Gaps |")
    lines.append("|---|---|---|---|---|")
    for r in reports:
        cov = ", ".join(f"{lvl}/{spin}" for lvl, spin in r["covered"]) or "—"
        gap_count = len(r["gaps"])
        sc = ", ".join(r["subcrates"]) or "—"
        lines.append(
            f"| `{r['name']}` | `{r['status']}` | {sc} | {cov} | {gap_count} |"
        )
    lines.append("")
    # Per-functional details for any non-OK report.
    detail_reports = [r for r in reports if r["status"] != "OK"]
    if detail_reports:
        lines.append("## Detailed Gap Reports")
        lines.append("")
        for r in detail_reports:
            lines.append(f"### `{r['name']}` — {r['status']}")
            lines.append("")
            for gap in r["gaps"]:
                kind = gap["kind"]
                detail = ", ".join(f"{k}={v}" for k, v in gap.items() if k != "kind")
                lines.append(f"- **{kind}**: {detail}")
            lines.append("")
    else:
        lines.append("## Detailed Gap Reports")
        lines.append("")
        lines.append("All 25 canonical functionals are at status `OK`. No gaps.")
        lines.append("")
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description=("Audit per-functional derivative-order coverage for "
                     "the 25 historically-deferred GGA functionals."),
    )
    parser.add_argument(
        "--json-out",
        default="log/09-05-deferred-gga-audit.json",
        help="Path for the JSON audit report (default: %(default)s)",
    )
    parser.add_argument(
        "--md-out",
        default=".planning/phases/09-reduce-kernel-build-time/09-05-DEFERRED-GGA-AUDIT.md",
        help="Path for the markdown audit report (default: %(default)s)",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit non-zero if any functional has status != \"OK\".",
    )
    parser.add_argument(
        "--repo-root",
        default=str(REPO_ROOT),
        help="Repo root path (default: %(default)s)",
    )
    args = parser.parse_args(argv)

    repo_root = Path(args.repo_root).resolve()
    canonical = load_canonical_list(repo_root / "libxc-master" / "src" / "maple2c")
    reports = [audit_functional(name, repo_root=repo_root) for name in canonical]

    json_path = Path(args.json_out)
    if not json_path.is_absolute():
        json_path = repo_root / json_path
    json_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "canonical_count": len(canonical),
        "canonical": canonical,
        "reports": reports,
    }
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")

    md_path = Path(args.md_out)
    if not md_path.is_absolute():
        md_path = repo_root / md_path
    md_path.parent.mkdir(parents=True, exist_ok=True)
    md_path.write_text(_render_markdown(reports, canonical))

    not_ok = [r for r in reports if r["status"] != "OK"]
    print(f"Audit: {len(reports)} functionals — "
          f"OK={len(reports) - len(not_ok)}, "
          f"GAP={sum(1 for r in not_ok if r['status'] == 'GAP')}, "
          f"FORBIDDEN_GATE={sum(1 for r in not_ok if r['status'] == 'FORBIDDEN_GATE')}.")
    print(f"JSON: {json_path}")
    print(f"Markdown: {md_path}")
    if args.strict and not_ok:
        print(f"--strict: {len(not_ok)} functional(s) failing audit.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
