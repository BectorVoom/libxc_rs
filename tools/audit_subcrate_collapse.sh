#!/usr/bin/env bash
# Phase 11 P11-INV-1: fail if any crates/kernels/{lda,gga,mgga}-N numbered
# subcrate exists OR if any family-level crate (crates/kernels/{lda,gga,mgga}/
# Cargo.toml or src/lib.rs) exists — under the D-10 per-functional-subcrate
# model the family level must be a plain directory, not a crate.
# Build env source of truth: .cargo/config.toml (do not duplicate values here).

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

FAIL=0

# --- Check 1: no numbered subcrates ---
NUMBERED=$(find "$REPO_ROOT/crates/kernels" -maxdepth 1 -mindepth 1 -type d -printf '%f\n' \
             | grep -E '^(lda|gga|mgga)-[0-9]' \
             | sort \
             || true)
COUNT=$(printf '%s\n' "$NUMBERED" | grep -c . || true)

if [[ "$COUNT" -gt 0 ]]; then
    echo "FAIL: $COUNT numbered subcrate(s) remain (P11-INV-1):"
    printf '  %s\n' $NUMBERED
    FAIL=1
fi

# --- Check 2: no family-level crate artifacts (D-LOCK-A) ---
# `|| true` is load-bearing: when no family crate artifacts remain (the correct
# post-11-03 state) the loop's final `[[ -f ]] && echo` test is false, so the
# command-substitution subshell exits 1 — and under `set -e` that would abort
# the script before Check 2's `if` even runs. The `|| true` neutralises that.
FAMILY_CRATES=$(for f in lda gga mgga; do
  [[ -f "$REPO_ROOT/crates/kernels/$f/Cargo.toml" ]] && echo "$f/Cargo.toml"
  [[ -f "$REPO_ROOT/crates/kernels/$f/src/lib.rs" ]] && echo "$f/src/lib.rs"
done) || true

if [[ -n "$FAMILY_CRATES" ]]; then
    echo "FAIL: family-level crate artifact(s) remain (P11-INV-1, per-functional-subcrate invariant):"
    printf '  %s\n' $FAMILY_CRATES
    FAIL=1
fi

if [[ "$FAIL" -ne 0 ]]; then
    exit 1
fi
echo "PASS: zero numbered subcrates and zero family-level crates under crates/kernels/"
