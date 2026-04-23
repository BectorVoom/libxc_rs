#!/usr/bin/env python3
"""Enumerate compiled MGGA functionals across crates/kernel-mgga-*.

For each functional module directory, determines:
  * name        — module name (e.g. "mgga_x_tpss")
  * batch       — sub-crate batch suffix (e.g. "33" for libxc_kernel_mgga_33)
  * libxc_id    — integer ID from src/meta/generated.rs, or "UNKNOWN"
  * completeness — "FULL" (all 10 arms) or "VXC_ONLY" (8 arms without exc)
  * has_exc     — 1 if exc_unpol/exc_pol modules exist, else 0
  * scalars     — comma-separated list of per-functional f64 scalar
                    argument names (between the output array and the
                    trailing dens_threshold/zeta_threshold pair). Empty
                    for functionals whose kernels take only thresholds.

Emits one tab-separated row per FULL/VXC_ONLY functional.
PARTIAL functionals (incomplete translations, commented-out in lib.rs,
or split-file-based) are silently skipped so that the roster aligns
with dispatch_mgga's supported set. Modules commented out in lib.rs
are never scanned.

See `tools/generate_gga_roster.py` for the GGA analog; this script is
the direct MGGA mirror (W6) with the broader input-buffer set
(rho + sigma + lapl + tau).
"""
from __future__ import annotations

import re
from pathlib import Path

WORKSPACE = Path(__file__).resolve().parent.parent
MGGA_CRATES = sorted(
    p for p in (WORKSPACE / "crates").glob("kernel-mgga-*")
    if p.is_dir() and p.name != "kernel-mgga"
)


def load_id_map() -> dict[str, int]:
    """Parse src/meta/generated.rs and build `{lowercase_name: id}`.

    Each FunctionalMeta block starts with
    `pub(crate) const XC_NAME: FunctionalMeta = FunctionalMeta {`
    and has `id: FunctionalId(N),` near the top. We map NAME (lowercased,
    `XC_` prefix stripped) to N.
    """
    text = (WORKSPACE / "src/meta/generated.rs").read_text()
    ids: dict[str, int] = {}
    pattern = re.compile(
        r"pub\(crate\)\s+const\s+XC_([A-Z0-9_]+):\s*FunctionalMeta[^}]+?"
        r"id:\s*FunctionalId\((\d+)\)",
        re.DOTALL,
    )
    for m in pattern.finditer(text):
        name = m.group(1).lower()
        ids[name] = int(m.group(2))
    return ids


SIG_RE = re.compile(
    r"#\[cube\(launch_unchecked\)\][\s\S]+?"
    r"pub\s+fn\s+\w+\(([\s\S]+?)\)\s*\{",
    re.MULTILINE,
)


def extract_scalars(source: str) -> list[str]:
    """Return the per-functional scalar `f64` argument names.

    Skip the standard `dens_threshold, zeta_threshold` pair — every kernel
    accepts those at the tail. Skip any `&Array<f64>` / `&mut Array<f64>`
    arrays — those are IO buffers threaded by the dispatch layer.
    """
    m = SIG_RE.search(source)
    if not m:
        return []
    body = m.group(1)
    scalars: list[str] = []
    for arg in [s.strip() for s in body.split(",") if s.strip()]:
        if ":" not in arg:
            continue
        name, ty = [s.strip() for s in arg.split(":", 1)]
        if ty != "f64":
            continue
        if name in ("dens_threshold", "zeta_threshold"):
            continue
        scalars.append(name)
    return scalars


def classify(func_dir: Path) -> tuple[str, int] | None:
    """Return (completeness, has_exc_flag) or None if PARTIAL / empty."""
    files = {p.name for p in func_dir.glob("*.rs")}
    standard = {
        "exc_unpol.rs",
        "exc_pol.rs",
        "vxc_unpol.rs",
        "vxc_pol.rs",
        "fxc_unpol.rs",
        "fxc_pol.rs",
        "kxc_unpol.rs",
        "kxc_pol.rs",
        "lxc_unpol.rs",
        "lxc_pol.rs",
    }
    present = files & standard
    if len(present) == 10:
        return ("FULL", 1)
    no_exc = standard - {"exc_unpol.rs", "exc_pol.rs"}
    if present == no_exc:
        return ("VXC_ONLY", 0)
    return None  # PARTIAL (split-file or incomplete)


def find_signature_source(func_dir: Path) -> str | None:
    for candidate in ("exc_unpol.rs", "vxc_unpol.rs", "exc_pol.rs", "vxc_pol.rs"):
        p = func_dir / candidate
        if p.exists():
            return p.read_text()
    return None


def main() -> None:
    id_map = load_id_map()
    rows: list[tuple[str, str, str, str, int, str]] = []

    for crate_dir in MGGA_CRATES:
        batch = crate_dir.name.replace("kernel-mgga-", "")
        lib_rs = crate_dir / "src/lib.rs"
        if not lib_rs.exists():
            continue
        # Only scan modules that are uncommented in lib.rs (commented-out
        # modules are deferred and must NOT be dispatched).
        active_mods: set[str] = set()
        for line in lib_rs.read_text().splitlines():
            stripped = line.strip()
            if stripped.startswith("//"):
                continue
            m = re.match(r"pub mod\s+(mgga_[a-z0-9_]+|hyb_mgga_[a-z0-9_]+);", stripped)
            if m:
                active_mods.add(m.group(1))
        src_dir = crate_dir / "src"
        for func_dir in sorted(src_dir.iterdir()):
            if not func_dir.is_dir():
                continue
            if func_dir.name not in active_mods:
                continue
            classification = classify(func_dir)
            if classification is None:
                continue
            completeness, has_exc = classification
            name = func_dir.name
            libxc_id = id_map.get(name, "UNKNOWN")
            sig_src = find_signature_source(func_dir)
            scalars = extract_scalars(sig_src) if sig_src else []
            rows.append((
                name,
                batch,
                str(libxc_id),
                completeness,
                has_exc,
                ",".join(scalars),
            ))

    # Sort for deterministic output: by libxc ID (UNKNOWN at end), then name.
    rows.sort(key=lambda r: (
        int(r[2]) if r[2] != "UNKNOWN" else 10_000,
        r[0],
    ))

    for row in rows:
        print("\t".join(str(x) for x in row))
    print(f"# total_compiled: {len(rows)}")


if __name__ == "__main__":
    main()
