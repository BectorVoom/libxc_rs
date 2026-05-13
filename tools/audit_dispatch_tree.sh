#!/usr/bin/env bash
# Phase 11 Blocker B1 audit: enumerate every `crate::kernel::{family}::batchN`
# reference under src/eval/{gga,mgga}_dispatch/ and compare to the batchN
# aliases exposed by crates/kernels/{gga,mgga}/src/lib.rs.
#
# Exits 0 if every dispatch reference resolves against the current façade.
# Exits 1 if any dispatch reference is UNRESOLVED — pre-existing staleness
# that plan 11-05's collapse migrator must close by regenerating dispatch.
#
# Build env source of truth: .cargo/config.toml.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# 1. Collect referenced batchN names from dispatch files.
REFERENCED_GGA=$(grep -rhoE 'crate::kernel::gga::batch[0-9]+[a-z]*' \
                       src/eval/gga_dispatch/ 2>/dev/null \
                  | sed 's|.*::||' | sort -u || true)
REFERENCED_MGGA=$(grep -rhoE 'crate::kernel::mgga::batch[0-9]+[a-z]*' \
                        src/eval/mgga_dispatch/ 2>/dev/null \
                  | sed 's|.*::||' | sort -u || true)

# 2. Collect exposed batchN names from façade lib.rs files.
EXPOSED_GGA=$(grep -E 'pub (use [A-Za-z_0-9]+ as |mod )batch[0-9]+[a-z]*' \
                    crates/kernels/gga/src/lib.rs 2>/dev/null \
                | sed -E 's/.*(batch[0-9]+[a-z]*).*/\1/' | sort -u || true)
EXPOSED_MGGA=$(grep -E 'pub (use [A-Za-z_0-9]+ as |mod )batch[0-9]+[a-z]*' \
                     crates/kernels/mgga/src/lib.rs 2>/dev/null \
                | sed -E 's/.*(batch[0-9]+[a-z]*).*/\1/' | sort -u || true)

# 3. Compute unresolved sets (referenced minus exposed).
UNRESOLVED_GGA=$(comm -23 <(printf '%s\n' "$REFERENCED_GGA") <(printf '%s\n' "$EXPOSED_GGA"))
UNRESOLVED_MGGA=$(comm -23 <(printf '%s\n' "$REFERENCED_MGGA") <(printf '%s\n' "$EXPOSED_MGGA"))

REF_GGA_COUNT=$(printf '%s\n' "$REFERENCED_GGA" | grep -c . || true)
EXP_GGA_COUNT=$(printf '%s\n' "$EXPOSED_GGA" | grep -c . || true)
GGA_BAD=$(printf '%s\n' "$UNRESOLVED_GGA" | grep -c . || true)

REF_MGGA_COUNT=$(printf '%s\n' "$REFERENCED_MGGA" | grep -c . || true)
EXP_MGGA_COUNT=$(printf '%s\n' "$EXPOSED_MGGA" | grep -c . || true)
MGGA_BAD=$(printf '%s\n' "$UNRESOLVED_MGGA" | grep -c . || true)

echo "GGA dispatch references: $REF_GGA_COUNT"
echo "GGA façade exposes:       $EXP_GGA_COUNT"
echo "GGA unresolved:           $GGA_BAD"
[[ -n "$UNRESOLVED_GGA" ]] && printf '  %s\n' $UNRESOLVED_GGA
echo "MGGA dispatch references: $REF_MGGA_COUNT"
echo "MGGA façade exposes:      $EXP_MGGA_COUNT"
echo "MGGA unresolved:          $MGGA_BAD"
[[ -n "$UNRESOLVED_MGGA" ]] && printf '  %s\n' $UNRESOLVED_MGGA

if [[ "$GGA_BAD" -gt 0 || "$MGGA_BAD" -gt 0 ]]; then
    echo "FAIL: dispatch tree has unresolved batchN references against the current façade."
    echo "      This is pre-existing staleness from Phase 4-04; plan 11-05's collapse"
    echo "      blast radius includes regenerating the dispatch tree to close it."
    exit 1
fi
echo "PASS: every dispatch batchN reference resolves against the current façade."
