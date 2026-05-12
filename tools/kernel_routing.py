"""Shared helper: which functionals are routed by `<X>Functional::from_id`.

Translators and the post-hoc demoter need the same source-of-truth set of
"routed" functional names so that:

  * `tools/translate_{lda_v2,gga,mgga}.py` emit `#[cube]` (no host-side
    launch wrapper) for unrouted functionals at regen time.
  * `tools/demote_unrouted_kernels.py` can clean up any in-tree
    launch_unchecked attributes that pre-date the routing-aware emit.

The "routed" set is derived from `src/model/{family}_functional.rs`:

  * GGA / MGGA use a `Self::Variant => "snake_name"` arrow map. Parse the
    string literal directly.
  * LDA uses `<id> => Ok(Self::Variant)` arms with no inline snake_case;
    derive plausible snake-case candidates and intersect with the actual
    on-disk crate dirs (LDA naming is ambiguous around letter-digit
    boundaries — `LdaCVwn1` could be `lda_c_vwn1` or `lda_c_vwn_1`).

The implementation was lifted from `demote_unrouted_kernels.py` and is the
single source of truth. That script imports from here; translators do the
same. Do not duplicate the parsing logic.
"""

from __future__ import annotations

import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = REPO_ROOT / "crates"


def variant_to_snake_candidates(variant: str) -> list[str]:
    """Generate all plausible snake_case names for a CamelCase variant.

    Returns multiple candidates because letter-digit boundaries are
    ambiguous (`LdaCVwn1` → `lda_c_vwn1` or `lda_c_vwn_1`). The caller
    intersects with on-disk dirs to pick the actual name.
    """
    base = re.sub(r"([A-Z])", r"_\1", variant).lower().lstrip("_")
    boundary_re = re.compile(r"(?<=[a-z])(?=[0-9])|(?<=[0-9])(?=[a-z])")
    boundaries = [m.start() for m in boundary_re.finditer(base)]
    n = len(boundaries)
    candidates: set[str] = set()
    for mask in range(2 ** n):
        out = []
        for i, ch in enumerate(base):
            for k, b in enumerate(boundaries):
                if i == b and (mask >> k) & 1:
                    out.append("_")
            out.append(ch)
        candidates.add("".join(out))
    candidates.add(base)

    # `LdaXc1dEhwlrg{1,2,3}` share a dir without the trailing digit suffix.
    extras: set[str] = set()
    for c in candidates:
        if "ehwlrg" in c:
            stripped = re.sub(r"_?[0-9]+$", "", c)
            extras.add(stripped)
    candidates.update(extras)
    return list(candidates)


def _model_path(family: str) -> Path:
    return REPO_ROOT / "src" / "model" / f"{family}_functional.rs"


def collect_func_dirs(family: str) -> dict[str, list[Path]]:
    """{funcname: [path, ...]} for every `crates/kernels/<family>*/src/<func>/`.

    Glob matches the post-`crates/kernel-* → crates/kernels/*` layout move.
    """
    out: dict[str, list[Path]] = {}
    for crate in sorted(CRATES_DIR.glob(f"kernels/{family}*")):
        src = crate / "src"
        if not src.is_dir():
            continue
        for entry in sorted(src.iterdir()):
            if not entry.is_dir():
                continue
            out.setdefault(entry.name, []).append(entry)
    return out


def parse_routed_funcnames(rs_path: Path,
                           known_dirs: set[str] | None = None) -> set[str]:
    """Read a model file and return the set of routed snake_case funcnames.

    GGA/MGGA: read `Self::Variant => "name"` arrows directly.
    LDA: derive candidates from `Ok(Self::Variant)` arms and intersect
    with `known_dirs`.
    """
    text = rs_path.read_text()
    out: set[str] = set()
    for m in re.finditer(r'Self::\w+\s*=>\s*"([a-z0-9_]+)"', text):
        out.add(m.group(1))
    if out:
        return out

    if known_dirs is None:
        known_dirs = set()
    for m in re.finditer(r"=>\s*Ok\(Self::([A-Z][A-Za-z0-9]+)\)", text):
        variant = m.group(1)
        for cand in variant_to_snake_candidates(variant):
            if cand in known_dirs:
                out.add(cand)
                break
        else:
            # No match — record the first candidate so the caller can flag
            # it instead of silently misclassifying as unrouted.
            out.add(variant_to_snake_candidates(variant)[0])
    return out


def routed_funcnames(family: str,
                     known_dirs: set[str] | None = None) -> set[str]:
    """Return the set of routed snake_case funcnames for a family.

    `family` is one of {"lda", "gga", "mgga"}. For LDA, callers should
    pass `known_dirs` (the set of on-disk functional dir names) so the
    variant→snake-case disambiguation finds the actual dirs. If
    `known_dirs` is None, LDA falls back to the first candidate per
    variant — fine for runtime checks but may misclassify edge cases.
    """
    if family not in {"lda", "gga", "mgga"}:
        raise ValueError(f"Unknown family: {family!r} (expected lda|gga|mgga)")
    return parse_routed_funcnames(_model_path(family), known_dirs)


# Module-level cache so translators don't re-parse on every kernel emit.
_routed_cache: dict[str, set[str]] = {}


def cached_routed_funcnames(family: str) -> set[str]:
    """Cached `routed_funcnames(family, known_dirs=collect_func_dirs)`.

    Use this from translators where the same routed set is consulted once
    per emitted kernel. For LDA, `known_dirs` is auto-derived from the
    in-tree crate layout so variant disambiguation works.
    """
    if family not in _routed_cache:
        known = set(collect_func_dirs(family))
        _routed_cache[family] = routed_funcnames(family, known)
    return _routed_cache[family]


if __name__ == "__main__":
    # CLI smoke test: report routed counts per family.
    for fam in ("lda", "gga", "mgga"):
        known = set(collect_func_dirs(fam))
        routed = routed_funcnames(fam, known)
        unrouted = sorted(name for name in known if name not in routed)
        print(f"{fam.upper()}: routed={len(routed)} dirs={len(known)} unrouted={len(unrouted)}")
