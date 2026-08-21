# Plan: bit-exact SIMD math that beats C libxc

Status: proposed  
Date: 2026-08-21  
Scope: `rmath` (the canonical SIMD-math implementation),
`crates/kernels-rayon/math` (the libxc compatibility adapter), the SIMD rewrite
in `tools/translate_rayon`, and the minimum benchmark/oracle support needed to
prove an end-to-end speed win.

For the bit-exact production objective, this supersedes
`docs/perf/rmath-fast-kernels-plan.md`; that document remains useful only as a
record of the approximate-math experiment.

## Outcome

Build a selective explicit-SIMD path whose generated kernel outputs are
bit-identical to the current scalar Rust kernels and whose complete evaluation
path is faster than both serial C libxc and caller-parallelised C libxc on the
reference CPU. Generic math algorithms and their vector implementations belong
in `rmath`; `libxc-rkernel-math` owns only libxc-specific compatibility wrappers,
generator integration, and the end-to-end fingerprint gate.

The first production target is `lda_c_vwn` `exc`/`vxc`, unpolarized. It has the
best time-to-win because each point contains four logarithms and two
arctangents, its scalar loop does not vectorize, and bit-exact SIMD `ln` and
cube-root functions already exist. The one missing high-value primitive is a
bit-exact vector `atan` in `rmath`.

This is an exact-math plan. Approximate vector math may remain useful as an
experiment, but it is not an acceptable production route for this objective.

## Exactness contract

“Bit-exact” has two enforceable meanings in this plan:

1. A SIMD math function must return the same `to_bits()` as the scalar Rust
   function it replaces for every tested lane, including signed zero,
   infinities, subnormals, and NaN payloads.
2. A generated SIMD kernel must have the same order-sensitive output
   fingerprint as the scalar generated kernel built from the same maple2c
   source.

The C-libxc comparison is a separate contract: no new oracle mismatch and
finite relative error at or below `1e-12`. The current Rust kernels are not
globally bit-identical to C libxc, so a math-only optimization cannot honestly
promise C/Rust bit identity. It can and must prove that optimization changes
zero Rust output bits while retaining or improving existing C parity.

## Constraints

- CPU-only Rust plus rayon; no production C/Fortran and no GPU path.
- Keep maple2c expression order and grouping unchanged outside the math call.
- Never hand-edit generated functional crates under `crates/kernels-rayon`.
- Keep `screened_call`, per-chunk zeroing, strides, polarized dimensions, and
  repeated-last-lane tail padding unchanged.
- Keep zero heap allocations per repeated evaluation.
- Keep CubeCL optional and default-off in `libxc-rkernel-math`.
- An exact SIMD helper must stay `#[inline(always)]`; an outlined call in the
  grid loop is a failed implementation.
- Explicit SIMD remains an allowlist per `(functional, order, spin)`. The
  existing scalar loop already auto-vectorizes pure arithmetic, so blanket SIMD
  would regress many kernels.

## CodeGraph map

CodeGraph shows the production flow as:

```text
libxc maple2c C
  -> from_maple.py::translate_expr
  -> from_maple.py::emit_function
  -> simd.py::simd_body             (allowlisted triples only)
  -> generated f64x8 kernel
  -> libxc-rkernel-math::{simd,powers,erf,...}
  -> libxc-reval generated dispatch
  -> screened rayon sweep
  -> bench-vs-libxc / facade / C ABI
```

The important blast radii are:

- `rmath` exact `exp`, `ln`, and future `atan`: its own all-domain exactness
  suite and microbenchmarks, followed by libxc adapter tests.
- `simd::cbrt`: its fractional-power wrappers and every exact SIMD kernel using
  a `POW_n_3` helper. This remains a libxc compatibility implementation until
  an `rmath` kernel can reproduce `powers::cbrt_f64` bit-for-bit.
- `piecewise3`: 2,438 generated call sites according to CodeGraph; SIMD
  rewriting must continue to turn its condition into a lane mask and selection.
- `erf_approx`: 495 current generated source sites; its scalar operation order
  is the reference if an exact vector form is added.
- `from_maple.py::{SIMD_EXACT_FUNCS,SIMD_RMATH_FAST_FUNCS}` and
  `simd.py::{FREE_EXACT,UNARY_EXACT,BINARY_METHODS_EXACT}` control which math
  semantics reach production.

## Current evidence

### Exact helpers that already work

`crates/kernels-rayon/math/src/simd.rs` currently provides:

| helper | exact scalar reference | current mechanism |
|---|---|---|
| `exp` | glibc 2.43 `f64::exp` FMA variant | vectorized ARM optimized-routines schedule plus scalar patch lanes |
| `ln` | glibc 2.43 `f64::ln` FMA variant | vectorized table/near-one schedules plus scalar patch lanes |
| `cbrt`, `pow_{2,4,5,7}_3` | `powers::cbrt_f64` and its exact expression sequences | identical lane-wise operation schedule |
| `sqrt`, arithmetic, comparison, select | scalar IEEE operations | native `wide::f64x8` operations |

`cargo test --release -p libxc-rkernel-math --test simd_exact` passes four
bitwise tests. The suite covers millions of physical and adversarial inputs,
branch boundaries, random bit patterns, and mixed special lanes.

Fresh reference-machine microbenchmarks, in ns/element, are:

| operation | scalar | exact SIMD | exact SIMD / scalar |
|---|---:|---:|---:|
| `exp` | 2.186 | 0.976 | 2.24x faster |
| `ln` | 1.998 | 0.777 | 2.57x faster |
| `cbrt_f64` | 1.207 | 1.415 | 0.85x |

The standalone cube root is not itself a win. It remains valuable inside a
kernel whose libm calls prevent LLVM from vectorizing the surrounding body.

### The current `rmath::fast` path fails this objective

The generated `lda_c_vwn` `exc_unpol` and `vxc_unpol` files currently call
`rmath_fast::{cbrt,ln,atan}`. That path is approximate by design, so it cannot
meet the fingerprint contract.

A fresh rebuild and `bench-vs-libxc --only lda_c_vwn --np 100000 --reps 7`
measured:

| leg | ns/point | comparison |
|---|---:|---:|
| libxc-1t | 68.80 | baseline |
| rust-1t | 79.40 | 0.87x; Rust loses |
| libxc-Nt | 11.55 | honest parallel baseline |
| rust-Nt | 8.59 | 1.34x faster |

Disassembly explains the single-thread result. The generated Rust function has
a `0x880`-byte stack frame and retains calls to
`rmath::reference::double::{cbrt,ln,invtrig::atan}` in the loop. The current
path therefore fails both desired properties: it is not bit-exact and it does
not beat serial libxc.

The implementation campaign must first move `lda_c_vwn` out of the approximate
production allowlist and establish a scalar fingerprint baseline. Historical
numbers in `docs/perf/simd-kernels.md` are useful context, but freshly rebuilt
measurements and disassembly are the acceptance evidence.

### Static source-site census

Current generated source contains the following math call sites. These are
static sites across all orders/spins, not dynamic calls per point, so they are
a candidate prefilter rather than a performance ranking.

| call family | sites | status / implication |
|---|---:|---|
| `pow_1_3` | 15,900 | exact vector form exists; only useful where it unlocks a blocked loop |
| `sqrt` | 5,928 | already exact hardware/vector operation |
| `ln` | 5,059 | exact vector form exists |
| `exp` | 2,788 | exact vector form exists |
| `powf` | 2,159 | highest-volume missing exact SIMD primitive |
| `erf_approx` | 495 | scalar Rust helper can be replicated exactly |
| `atan` | 391 | first target because it unlocks `lda_c_vwn` |
| `tanh` | 131 | later target |
| `atan2` | 60 | later target, built on exact `atan` plus exact quadrant rules |
| `erfc_approx` | 30 | pair with `erf_approx` |
| `cosh` / `sinh` | 30 / 12 | profile-driven only |
| `cos` / `sin` | 19 / 17 | profile-driven only |

Among routed kernels, `gga_x_airy` and `gga_x_lag` are the clearest later
`powf` candidates. Their `vxc` bodies contain 11/22 and 5/10 `powf` sites for
unpolarized/polarized forms respectively. `gga_c_hcth_a` is a later exact-atan
candidate with 4/7 sites in `vxc`, but its pre-existing oracle issue must be
tracked separately.

## Design

### 1. Make “exact” a mechanically enforced generator policy

The current `UNARY_EXACT` name is unsafe: it includes `wide` approximations for
`atan`, `tanh`, `sinh`, `cosh`, `asin`, `acos`, `exp_m1`, and `ln_1p`. A kernel
can therefore be labeled exact while silently containing inexact math.

Replace the mappings with explicit capability classes:

```python
EXACT_FREE = {
    "f64::exp": "rmath_exact::exp",
    "f64::ln": "rmath_exact::ln",
    "f64::atan": "rmath_exact::atan", # after rmath Phase 2
    "pow_1_3": "simd::cbrt",
    # remaining exact cube-root family
}

EXACT_NATIVE = {
    "f64::sqrt": "sqrt",
    "f64::abs": "abs",
}
```

For `math_mode="exact"`, generation must fail if any mapped scalar
transcendental lacks an exact SIMD implementation. It must never fall through
to a `wide` approximation or `rmath::fast`. Add a post-rewrite audit that rejects
residual `.atan()`, `.tanh()`, `.powf_simd()`, `rmath_fast::`, or other
non-approved math calls in an exact kernel.

Keep the exact and experimental/approximate allowlists disjoint. For this
campaign, the production approximate allowlist should be empty. If the
approximate machinery is retained for research, its generated files and docs
must say `approximate`, and no bit-exact benchmark result may use it.

### 2. Use a vector fast path with exact scalar patch lanes

Follow the successful `exp`/`ln` pattern for each new primitive:

1. Classify lanes with bit-equivalent thresholds.
2. Evaluate the common physical regions as vector arithmetic with the same
   operation and FMA schedule as the scalar reference.
3. Blend regions with masks.
4. Patch only unsupported or exceptional lanes with the scalar Rust function.
5. Return early when all lanes are on the vector fast path.

This makes exactness unconditional without forcing rare IEEE cases through the
hot vector schedule. Mixed-lane tests must prove that a scalar-patched lane
cannot perturb its neighbors.

### 3. Treat platform-specific scalar libm behavior explicitly

The reference machine is:

- `x86_64-unknown-linux-gnu`;
- glibc 2.43;
- rustc 1.97.1 / LLVM 22.1.6;
- AMD Ryzen AI 7 350 (Zen 5), FMA and AVX-512;
- `-C target-cpu=native` from the repository target-cfg configuration.

`f64::atan` is a glibc ifunc on this target. As with existing exact `exp` and
`ln`, the vector schedule must name the supported scalar implementation and be
tested against it. Do not claim portable bit identity against every platform's
libm.

On an unsupported target/libm, correctness must fall back to scalar lane calls
or to the ordinary scalar generated kernel. It is acceptable for the optimized
allowlist to be target-specific; it is not acceptable to emit a fast vector
answer whose bits depend silently on the host libm version.

Algorithm and table sources must be permissively licensed and recorded with
SPDX/provenance comments. glibc disassembly may be used as a behavioral oracle
for instruction ordering, but copying an LGPL implementation into the
production crate requires an explicit license decision and is not assumed by
this plan.

### 4. Measure complete kernels, not isolated functions alone

Microbenchmarks answer whether a primitive is viable. Admission is decided by
the complete load/compute/store kernel and the complete screened rayon sweep.
An exact helper does not imply that forcing explicit SIMD is a win: arithmetic
kernels already auto-vectorize, and large derivative bodies may spill enough
registers to lose.

Every `(functional, order, spin)` is a separate candidate. Success for
`exc_unpol` does not authorize `vxc`, a polarized form, or a higher derivative.

## Work plan

### Phase 0 — freeze honest baselines

1. Add a benchmark-only generator override that can emit `scalar` or `exact`
   for one requested triple without editing production allowlists by hand.
2. Regenerate scalar `lda_c_vwn` `exc`/`vxc` unpolarized and record:
   output fingerprint, C-libxc error by field, single-thread and rayon timing,
   allocations, stack frame, vector width, and hot-path calls.
3. Record the current approximate result separately, then empty
   `SIMD_RMATH_FAST_FUNCS` for the production bit-exact baseline.
4. Capture compiler, glibc, CPU, target flags, grid construction, thread count,
   `XCVS_TAIL`, `XCVS_TAIL_LAYOUT`, `XCVS_MIN_CHUNK`, and benchmark command in a
   checked-in results table.
5. Extend the benchmark report with the median and a paired 95% confidence
   interval for the Rust/libxc ratio; keep the existing best time for continuity
   but do not use it alone for admission.
6. Update misleading current documentation: approximate `rmath::fast` output
   is not an exact policy and its old performance number is not a current gate.

Deliverable: a reproducible scalar fingerprint and honest libxc timing baseline.

### Phase 1 — harden the exact generator boundary

1. Refactor `tools/translate_rayon/simd.py` into exact-native, exact-helper,
   and explicitly approximate mappings.
2. Add exact-mode rejection for every unsupported transcendental.
3. Correct actual post-translation spellings. For example, generated scalar
   code calls `erf_approx`, not `erf`; a mapping for the latter alone is dead.
4. Update `tools/translate_rayon/test_simd.py` to assert:
   exact `atan` maps only to `rmath_exact::atan`; exact unsupported calls fail;
   approximate names never appear in exact output; nested calls preserve
   argument order/grouping; `atan2(y, x)` keeps its argument order; piecewise
   masks and tail logic are unchanged.
5. Make generator comments state the exact capability actually used by that
   file rather than a generic claim about all SIMD math.

Deliverable: an exact allowlist that cannot silently emit approximate math.

### Phase 2 — make `rmath::Atan<BitExact, FullRange>` vectorized

1. Work in `/home/user/Documents/workspace/rmath`, not in the generated libxc
   tree or `libxc-rkernel-math::simd`. CodeGraph shows that rmath already has a
   vector `kernels::double::invtrig::atan` with the required region masks,
   tables, and FMA schedule, while the generic `dispatch` currently chooses
   `map_lanes(reference::atan)` whenever `A::BIT_EXACT` is true. Promote that
   vector path only after it exactly replays `reference::double::invtrig::atan`.
2. Resolve and disassemble the glibc 2.43 FMA `atan` variant selected on the
   reference machine. Compare its thresholds, range reduction, constants,
   polynomial/rational evaluation, FMA contraction points, sign restoration,
   and exceptional paths to rmath's existing reference and vector paths.
3. Keep generic constants, tables, dispatch, and the vector implementation in
   rmath (`src/kernels/double/invtrig.rs` and its table module). Add only the
   smallest libxc adapter needed to call `Atan<BitExact, FullRange>` on
   `wide::f64x8`; do not copy the algorithm into `libxc-rkernel-math`.
4. Use `rmath::simd::patch_lanes` for exceptional lanes and retain scalar
   `reference::atan` repair where necessary. Optimize the physical `0..6.4`
   interval first, then broaden vectorized regions only after each is exact and
   profitable.
5. Preserve `atan(-0.0) == -0.0`, infinities, NaN payloads, and sign symmetry.
6. Add rmath tests for at least one million physical-range values, threshold
   neighborhoods at `-2..+2` ULP, log-uniform magnitudes, random raw bit
   patterns, every special, all-eight-lanes-per-region cases, and deliberately
   mixed regions/specials. Assert equality to rmath's scalar `BitExact`
   reference in `rmath`; repeat a focused adapter test in libxc.
7. Add or extend rmath's benchmark for scalar, `wide` approximate, exact SIMD,
   and the `lda_c_vwn` argument distribution. Inspect optimized assembly: the
   all-physical-lanes path must be inlined, use packed `zmm` arithmetic, and
   execute no scalar `atan` call.

Microbenchmark gate: exact SIMD `atan` must be at least 2x faster per element
than scalar `f64::atan` on the `lda_c_vwn` argument distribution. If it misses,
profile before integrating; do not hide a weak primitive result in rayon speedup.

Deliverable: all-domain bit identity with a materially faster physical fast path.

### Phase 3 — land the first end-to-end win

1. Add a narrow `rmath_exact` facade in `libxc-rkernel-math`, marked
   `#[inline(always)]`, that calls rmath's `BitExact` function objects for
   `f64x8`. Map `f64::atan` to `rmath_exact::atan` in exact mode.
2. Put `lda_c_vwn` `exc_unpol` and `vxc_unpol` in `SIMD_EXACT_FUNCS`; remove them
   from the approximate list.
3. Benchmark rmath's exact `ln` and `exp` against the existing libxc exact
   helpers before rerouting them. Adopt an rmath helper only if its bits and
   generated-kernel performance both match or improve; retain libxc's
   `simd::cbrt` where it is needed to reproduce `powers::cbrt_f64`.
4. Ensure exact generated files use only `rmath_exact`, approved libxc
   compatibility helpers, and native operations — never `rmath_fast`.
5. Regenerate through `from_maple.py`; do not edit the emitted Rust.
6. Compare scalar and exact SIMD outputs bitwise for:
   lengths `1,2,7,8,9,15,16,17`; normal physical grids; threshold
   neighborhoods; 40% contiguous screened tails; scattered screened points;
   and mixed special lanes at the math-adapter level.
7. Run the C oracle and `revalcheck` before timing.
8. Benchmark `bench-vs-libxc --only lda_c_vwn` at 100k and 1M points, with at
   least 15 accepted repetitions, for active and 40%-contiguous-tail grids.
9. Inspect the final kernel assembly and profile samples. Cold scalar patch
   blocks may exist, but the physical benchmark must execute none of them.
10. If no production file uses `rmath_fast`, remove that approximate re-export
    from the production exact path, but retain the normal local `rmath`
    dependency. Keep only its `wide` feature, avoid a duplicate `wide`, and
    preserve the optional CubeCL feature exactly.

End-to-end acceptance requires the lower confidence bound to show at least:

- 1.5x over serial C libxc for `rust-1t`;
- 1.25x over caller-parallel C libxc for `rust-Nt`;
- zero Rust scalar-vs-SIMD fingerprint changes;
- no new C-oracle or `revalcheck` mismatch;
- zero allocations per evaluation.

The scattered-tail layout is an adversarial screening case, not the primary
win target, but it must be reported and must not suffer an unexplained severe
regression.

Deliverable: the first production bit-exact SIMD kernel that wins against both
honest libxc bars.

### Phase 4 — expand exact math in measured priority order

Evaluate each family only after the previous helper is exact, inlined, and
microbenchmarked.

1. **Reuse exact `atan`.** Rank `lda_c_vwn_{2,3,4,rpa}` and
   `gga_c_hcth_a` triples by sampled time. Admit only fingerprint-preserving
   whole-kernel wins.
2. **Exact `powf`.** This is the largest missing call family and the route to
   `gga_x_airy`/`gga_x_lag`. Reproduce the selected glibc `pow` FMA schedule,
   preferably from the same permissively licensed optimized-routines lineage as
   current `exp`/`ln`; use exact vector `ln`/`exp` components only where their
   combined rounding schedule is proven identical to scalar `powf`. Patch hard
   sign, integer-exponent, overflow/underflow, zero, infinity, and NaN lanes.
   Do not implement `powf(x,y)` as a naïve `exp(y*ln(x))`.
3. **Exact `erf_approx`/`erfc_approx`.** Replicate the existing scalar Rust
   region formulas and operation order over `f64x8`, using an approved exact
   `rmath` or libxc compatibility `exp` helper.
   The reference is the crate helper, not system `erf`. First profile whether
   any routed functional benefits; many current sites are in unrouted kernels.
4. **Exact `tanh`, then `atan2`.** Implement only after a routed, sampled
   kernel shows enough time in the call. `atan2` must reproduce scalar quadrant
   and signed-zero rules rather than merely calling exact `atan(y/x)`.
5. **Remaining trig/hyperbolic functions.** `sin`, `cos`, `sinh`, and `cosh`
   have low site counts; require profile evidence before paying implementation,
   test, and compile-time cost.

For every helper, repeat the math-level exactness suite, microbenchmark,
translator tests, per-triple scalar fingerprint comparison, assembly audit,
oracle checks, and whole-kernel benchmark. Do not infer one order/spin from
another.

## Verification matrix

| layer | required proof |
|---|---|
| math semantics | `to_bits()` equality against the scalar reference over physical, boundary, random-bit, special, and mixed-lane inputs |
| generator | exact mode contains only approved exact calls; expressions, masks, strides, and tails remain structurally unchanged |
| kernel | scalar and SIMD fingerprints are identical for every admitted triple and tested grid layout |
| C correctness | oracle stays at zero new finite mismatches; known offenders do not worsen |
| chunking | `revalcheck` gains no differences beyond the four documented `gga_c_op_pw91` values |
| missing oracle coverage | add retained direct C comparisons for every admitted polarized or MGGA triple |
| screening | below-threshold outputs stay exact `+0.0`; contiguous and scattered layouts are both exercised |
| hot path | physical lanes take no scalar fallback; helper is inlined; expected packed width is present; stack spills are measured |
| performance | both 1t and caller-parallel libxc bars are beaten by the phase-specific minimum margin |
| allocation | repeated evaluation remains zero-allocation |
| dependencies | default tree has zero CubeCL; no duplicate `wide`; CubeCL feature test still passes |

Commands at the final gate:

```bash
cd /home/user/Documents/workspace/rmath && cargo test --release --test bit_exact
cd /home/user/Documents/workspace/rmath && cargo test --release --test fast_path
cd /home/user/Documents/workspace/libxc_rs
python3 tools/translate_rayon/test_simd.py
cargo test --release -p libxc-rkernel-math --test simd_exact
cargo run --release --manifest-path crates/kernels-rayon/math/Cargo.toml \
  --example simd_bench
python3 tools/translate_rayon/from_maple.py --all
python3 tools/translate_rayon/gen_eval.py
cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml
cargo run --release -p libxc-reval --bin revalcheck
cargo run --release -p bench-vs-libxc --bin xcvs -- \
  --only lda_c_vwn --np 100000 --reps 15
cargo tree -e normal
cargo tree -d
cargo test -p libxc-rkernel-math --features cubecl
```

Run the benchmark again with `--np 1000000`, `XCVS_TAIL=0.4`, and both
contiguous and scattered layouts. Store exact commands and results rather than
copying only the best headline number.

## Stop conditions

Reject or postpone a helper/triple when any of these is true:

- any finite lane differs in bits from its scalar Rust reference;
- a signed zero, infinity, or NaN payload differs without being scalar-patched;
- the generated kernel fingerprint moves;
- exactness depends on an undocumented libc/CPU implementation;
- a license-compatible algorithm/table source cannot be established;
- the physical fast path executes scalar fallback calls or is outlined;
- the primitive wins alone but the complete kernel regresses;
- the 1t or caller-parallel libxc win margin is below the phase gate;
- screening, allocation, build-tree, or CubeCL-feature invariants regress;
- the change requires reordering maple2c arithmetic outside the math call.

Document rejected candidates and measurements in `docs/perf/simd-kernels.md`
so they are not repeated after unrelated translator changes.

## Rollback

Each production optimization is one exact-allowlist entry plus shared exact
helpers. To roll back a triple:

1. remove it from `SIMD_EXACT_FUNCS`;
2. regenerate from maple2c;
3. confirm the recorded scalar fingerprint returns;
4. rerun the oracle and `revalcheck`.

There is no public runtime switch and no public API migration. The scalar
generated kernel remains the canonical reference and rollback path.

## Completion criteria

The first milestone is complete only when `lda_c_vwn` `exc`/`vxc`
unpolarized has an unchanged scalar fingerprint and beats both C-libxc timing
bars by the stated margins. The broader campaign is complete when every
production exact-SIMD triple has retained math-level proofs, scalar
fingerprints, C-oracle evidence, assembly evidence, benchmark results, and a
documented rollback, with no approximate math left under an “exact” label.
