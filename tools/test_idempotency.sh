#!/usr/bin/env bash
# Phase 11 P11-INV-6: splitter must be idempotent — running it twice produces no diff.
# Build env source of truth: .cargo/config.toml (do not duplicate values here).
#
# Wave-0 NOTE: this script will FAIL until plans 11-02..05 land. It is committed
# at Wave 0 as a phase-gate tool so later waves have a stable acceptance contract.
# Do NOT modify it to "pass at Wave 0" — the contract is intentionally aspirational.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

if [[ -n "$(git status --porcelain crates/kernels)" ]]; then
    echo "FAIL: working tree has uncommitted changes under crates/kernels/ before idempotency check"
    git status --short crates/kernels | head -20
    exit 1
fi

# Run the unified driver. Defaults are inside the driver — do NOT pass --split-threshold.
# Falls back to a per-family loop if the driver doesn't accept --family all.
if python3 tools/maple_to_kernels.py all --family all 2>/dev/null; then
    :
else
    echo "info: driver did not accept --family all, falling back to per-family loop"
    for family in lda gga mgga; do
        python3 tools/maple_to_kernels.py all --family "$family"
    done
fi
SNAPSHOT1=$(git status --porcelain crates/kernels | sort)

if python3 tools/maple_to_kernels.py all --family all 2>/dev/null; then
    :
else
    for family in lda gga mgga; do
        python3 tools/maple_to_kernels.py all --family "$family"
    done
fi
SNAPSHOT2=$(git status --porcelain crates/kernels | sort)

if [[ "$SNAPSHOT1" != "$SNAPSHOT2" ]]; then
    echo "FAIL: re-run produced diff (P11-INV-6)"
    diff <(echo "$SNAPSHOT1") <(echo "$SNAPSHOT2") || true
    exit 1
fi
echo "PASS: splitter is idempotent"
