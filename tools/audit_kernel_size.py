#!/usr/bin/env python3
"""Audit per-file kernel line counts against the Phase 11 D-LOCK-B 5,000-line cap.

Phase 11 P11-INV-2 — every `.rs` file under `crates/kernels/` must be ≤ 5,000 lines.

This script walks `crates/kernels/**/*.rs`, counts lines per file, sorts by line
count descending, and reports:
  - Total file count
  - Maximum line count and path
  - Number of files exceeding the cap (KERNEL_LINE_CAP)
  - Optional JSON or markdown emission for downstream tooling

Build env source of truth: `.cargo/config.toml` (D-08/D-09) — this script does
NOT set `CARGO_BUILD_JOBS`, does NOT pass `--jobs`, does NOT touch
`RUST_MIN_STACK`. It is a pure filesystem audit.

Usage:
    python3 tools/audit_kernel_size.py [--strict] \\
        [--json-out PATH] [--md-out PATH]

Exits:
    0 — clean OR no `--strict` flag.
    1 — at least one file exceeds KERNEL_LINE_CAP AND `--strict` is set.

Public API (used by tests):
    audit_kernel_size(repo_root=...) -> dict
    main() -> int
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
KERNEL_LINE_CAP = 5000  # Phase 11 D-LOCK-B hard cap.


def _count_lines(path: Path) -> int:
    """Return the number of newline-terminated lines in `path`.

    Uses `wc -l` semantics (counts `\\n` occurrences). A file without a
    trailing newline still counts the final partial line.
    """
    try:
        with path.open("rb") as fh:
            data = fh.read()
    except OSError:
        return 0
    if not data:
        return 0
    count = data.count(b"\n")
    if not data.endswith(b"\n"):
        count += 1
    return count


def audit_kernel_size(repo_root: Path = REPO_ROOT) -> dict:
    """Walk `<repo_root>/crates/kernels/**/*.rs` and return a size report.

    Returns a dict with:
        total_files: int
        oversized:   list[{path: str (rel to repo_root), lines: int}]  sorted desc
        max_lines:   int   (0 if no files)
        max_path:    str   ("" if no files)
        cap:         int   (KERNEL_LINE_CAP)
    """
    kernels_root = repo_root / "crates" / "kernels"
    rs_paths = sorted(kernels_root.rglob("*.rs"))
    files: list[tuple[Path, int]] = [(p, _count_lines(p)) for p in rs_paths]

    files.sort(key=lambda pair: pair[1], reverse=True)
    oversized = [
        {"path": str(p.relative_to(repo_root)), "lines": n}
        for p, n in files
        if n > KERNEL_LINE_CAP
    ]

    if files:
        max_path, max_lines = files[0]
        max_path_str = str(max_path.relative_to(repo_root))
    else:
        max_path_str = ""
        max_lines = 0

    return {
        "total_files": len(files),
        "oversized": oversized,
        "max_lines": max_lines,
        "max_path": max_path_str,
        "cap": KERNEL_LINE_CAP,
    }


def _format_md(report: dict, top_n: int = 20) -> str:
    head = report["oversized"][:top_n]
    lines = [
        f"# Phase 11 — Kernel-size Audit (cap = {report['cap']} lines)",
        "",
        f"Total `.rs` files under `crates/kernels/`: **{report['total_files']}**",
        f"Files exceeding cap: **{len(report['oversized'])}**",
        f"Maximum file size: **{report['max_lines']} lines** at `{report['max_path']}`",
        "",
        "## Top offenders",
        "",
        "| Lines | Path |",
        "|-------|------|",
    ]
    for entry in head:
        lines.append(f"| {entry['lines']} | `{entry['path']}` |")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Audit per-file kernel line counts (P11-INV-2)."
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit 1 if any file exceeds the cap.",
    )
    parser.add_argument("--json-out", type=Path, default=None)
    parser.add_argument("--md-out", type=Path, default=None)
    args = parser.parse_args()

    report = audit_kernel_size()

    print(
        f"Files >5000 lines: {len(report['oversized'])} "
        f"(max={report['max_lines']} lines at {report['max_path']})"
    )

    if args.json_out is not None:
        args.json_out.write_text(json.dumps(report, indent=2) + "\n")
    if args.md_out is not None:
        args.md_out.write_text(_format_md(report))

    if args.strict and report["oversized"]:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
