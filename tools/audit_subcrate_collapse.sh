#!/usr/bin/env bash
# Phase 11 P11-INV-1: fail if any crates/kernels/{lda,gga,mgga}-N numbered subcrate exists.
# Build env source of truth: .cargo/config.toml (do not duplicate values here).

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

NUMBERED=$(find "$REPO_ROOT/crates/kernels" -maxdepth 1 -mindepth 1 -type d -printf '%f\n' \
             | grep -E '^(lda|gga|mgga)-[0-9]' \
             | sort \
             || true)
COUNT=$(printf '%s\n' "$NUMBERED" | grep -c . || true)

if [[ "$COUNT" -gt 0 ]]; then
    echo "FAIL: $COUNT numbered subcrate(s) remain (P11-INV-1):"
    printf '  %s\n' $NUMBERED
    exit 1
fi
echo "PASS: zero numbered subcrates under crates/kernels/"
