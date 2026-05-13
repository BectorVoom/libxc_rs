#!/usr/bin/env bash
# Phase 11 P11-INV-5: fail if #[cube(launch_unchecked)] count under crates/kernels/
# exceeds the post-W0 baseline (23 = pre-Phase-11 22 + 1 added by the Wave 0
# tuple-return spike at crates/kernels/math/tests/spike_tuple_return_cube.rs).
# The pre-Phase-11 count recorded in 11-BASELINE.md is 22; the +1 deviation is
# documented in 11-01-SUMMARY.md. Subsequent waves MUST NOT increase further.
# Build env source of truth: .cargo/config.toml.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BASELINE=23

# Count occurrences, excluding comment lines (// ...).
COUNT=$(find "$REPO_ROOT/crates/kernels" -name '*.rs' -print0 \
          | xargs -0 grep -h '#\[cube(launch_unchecked)\]' \
          | grep -vc '^[[:space:]]*//' \
          || true)

if [[ "$COUNT" -gt "$BASELINE" ]]; then
    echo "FAIL: cube(launch_unchecked) count = $COUNT (baseline=$BASELINE) [P11-INV-5]"
    exit 1
fi
echo "PASS: cube(launch_unchecked) count = $COUNT (≤ baseline $BASELINE)"
