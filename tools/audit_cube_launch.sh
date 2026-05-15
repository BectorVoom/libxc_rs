#!/usr/bin/env bash
# Phase 11 P11-INV-5 (D-13 form): per-DESIGN launch-budget audit.
#
# The original P11-INV-5 was a flat `#[cube(launch_unchecked)]` count ≤ 23
# under crates/kernels/. D-13 (11-CONTEXT.md, user decision 2026-05-15) found
# that form unsatisfiable against the D-10b dispatch design: the dispatch
# macros (preserved verbatim) call `.launch_unchecked()` per
# `(functional × output module)`, so every routed entry kernel MUST be
# `#[cube(launch_unchecked)]` (~1677 total). D-13 keeps the macros and
# redefines the invariant to a per-design budget. This script asserts that
# budget — NOT a flat count:
#
#   ASSERTION 1 — ROUTED functionals: exactly one `#[cube(launch_unchecked)]`
#     entry kernel per emitted output module. An output module is each
#     `pub mod <output>;` line in the functional's `src/lib.rs`; its entry
#     file is `src/<output>.rs` (single-file) or `src/<output>/mod.rs`
#     (nested-by-output, D-04). The `partNN` chunk files carry plain `#[cube]`
#     and contribute zero launchables.
#   ASSERTION 2 — UNROUTED functionals: zero launchable kernels anywhere
#     under the functional's directory (the §4 anti-pattern fix from quick
#     task 260512-q01 must hold; includes the D-11 deferred kernels and
#     mgga_x_br89_explicit).
#   ASSERTION 3 — math/ budget: `crates/kernels/math/src/`
#     `#[cube(launch_unchecked)]` count ≤ 22, EXCLUDING `crates/kernels/math/
#     tests/` (the Wave-0 spike spike_tuple_return_cube.rs is a test fixture).
#
# The routed/unrouted partition comes from tools/kernel_routing.py
# (`cached_routed_funcnames(family)` / `collect_func_dirs(family)`) — the
# single source of truth for "is this functional routed?". The audit logic
# is implemented as a Python block (the routing partition is most naturally
# consumed in Python); bash only locates the interpreter and the repo root.
#
# This script runs NO cargo — it is a filesystem + routing-table audit. It
# sets no build-job or stack-size environment overrides and passes no
# job-count flag to any tool.
# Build env source of truth: .cargo/config.toml (do not duplicate values here).

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

python3 - "$REPO_ROOT" <<'PYEOF'
import re
import sys
from pathlib import Path

REPO_ROOT = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(REPO_ROOT / "tools"))
import kernel_routing as kr

KERNELS = REPO_ROOT / "crates" / "kernels"
FAMILIES = ("lda", "gga", "mgga")

LAUNCH_RE = re.compile(r"#\[cube\(launch_unchecked\)\]")
COMMENT_RE = re.compile(r"^\s*//")


def count_launch(path: Path) -> int:
    """Count `#[cube(launch_unchecked)]` lines in a file, excluding // comments."""
    if not path.is_file():
        return 0
    n = 0
    for line in path.read_text().splitlines():
        if COMMENT_RE.match(line):
            continue
        if LAUNCH_RE.search(line):
            n += 1
    return n


def count_launch_tree(root: Path) -> int:
    """Sum `#[cube(launch_unchecked)]` across all .rs files under a directory."""
    total = 0
    for rs in root.rglob("*.rs"):
        total += count_launch(rs)
    return total


def output_modules(func_dir: Path) -> list[str]:
    """The `pub mod <output>;` names from the functional subcrate's src/lib.rs.

    This is the authoritative output-module list per D-04."""
    lib = func_dir / "src" / "lib.rs"
    if not lib.is_file():
        return []
    return re.findall(r"^\s*pub mod\s+([A-Za-z0-9_]+)\s*;", lib.read_text(), re.M)


def entry_file(func_dir: Path, output: str) -> Path | None:
    """Entry file for an output module: src/<output>.rs OR src/<output>/mod.rs."""
    single = func_dir / "src" / f"{output}.rs"
    if single.is_file():
        return single
    nested = func_dir / "src" / output / "mod.rs"
    if nested.is_file():
        return nested
    return None


fail = False
routed_pair_total = 0
unrouted_launchable = []  # (family, func, count)

# --- ASSERTIONS 1 & 2: per-functional, partitioned by routing table ---
for family in FAMILIES:
    routed = kr.cached_routed_funcnames(family)
    func_dirs = kr.collect_func_dirs(family)  # {funcname: [Path, ...]}

    for func, paths in sorted(func_dirs.items()):
        func_dir = paths[0]
        is_routed = func in routed

        if is_routed:
            # ASSERTION 1: exactly one launchable per output-module entry file.
            mods = output_modules(func_dir)
            if not mods:
                print(f"FAIL: routed functional {family}/{func} has no "
                      f"`pub mod` output modules in src/lib.rs [P11-INV-5 A1]")
                fail = True
                continue
            for output in mods:
                ef = entry_file(func_dir, output)
                if ef is None:
                    print(f"FAIL: routed {family}/{func} output module "
                          f"'{output}' has no entry file "
                          f"(src/{output}.rs or src/{output}/mod.rs) "
                          f"[P11-INV-5 A1]")
                    fail = True
                    continue
                n = count_launch(ef)
                routed_pair_total += 1
                if n != 1:
                    rel = ef.relative_to(REPO_ROOT)
                    print(f"FAIL: routed ({family}/{func}, {output}) entry "
                          f"file {rel} has {n} `#[cube(launch_unchecked)]` "
                          f"(expected exactly 1) [P11-INV-5 A1]")
                    fail = True
        else:
            # ASSERTION 2: unrouted functional → zero launchables anywhere.
            n = count_launch_tree(func_dir)
            if n != 0:
                unrouted_launchable.append((family, func, n))
                fail = True

if unrouted_launchable:
    print(f"FAIL: {len(unrouted_launchable)} unrouted functional(s) carry "
          f"≥1 `#[cube(launch_unchecked)]` (the §4 anti-pattern — unrouted "
          f"entry kernels must be plain `#[cube]`) [P11-INV-5 A2]:")
    for family, func, n in unrouted_launchable:
        print(f"  {family}/{func}: {n} launchable kernel(s)")

# --- ASSERTION 3: math/src/ budget ≤ 22, excluding math/tests/ ---
MATH_SRC = KERNELS / "math" / "src"
math_count = count_launch_tree(MATH_SRC) if MATH_SRC.is_dir() else 0
MATH_BUDGET = 22
if math_count > MATH_BUDGET:
    print(f"FAIL: crates/kernels/math/src/ has {math_count} "
          f"`#[cube(launch_unchecked)]` (budget ≤ {MATH_BUDGET}; "
          f"crates/kernels/math/tests/ is excluded) — Phase 11 MUST NOT add "
          f"launchables to math/ [P11-INV-5 A3]")
    fail = True

if fail:
    sys.exit(1)

print("PASS: D-13 per-design launch budget holds [P11-INV-5]")
print(f"  A1: {routed_pair_total} routed (functional, output) launch pairs — "
      f"each entry file has exactly 1 `#[cube(launch_unchecked)]`")
print(f"  A2: 0 unrouted functionals carry a launchable kernel")
print(f"  A3: crates/kernels/math/src/ launch count = {math_count} "
      f"(≤ {MATH_BUDGET}; math/tests/ excluded)")
PYEOF
