#!/usr/bin/env python3
"""Demote `#[cube(launch_unchecked)]` -> `#[cube]` for kernels that no
runtime dispatch path ever invokes.

Background:
- `crates/kernel-{lda,gga,mgga}-*/` collectively contain ~3000+
  `#[cube(launch_unchecked)]` entry-kernel decls.
- The runtime dispatch (`src/eval/dispatch.rs`,
  `src/eval/{gga,mgga}_dispatch/*`) only routes a small fraction:
    * LDA: 37 routed of 41 functionals (4 deferred)
    * GGA: 105 routed of 256 functionals (rest unrouted)
    * MGGA: 25 routed of 146 functionals (rest unrouted)
- Per `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` Anti-pattern 1
  ("Every Helper Is Launchable"), the launch-wrapper boilerplate emitted
  for unreachable kernels is pure compile-time waste.

Strategy: scan each translator output module in
`crates/kernel-{lda,gga,mgga}-*/src/<func>/` for whether `<func>` is in
the dispatch routing table. If NOT routed, demote every
`#[cube(launch_unchecked)]` line to `#[cube]` for that functional.

This is the same surgery as
- tools/shrink_part_fanout.py (1396 _partN files, scope = split helpers)
- tools/demote_deferred_lda_fanout.py (38 deferred-LDA launches)

This script generalises to all unrouted functionals across all 3 families.
The routing tables are derived live from `src/model/{lda,gga,mgga}_functional.rs`
so this stays correct as routing changes.

Usage:
    python3 tools/demote_unrouted_kernels.py [--dry-run]
"""

import re
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
CRATES_DIR = REPO_ROOT / "crates"
LAUNCH_LINE_RE = re.compile(r"^#\[cube\(launch_unchecked\)\]\s*$", re.MULTILINE)


def variant_to_snake_candidates(variant: str) -> list[str]:
    """Convert a CamelCase variant name to all plausible snake_case dir
    names. Returns a list because the conversion is ambiguous around
    digit boundaries:
      - `LdaCVwn1`      -> `lda_c_vwn1` OR `lda_c_vwn_1`
      - `LdaCW20`       -> `lda_c_w20` OR `lda_c_w_20`
      - `LdaXc1dEhwlrg1`-> `lda_xc_1d_ehwlrg1`, `lda_xc1d_ehwlrg1`,
                            `lda_xc_1d_ehwlrg`, ...
    The caller intersects the candidate set with actual on-disk dir
    names and picks the first match.
    """
    # Stage 1: insert `_` before every uppercase letter.
    base = re.sub(r"([A-Z])", r"_\1", variant).lower().lstrip("_")

    # Build the candidate set by independently choosing whether to insert
    # a `_` at each letter↔digit boundary.
    # Find letter↔digit boundaries.
    boundary_re = re.compile(r"(?<=[a-z])(?=[0-9])|(?<=[0-9])(?=[a-z])")
    # All 2^n combinations of insert/no-insert at each boundary.
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

    # Add the original (no boundary `_`) for safety.
    candidates.add(base)

    # `LdaXc1dEhwlrg{1,2,3}` share a dir without the trailing digit suffix.
    extras: set[str] = set()
    for c in candidates:
        if "ehwlrg" in c:
            stripped = re.sub(r"_?[0-9]+$", "", c)
            extras.add(stripped)
    candidates.update(extras)
    return list(candidates)


def parse_routed_funcnames(rs_path: Path,
                           known_dirs: set[str] | None = None) -> set[str]:
    """Read a `<family>_functional.rs` file and return the set of routed
    funcnames.

    Two strategies, in order:
      1. If the file has a `Self::Variant => "func_name"` map (GGA/MGGA),
         read names directly from that.
      2. Otherwise (LDA), derive names from `<id> => Ok(Self::<Variant>)`
         arms; for each variant produce candidate snake-case names and
         intersect with `known_dirs` to pick the actual one.

    This is the canonical "is this dispatched" set: variants that don't
    appear here aren't reachable through `<X>Functional::from_id` ->
    dispatch -> kernel-launch.
    """
    text = rs_path.read_text()
    out: set[str] = set()
    for m in re.finditer(r'Self::\w+\s*=>\s*"([a-z0-9_]+)"', text):
        out.add(m.group(1))
    if out:
        return out

    # Fallback: derive name from variant + intersect with known dirs.
    if known_dirs is None:
        known_dirs = set()
    for m in re.finditer(r"=>\s*Ok\(Self::([A-Z][A-Za-z0-9]+)\)", text):
        variant = m.group(1)
        for cand in variant_to_snake_candidates(variant):
            if cand in known_dirs:
                out.add(cand)
                break
        else:
            # No match found — record the first candidate so the caller
            # can flag it (instead of silently misclassifying as unrouted).
            out.add(variant_to_snake_candidates(variant)[0])
    return out


def collect_func_dirs(family_glob: str) -> dict[str, list[Path]]:
    """Return {funcname: [path, ...]} for every functional dir found
    in any `crates/kernel-<family>*/src/<func>/`."""
    out: dict[str, list[Path]] = {}
    for crate in sorted(CRATES_DIR.glob(family_glob)):
        src = crate / "src"
        if not src.is_dir():
            continue
        for entry in sorted(src.iterdir()):
            if not entry.is_dir():
                continue
            out.setdefault(entry.name, []).append(entry)
    return out


def demote_dir(d: Path, dry_run: bool) -> tuple[int, int]:
    """Return (files_modified, total_replacements)."""
    files = 0
    repls = 0
    for rs in sorted(d.glob("*.rs")):
        text = rs.read_text()
        new_text, count = LAUNCH_LINE_RE.subn("#[cube]", text)
        if count == 0 or new_text == text:
            continue
        files += 1
        repls += count
        if not dry_run:
            rs.write_text(new_text)
    return files, repls


def process_family(family: str, model_path: Path, dry_run: bool) -> None:
    func_dirs = collect_func_dirs(f"kernel-{family}*")
    routed = parse_routed_funcnames(model_path, known_dirs=set(func_dirs))
    print(f"=== {family.upper()} ===")
    print(f"  Routed functionals (from {model_path.name}): {len(routed)}")
    print(f"  Distinct functional dirs across sub-crates: {len(func_dirs)}")

    unrouted = sorted(name for name in func_dirs if name not in routed)
    print(f"  Unrouted (demote candidates): {len(unrouted)}")

    if not unrouted:
        print()
        return

    total_files = 0
    total_repls = 0
    for func in unrouted:
        per_func_files = 0
        per_func_repls = 0
        for d in func_dirs[func]:
            f, r = demote_dir(d, dry_run)
            per_func_files += f
            per_func_repls += r
        if per_func_repls > 0:
            print(f"    {func}: {per_func_files} file(s), {per_func_repls} demoted")
        total_files += per_func_files
        total_repls += per_func_repls

    print(f"  TOTAL: {total_files} file(s) / {total_repls} #[cube(...)] lines\n")


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    process_family(
        "lda",
        REPO_ROOT / "src" / "model" / "lda_functional.rs",
        dry_run,
    )
    process_family(
        "gga",
        REPO_ROOT / "src" / "model" / "gga_functional.rs",
        dry_run,
    )
    process_family(
        "mgga",
        REPO_ROOT / "src" / "model" / "mgga_functional.rs",
        dry_run,
    )
    if dry_run:
        print("Dry run -- no changes made.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
