#!/usr/bin/env python3
"""Phase 11 / 11-12 (G-2): feature-gate the umbrella's per-functional kernel
deps by family (Path A).

Transforms root Cargo.toml so each `libxc-kernel-<func>` path-dep under
crates/kernels/{lda,gga,mgga}/ becomes `optional = true`, and adds a
`[features]` section with `oracle-lda` / `oracle-gga` / `oracle-mgga` grouping
each family plus `default = [all three]`.

`default = [all three]` means a bare `cargo build -p libxc_rs` activates every
family exactly as before this change — the default umbrella build is preserved
by construction. A `--no-default-features --features oracle-mgga` build resolves
ONLY the MGGA kernels (provable via `cargo tree`), which is the memory-safe
family-chunked oracle path G-2 needs.

`libxc-kernel-math` is intentionally left non-optional (always required).

Idempotent: re-running is a no-op (already-optional lines and an existing
[features] block are detected and skipped).
"""
from __future__ import annotations
import re
import sys
from pathlib import Path

CARGO = Path(__file__).resolve().parent.parent / "Cargo.toml"

# libxc-kernel-<func> = { path = "crates/kernels/<fam>/<dir>" }   (fam in lda/gga/mgga)
DEP_RE = re.compile(
    r'^(?P<name>libxc-kernel-[A-Za-z0-9_]+) = \{ path = "crates/kernels/'
    r'(?P<fam>lda|gga|mgga)/[^"]+"(?P<opt>, optional = true)? \}\s*$'
)

FAMS = ("lda", "gga", "mgga")


def main() -> int:
    text = CARGO.read_text()
    lines = text.splitlines(keepends=False)

    # Operate only on the [dependencies] section (up to [dev-dependencies]).
    dep_start = next(i for i, l in enumerate(lines) if l.strip() == "[dependencies]")
    dev_start = next(i for i, l in enumerate(lines) if l.strip() == "[dev-dependencies]")

    fam_members: dict[str, list[str]] = {f: [] for f in FAMS}
    transformed = 0
    out: list[str] = []

    for i, line in enumerate(lines):
        m = DEP_RE.match(line) if dep_start < i < dev_start else None
        if m:
            name, fam = m.group("name"), m.group("fam")
            fam_members[fam].append(name)
            if m.group("opt"):
                out.append(line)  # already optional — leave as-is
            else:
                out.append(line.replace(" }", ", optional = true }", 1))
                transformed += 1
        else:
            out.append(line)

    for fam in FAMS:
        if len(fam_members[fam]) != {"lda": 43, "gga": 131, "mgga": 106}[fam]:
            print(f"ABORT: {fam} matched {len(fam_members[fam])} deps (expected sentinel)",
                  file=sys.stderr)
            return 2

    # Build the [features] block.
    feat: list[str] = ["", "[features]",
                       "# Phase 11 / 11-12 (G-2, Path A): family-chunked oracle builds.",
                       "# default activates all families -> bare `cargo build -p libxc_rs`",
                       "# is unchanged. `--no-default-features --features oracle-<fam>`",
                       "# resolves ONLY that family's kernels (memory-safe; provable via",
                       "# `cargo tree`). The umbrella SOURCE cfg-gating that lets a single",
                       "# family actually COMPILE is the deferred follow-up (see",
                       "# .planning/phases/11-splitter-v2-unified-5k-cap/11-12-ORACLE-F32-LOG.md).",
                       'default = ["oracle-lda", "oracle-gga", "oracle-mgga"]']
    for fam in FAMS:
        feat.append("")
        feat.append(f'oracle-{fam} = [')
        for name in fam_members[fam]:
            feat.append(f'    "dep:{name}",')
        feat.append("]")

    if any(l.strip() == "[features]" for l in out):
        print("ABORT: [features] already present — manual review needed", file=sys.stderr)
        return 3

    # Insert the [features] block immediately before [dev-dependencies].
    dev_idx = next(i for i, l in enumerate(out) if l.strip() == "[dev-dependencies]")
    out[dev_idx:dev_idx] = feat + [""]

    CARGO.write_text("\n".join(out) + "\n")
    print(f"transformed {transformed} deps to optional "
          f"(lda={len(fam_members['lda'])} gga={len(fam_members['gga'])} "
          f"mgga={len(fam_members['mgga'])}); added [features]")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
