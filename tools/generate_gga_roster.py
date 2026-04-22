#!/usr/bin/env python3
"""Enumerate compiled GGA functionals across crates/kernel-gga-*.

For each functional module directory, determines:
  * name        — module name (e.g. "gga_c_pbe")
  * batch       — sub-crate batch suffix (e.g. "4g" for libxc_kernel_gga_4g)
  * libxc_id    — integer ID from src/meta/generated.rs, or "UNKNOWN"
  * completeness — "FULL" (all 10 arms), "VXC_ONLY" (vxc/fxc/kxc/lxc
                    without exc), or "PARTIAL" (anything else — skipped)
  * has_exc     — 1 if exc_unpol/exc_pol modules exist, else 0
  * scalars     — comma-separated list of per-functional f64 scalar
                    argument names (between the output array and the
                    trailing dens_threshold/zeta_threshold pair). Empty
                    for functionals whose kernels take only thresholds.

Emits one tab-separated row per FULL/VXC_ONLY functional.
PARTIAL functionals (incomplete translations) are silently skipped so
that the roster aligns with `dispatch_gga`'s supported set.
"""
from __future__ import annotations

import re
from pathlib import Path

WORKSPACE = Path(__file__).resolve().parent.parent
GGA_CRATES = sorted(
    p for p in (WORKSPACE / "crates").glob("kernel-gga-*")
    if p.is_dir() and p.name != "kernel-gga"
)


def load_id_map() -> dict[str, int]:
    """Parse src/meta/generated.rs and build `{lowercase_name: id}`.

    Each FunctionalMeta block starts with `pub(crate) const XC_NAME: FunctionalMeta = FunctionalMeta {`
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


# Template kernels: the kernel module name does not match a single libxc
# functional name (the kernel backs multiple libxc IDs via varying
# ext_params defaults). For Phase 4, we route each template to its
# *primary* libxc ID. Other IDs backed by the same template are left
# `UnsupportedFunctional` until a follow-up plan adds per-variant
# parameter plumbing.
#
# `gga_x_herman` maps to libxc ID 104, which is on the removed list
# (see `libxc-master/src/xc_funcs_removed.h`); we leave it as UNKNOWN
# so the roster pipeline flags it and the dispatch layer omits it.
TEMPLATE_ID_OVERRIDES = {
    "gga_x_dk87":          111,   # gga_x_dk87_r1 (R2 uses same template)
    "gga_x_vmt":           70,    # gga_x_vmt_ge  (VMT_PBE=71 same template)
    "gga_x_vmt84":         68,    # gga_x_vmt84_ge
    "gga_x_kt":            145,   # gga_x_kt1 (exchange-only form)
    "gga_x_s12":           495,   # gga_x_s12g (hybrids skip this row)
    "hyb_gga_x_cam_s12":   646,   # hyb_gga_x_cam_s12g
    "gga_k_tflw":          52,    # gga_k_tfvw
    "gga_k_pw86":          515,   # gga_k_fr_pw86
    "gga_k_mpbe":          616,   # gga_k_pbe2
    "gga_k_pg":            219,   # gga_k_pg1
    # gga_x_herman -> 104 (removed) — intentionally absent; see above.
}


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
    # Split on commas at top-level (argument signatures never contain
    # nested commas inside generics here, so a naive split is fine).
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
    # VXC-only: 8 files, none of them exc.
    no_exc = standard - {"exc_unpol.rs", "exc_pol.rs"}
    if present == no_exc:
        return ("VXC_ONLY", 0)
    return None  # PARTIAL


def find_signature_source(func_dir: Path) -> str | None:
    for candidate in ("exc_unpol.rs", "vxc_unpol.rs", "exc_pol.rs", "vxc_pol.rs"):
        p = func_dir / candidate
        if p.exists():
            return p.read_text()
    return None


def main() -> None:
    id_map = load_id_map()
    rows: list[tuple[str, str, str, str, int, str]] = []

    for crate_dir in GGA_CRATES:
        batch = crate_dir.name.replace("kernel-gga-", "")
        src_dir = crate_dir / "src"
        for func_dir in sorted(src_dir.iterdir()):
            if not func_dir.is_dir():
                continue
            classification = classify(func_dir)
            if classification is None:
                continue
            completeness, has_exc = classification
            name = func_dir.name
            # Primary lookup: direct name match (most kernels). Templates
            # fall through to TEMPLATE_ID_OVERRIDES.
            if name in id_map:
                libxc_id = id_map[name]
            elif name in TEMPLATE_ID_OVERRIDES:
                libxc_id = TEMPLATE_ID_OVERRIDES[name]
            else:
                libxc_id = "UNKNOWN"
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


if __name__ == "__main__":
    main()
