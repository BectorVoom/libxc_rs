# SIMD math-kernel optimization plan

- Status: completed
- Prepared: 2026-08-21
- Primary target: beat caller-parallelized libxc 7.0.0 with the same CPU and thread count
- Scope: `crates/kernels-rayon/math/`, its `rmath` integration, and the generator/measurement seams required to use SIMD math safely in generated kernels

## 1. Outcome

Make libm-heavy LDA, GGA, and MGGA evaluations faster than the honest reference,
`libxc-Nt`, while preserving libxc reachability, density screening, zero-allocation
evaluation, and the numerical contract.

The first implementation path is **bit-exact SIMD through `rmath`**. Approximate
SIMD remains a separately measured fallback, not the default strategy. This order
matters because the local `rmath` checkout now contains vector, bit-exact schedules
for most functions that the current libxc SIMD adapter still sends through
`wide`'s approximate methods.

The campaign is successful when:

1. every promoted `(functional, order, spin)` triple is at least **1.10x faster
   than `libxc-Nt`** at the same thread count on the reference machine;
2. an exact-SIMD promotion keeps the scalar-Rust output fingerprint unchanged;
3. no new finite output exceeds `1e-12` relative error against C libxc on the
   applicable physical corpus, with the existing documented oracle offenders
   compared by delta rather than silently accepted;
4. `revalcheck`, direct C-oracle coverage, screening, tail, and allocation gates
   remain unchanged; and
5. the final allowlist contains measured winners only. SIMD is not considered a
   win merely because vector instructions appear in assembly.

## 2. Non-goals and fixed constraints

- Do not hand-edit generated functional crates under `crates/kernels-rayon/`.
  All generated changes go through `tools/translate_rayon/`.
- Do not introduce CubeCL, a GPU path, C, or Fortran into production.
- Do not enable `fast-math`, reassociation, reciprocal approximations, or global
  floating-point contraction.
- Do not change maple2c statement order or expression grouping around a replaced
  math call.
- Do not bypass `screened_call`, change `MIN_RUN`, move output zeroing out of
  `par_sweep`, or weaken zero-allocation workspace reuse.
- Do not assume that a fast function's ULP bound survives differentiation.
  Acceptance is based on complete libxc output fields, not the direct math call.
- Do not force explicit SIMD on arithmetic-only kernels. LLVM already vectorizes
  many of them, and previous measurements show that explicit SIMD can regress
  such kernels.
- Keep f64 as the only numerical format.

## 3. Evidence collected before this plan

### 3.1 CodeGraph call path

CodeGraph reports the active generation path as:

```text
maple2c source
  -> from_maple.py::translate_expr
  -> from_maple.py::emit_function
  -> simd.py::simd_body
  -> generated f64x8 grid loop
  -> libxc-rkernel-math::{simd, rmath_fast, helpers}
  -> rmath exact or Fast vector kernel
  -> generated sweep
  -> libxc-reval routing
```

The control points are:

| file/symbol | current responsibility | planned responsibility |
|---|---|---|
| `tools/translate_rayon/from_maple.py::SIMD_EXACT_FUNCS` | exact explicit-SIMD allowlist | measured exact winners only |
| `tools/translate_rayon/from_maple.py::SIMD_RMATH_FAST_FUNCS` | approximate allowlist | exceptional, stricter second tier |
| `tools/translate_rayon/simd.py::rewrite_calls` | maps scalar expressions to vector expressions | use the math crate's exact facade for every supported transcendental |
| `tools/translate_rayon/simd.py::simd_body` | emits fixed `f64x8` loops and padded tails | add eligibility diagnostics and stride-safe polarized support before expanding coverage |
| `crates/kernels-rayon/math/src/simd.rs` | exact `exp`, `ln`, and cube-root-family facade | exact facade for the full rmath surface needed by kernels |
| `crates/kernels-rayon/math/tests/simd_exact.rs` | adapter bit-identity for three families | adapter bit-identity for every exact mapping, including binary calls |
| `bench-vs-libxc` | four-leg end-to-end timing and fingerprints | machine-readable candidate gate and strict per-field reporting |

CodeGraph also found a high-value helper seam: `lambert_w` is called from 28
generated functions, including every order/spin variant of `gga_x_am05`. A scalar
helper call prevents the surrounding grid loop from becoming a complete SIMD
loop even when all elementary calls inside the helper have vector forms.

### 3.2 Current generated call inventory

A static scan of the current generated Rust sources gives these source-site
counts. They are prioritization data, not dynamic profiles.

| elementary call | emitted sites | initial action |
|---|---:|---|
| `sqrt` | 5,928 | keep native; hardware already vectorizes it |
| `ln` | 5,059 | P0 exact-rmath mapping |
| `exp` | 2,788 | P0 exact-rmath mapping |
| `pow` | 2,159 | P1 exact-rmath mapping, then kernel measurement |
| `atan` | 451 | P0 exact-rmath mapping; current exact adapter is stale |
| `erf` | 365 | P1 exact-rmath mapping; rmath is correctly rounded |
| `abs` | 205 | keep native vector operation |
| `tanh` | 131 | P0 exact-rmath mapping |
| `cosh` | 30 | P1 exact-rmath mapping |
| `cos` | 19 | P1 exact-rmath mapping |
| `sin` | 17 | P1 exact-rmath mapping |
| `sinh` | 12 | P1 exact-rmath mapping |

Scalar helper imports appear in fewer functions but can dominate their kernels:

| helper | generated imports | SIMD suitability |
|---|---:|---|
| `xc_mgga_x_br89_get_x` | 46 | divergent fixed-iteration root solver; high risk |
| `lambert_w` | 28 | fixed-state Halley iteration; strong SIMD candidate |
| `xc_e1_scaled` | 26 | piecewise Chebyshev/Clenshaw; strong SIMD candidate |
| `xc_mgga_x_mbrxc_get_x` | 20 | divergent Brent solver; high risk |
| modified Bessel helpers | 20 | piecewise polynomial plus exp/sqrt; medium risk |
| `xc_dilogarithm` | 10 | piecewise Chebyshev plus ln; medium risk |
| `xc_erfcx` | 10 | piecewise rational/exp; medium risk |
| integration helpers | 30 across three pairs | fixed quadrature; medium/strong candidate |
| B-spline helpers | 10 | branch/mask conversion; medium risk |

The largest static counts occur in high derivative and polarized bodies such as
`gga_x_airy lxc_pol`. Candidate ranking must therefore intersect this inventory
with `libxc-reval` routing and the user's requested order; unreachable kernels do
not improve an end-to-end benchmark.

### 3.3 rmath reference and reusable techniques

- Reference checkout: `/home/user/Documents/workspace/rmath`
- Inspected revision: `1d1318a48a436e74217a2eebfb1d09e2a06fb768`
- Worktree state at inspection: clean

The integration should reuse these rmath designs rather than duplicate them:

- generic `Simd` kernels with a `wide` backend, allowing the same exact API to
  accept scalar `f64` or `wide::f64x8`;
- compile-time `BitExact`/`Fast` and `FullRange`/`Finite` policy axes;
- vector main paths followed by `patch_lanes`/`patch_lanes2` for exceptional
  lanes, including zero-mask and all-mask fast paths;
- exact replay of the platform operation schedule, including intentional FMA
  and non-FMA sites;
- table lookup plus polynomial evaluation for exact ports, and table-free
  vector polynomials only in the explicit `Fast` policy;
- masks and selection for piecewise algorithms, with coherent-lane shortcuts
  before the mixed-lane path; and
- deterministic boundary, random-bit, mixed-special, multi-width, accuracy,
  repair-density, and throughput suites.

The rmath `final_native.csv` artifact shows that exact vector math is no longer
limited to exp/ln/cbrt. Representative recorded speedups over scalar libm are:

| function | rmath exact speedup | relevance |
|---|---:|---|
| `exp` | 3.20x | widespread; blocks LLVM vectorization |
| `ln` | 2.53x | widespread; blocks LLVM vectorization |
| `cbrt` | 1.88x | very frequent fractional powers |
| `pow` | 1.46x | frequent but register-heavy |
| `atan` | 9.04x | directly enables exact `lda_c_vwn` SIMD |
| `tanh` | 2.97x | removes a remaining `wide` approximation |
| `sinh` | 4.21x | exact vector mapping |
| `cosh` | 2.55x | exact vector mapping |
| `erf` | 4.55x | correctly rounded and vectorized |
| `erfc` | 4.28x | correctly rounded and vectorized |
| `atan2` | 1.82x | exact binary mapping |

These are reference data, not acceptance data for libxc. They must be rerun at
the pinned revision and then measured inside complete functional kernels.

### 3.4 Live math-facade baseline

`simd_bench` was run in release mode on the current machine while preparing this
plan. Best observed throughput was:

| call | scalar | current exact SIMD | speedup |
|---|---:|---:|---:|
| `exp` | 2.420 ns/element | 0.555 ns/element | 4.36x |
| `ln` | 2.102 ns/element | 0.654 ns/element | 3.22x |
| `cbrt` | 5.675 ns/element | 1.219 ns/element | 4.66x |

This proves that the dependency and native SIMD backend are working. It does not
prove a complete functional wins; large generated bodies can spill registers,
tail/setup costs can dominate short grids, and a helper call can still serialize
the loop.

## 4. Current gap to close first

The exact generator currently has two different numerical meanings:

- `exp`, `ln`, and cube-root powers use `libxc_rkernel_math::simd` and are
  bit-identical to the scalar Rust calls;
- `atan`, `tanh`, `sinh`, `cosh`, `asin`, `acos`, `expm1`, `log1p`, and `atanh`
  use `wide` methods, which are approximate; and
- `pow` and `atan2` use `wide` methods rather than the now-available exact rmath
  vector kernels.

Consequently, the name `SIMD_EXACT_FUNCS` promises more than the mapping actually
guarantees. Fixing this is P0. It is both a correctness cleanup and a speed
opportunity because the current rmath revision has fast exact ports for the
important functions, particularly `atan`.

`lda_c_vwn` is currently in the approximate rmath allowlist. Existing recorded
evidence reports excellent energy agreement but approximately `5.7e-11` on
`vrho`. The exact rmath `atan` result changes the decision: first try
`lda_c_vwn` with fully exact SIMD. Keep the approximate variant only if exact
cannot beat `libxc-Nt` and the approximate variant independently clears the
strict whole-output gate. Approximate speed is not a reason to normalize a new
response-field error.

## 5. Proposed architecture

### 5.1 Three explicit kernel modes

Keep three mutually exclusive modes per `(functional, order, spin)`:

1. **Scalar generated Rust** — baseline and portable fallback; LLVM may
   auto-vectorize it.
2. **Exact explicit SIMD** — `f64x8` grid loop; every nontrivial math call goes
   through the exact math facade and must preserve the scalar-Rust fingerprint.
3. **Approximate explicit SIMD** — opt-in rmath `Fast + FullRange`; fingerprint
   may change, so direct C-libxc output validation is mandatory.

The allowlists remain disjoint. Add a generator assertion that an exact-mode
expression cannot leave behind an approximate `wide` transcendental method.

### 5.2 Narrow exact facade in the math crate

Extend `crates/kernels-rayon/math/src/simd.rs` rather than importing rmath from
generated crates. Every wrapper remains `#[inline(always)]` because an outlined
call inside the grid loop destroys vector throughput.

Planned exact facade:

| facade operation | implementation |
|---|---|
| `exp`, `ln`, `cbrt` | `rmath::{exp,ln,cbrt}(f64x8)` |
| `pow_2_3`, `pow_4_3`, `pow_5_3`, `pow_7_3` | one exact cbrt plus the scalar operation sequence |
| `expm1`, `log1p` | exact rmath vector calls |
| `atan`, `atan2` | exact rmath vector calls; preserve `(y, x)` order for `atan2` |
| `tanh`, `sinh`, `cosh`, `atanh` | exact rmath vector calls |
| `sin`, `cos`, `tan` | exact rmath vector calls when a generated kernel needs them |
| `erf`, `erfc` | rmath correctly-rounded vector calls |
| `pow` | exact rmath binary vector call |
| `asin`, `acos` | exact rmath only after whole-kernel measurement because their table gather can be slower than scalar |
| `sqrt`, `abs` | native `wide` operations; no facade benefit |

Use `FullRange` in production exact mode. `Finite` may be evaluated later only
for a function whose argument range is mechanically proven from its formula and
covered by adversarial tests. Density screening alone is not a proof that every
intermediate exp/log/pow argument is in a finite fast domain.

### 5.3 Target-feature fallback

rmath's exact schedules rely on the intended FMA placement. `wide` does not give
the same schedule when compiled for a target without hardware FMA. Therefore:

- enable explicit exact SIMD only under a target feature set proven by the rmath
  bit-exact suite;
- compile the scalar generated kernel as the fallback for non-FMA targets;
- keep `-C target-cpu=native` in the repository's target `cfg` configuration for
  the reference build; and
- test an AVX2+FMA build separately from native AVX-512. Do not infer AVX2
  behavior from an AVX-512 result.

This is compile-time dispatch. A runtime CPU dispatcher is outside this plan.

### 5.4 Helper functions become lane-generic in priority order

Elementary SIMD alone cannot vectorize a grid loop that calls a scalar helper.
Add vector entry points without changing the scalar API:

```text
helper(f64) -> f64                  existing scalar reference
helper_simd(f64x8) -> f64x8        exact lane-parallel implementation
```

The generator maps the helper name only in exact SIMD mode. The vector helper
must use the same per-lane operation sequence as the scalar helper. Branches
become masks; loops retain their iteration order; no Horner-to-Estrin or
multiply-add rewrite is allowed unless the scalar reference uses it too.

Priority:

1. `lambert_w`: vectorize the initial-region masks and each Halley state update;
   compare fixed 15-step and active-mask convergence forms, retaining only the
   one that matches the scalar result and wins end to end.
2. `xc_e1_scaled`: vectorize the existing Clenshaw recurrences and region masks;
   coherent-lane branches avoid evaluating all coefficient sets when possible.
3. `xc_dilogarithm` and `xc_erfcx`: piecewise polynomial/rational paths using
   exact rmath ln/exp/erf primitives.
4. modified Bessel I/K helpers: preserve the current Chebyshev recurrence and
   scaling branches.
5. fixed quadrature helpers: vectorize across grid points while retaining the
   scalar quadrature-node order inside each lane.
6. B-spline helpers: convert case selection to masks and measure divergence.
7. `br89` and `mbrxc` Brent solvers: last, because per-lane root-finding state,
   large unrolled bodies, and mixed convergence are likely to cause spills. A
   SIMD microbenchmark is insufficient; the consuming MGGA kernel must win.

### 5.5 Stride-safe expansion

The current explicit SIMD set is unpolarized. Before enabling polarized or
multi-component paths:

- teach the generator to load each component with `point * dimension + lane`
  semantics rather than assuming `rho[ip]`-shaped inputs;
- derive the point count as `buffer.len() / dimension`, using the same parsed
  dimension authority as the scalar emitter;
- store every output component with its declared stride;
- test lengths `0, 1, 7, 8, 9, 15, 16, 17` for every supported dimension; and
- compare polarized exact SIMD to scalar Rust bit-for-bit before C-libxc parity.

Do not expand the allowlist to polarized triples until these tests exist.

## 6. Work plan

### Phase 0 — freeze a reproducible baseline

1. Record libxc commit/version, libxc_rs commit, rmath commit, `rustc -Vv`, CPU
   model, active target features, `RUSTFLAGS`, rayon thread count, governor,
   load average, grid seed, grid size, tail layout, and repetitions.
2. Pin the rmath source used by libxc. The sibling path is suitable for local
   development but not a reproducible release. Before completion, use a pinned
   published version, pinned Git revision, or vendored source.
3. Capture exact outputs and performance for:
   - all eight currently explicit-SIMD files;
   - scalar and exact forms of `lda_c_vwn`;
   - every existing `bench-vs-libxc` case;
   - contiguous 40% screened tails and scattered-tail stress; and
   - 1, 2, 4, 8, and 16 rayon threads where available.
4. Store raw samples, not only best-of-N. Report median, minimum, p95, and
   median absolute deviation.
5. Save current Rust and C fingerprints and the worst field/input for every
   cross-check.

Deliverable: a versioned CSV/JSON baseline and environment manifest.

### Phase 1 — complete the exact rmath facade

1. Add exact wrappers listed in section 5.2 to `math/src/simd.rs`.
2. Replace `simd.py`'s approximate `UNARY_EXACT` and `BINARY_METHODS_EXACT`
   entries with calls to the exact facade.
3. Keep `sqrt` and `abs` as native vector methods.
4. Extend `test_simd.py` to assert every scalar spelling, nested call, binary
   argument order, piecewise arm, and identifier-boundary case.
5. Extend `simd_exact.rs` with unary and binary checkers. Cover special values,
   branch boundaries, adjacent representable values, physical ranges, and
   mixed-special vectors.
6. Run rmath's own multi-width bit-exact suite at the pinned revision; the
   libxc adapter tests verify wiring, while rmath's suite remains the algorithm
   authority.
7. Make a generator failure, not a warning, if exact mode leaves an unsupported
   scalar transcendental or approximate `wide` transcendental in the emitted
   body.

Deliverable: exact mode has one unambiguous numerical contract.

### Phase 2 — move `lda_c_vwn` to exact SIMD

1. Generate scalar, exact-rmath, and current Fast variants from the same
   maple2c source without hand-editing emitted Rust.
2. Confirm exact-rmath output has the scalar-Rust fingerprint for `exc` and
   `vxc` unpolarized.
3. Benchmark the four legs against libxc on active, contiguous-tail, and
   scattered-tail grids.
4. Promote exact-rmath if it beats `libxc-Nt` by at least 1.10x and improves or
   matches the current exact baseline.
5. Demote the Fast variant unless it separately passes the whole-output
   `1e-12` gate and provides at least 1.50x over exact-rmath. This higher bar
   pays for the additional numerical risk and maintenance surface.

Deliverable: `lda_c_vwn` wins against libxc with the strongest practical
contract.

### Phase 3 — build a candidate profiler and eligibility report

Add a translator-side report keyed by `(functional, order, spin)` with:

- counts by elementary rmath operation;
- scalar helper calls that prevent SIMD emission;
- estimated scalar transcendental cost from the current microbenchmark table;
- input/output dimensions and polarized status;
- generated statement count and a register-pressure proxy;
- routed/unsupported status and reason; and
- whether every call has an exact vector mapping.

Use static cost only to shortlist. Profile shortlisted kernels on the physical
grid because branches and repeated expressions determine dynamic cost.

Initial queues:

- **P0:** current exact MGGA triples plus exact `lda_c_vwn`;
- **P1:** routed unpolarized `exc`/`vxc` kernels dominated by exp, ln, atan,
  tanh, erf/erfc, or pow and containing no scalar-only helper;
- **P2:** `gga_x_am05` after `lambert_w_simd`, then users of `xc_e1_scaled`,
  dilogarithm, erfcx, and modified Bessel helpers;
- **P3:** polarized and higher derivatives after stride-safe emission;
- **P4:** root-solver and integration-heavy kernels after dedicated helper
  prototypes.

Deliverable: ranked candidates with explicit rejection reasons.

### Phase 4 — exact elementary-function expansion

For each P1 triple:

1. compare scalar Rust with exact explicit SIMD;
2. require an unchanged Rust fingerprint;
3. inspect release assembly for packed arithmetic, inlining, and the absence of
   scalar libm calls inside the point loop;
4. measure single-thread and rayon paths against `libxc-1t` and `libxc-Nt`;
5. retain only candidates meeting the gates in section 8; and
6. record rejected candidates so generator work is not repeated later.

Evaluate `pow` and exact `asin`/`acos` last in this phase. Their microbenchmark
speedups are smaller or negative, and a large exact table walk may increase
register pressure enough to lose in a generated formula.

Deliverable: a minimal expanded `SIMD_EXACT_FUNCS` allowlist.

### Phase 5 — vectorize scalar helpers

Implement section 5.4 one helper family at a time. Each helper gets:

- scalar-vs-vector `to_bits()` tests on valid-domain inputs;
- explicit behavior checks at branch boundaries and invalid inputs;
- coherent-lane, alternating-region, and random-region benchmarks;
- assembly inspection for hidden scalar calls and spills; and
- one consuming functional benchmark against libxc before another helper starts.

If mixed-lane evaluation computes every branch and loses to scalar, use the
rmath pattern: all-lanes coherent shortcuts plus a mixed mask/select path. If
that still loses, keep the scalar kernel and record the rejection.

Deliverable: exact helper SIMD only where a consuming functional wins.

### Phase 6 — polarized and higher-derivative support

Implement the stride-safe emitter work, then promote triples independently.
Never infer `fxc`, `kxc`, or `lxc` performance/accuracy from `exc` or `vxc`.
Higher derivatives amplify math error and increase live-value pressure, so they
have separate measurements and allowlist entries.

The existing oracle does not cover polarized or MGGA directly. Add retained
direct C-libxc tests for each promoted polarized/MGGA triple before production
enablement.

Deliverable: tested polarized/higher-order SIMD support with no dimension
assumptions.

### Phase 7 — optional approximate tier

Only after exact candidates are exhausted:

1. evaluate `rmath::fast` one operation family at a time;
2. keep `FullRange` unless a finite domain is formally established;
3. compare Fast to exact Rust and C libxc on physical, boundary, tail, and MGGA
   von-Weizsaecker-bound corpora;
4. report maximum absolute, relative, and ULP error separately by output field;
5. require at least 1.50x whole-kernel improvement over exact SIMD; and
6. add a Fast allowlist entry only if every applicable finite field clears
   `1e-12` and no existing known offender worsens.

Per-call rmath accuracy tests are necessary but not sufficient. The libxc
formula is the acceptance unit.

Deliverable: a small or empty Fast allowlist with retained evidence.

### Phase 8 — full regeneration, portability, and documentation

1. Regenerate all kernels and the eval/routing layer.
2. Run the full gates in section 9.
3. Measure cold and incremental build time, peak RSS, binary size, and duplicate
   dependency versions. Use a separate `CARGO_TARGET_DIR`; do not clean the
   shared cache destructively.
4. Test native AVX-512, AVX2+FMA, and scalar fallback correctness.
5. Update the permanent performance documentation with accepted/rejected
   candidates, exact rmath revision, environment manifest, fingerprints, field
   errors, and rollback instructions.

Deliverable: reproducible production integration.

## 7. Measurement design

### 7.1 Math microbenchmarks

Expand `math/examples/simd_bench.rs` to cover the complete facade. For each
operation report:

- scalar `rmath`/Rust reference;
- existing `wide` method where one exists;
- exact facade;
- rmath Fast only as diagnostic data;
- ns/element, cycles/element, median, MAD, minimum, and p95;
- output checksum/fingerprint; and
- coherent, boundary-heavy, random-bit, and mixed-special corpora.

Use lengths `1`, `7`, `8`, `9`, `64`, `4096`, and `1 << 20`. Short lengths
expose setup/tail overhead; large lengths expose steady-state throughput.

### 7.2 Functional benchmarks

`bench-vs-libxc` remains authoritative because it times both libraries. Its
four legs are:

| leg | role |
|---|---|
| `libxc-1t` | scalar C baseline |
| `libxc-Nt` | honest caller-parallel C target to beat |
| `rust-1t` | kernel quality without rayon scaling |
| `rust-Nt` | production sweep |

Run at minimum:

- `np = 1_000`, `100_000`, and `1_000_000`;
- 1, 2, 4, 8, and 16 threads where available;
- no screened tail, 40% contiguous tail, and 40% scattered tail;
- at least 15 post-warm-up samples for promoted candidates; and
- alternating candidate/baseline order to reduce thermal and load bias.

The output must include machine-readable per-leg samples, allocations, C heap
delta, fingerprints, and worst error by field and input index.

### 7.3 Assembly gates

For each promoted triple inspect the actual release artifact and record:

- packed lane width (`zmm` on the native reference target, expected AVX2 form
  on the portability target);
- no call to scalar `exp`, `log`, `atan`, `pow`, `erf`, or other replaced libm
  routine in the point loop;
- rmath/facade calls fully inlined;
- stack-frame size and spill loads/stores;
- vector load/store shape and bounds-check branches; and
- no accidental FMA contraction beyond explicit `mul_add` sites.

Assembly is diagnostic. The complete functional timing and numerical gates are
decisive.

## 8. Promotion and rejection gates

### Exact SIMD promotion

- Scalar-Rust and exact-SIMD fingerprints are identical for every tested grid.
- Adapter exact tests and the pinned rmath bit-exact suite pass.
- No new C-libxc field exceeds `1e-12`; known offenders do not worsen.
- `rust-Nt` is at least 1.10x faster than `libxc-Nt` at the primary grid size.
- Candidate improves current Rust by at least 5%, or improves at least three
  common functionals by 3% each through a shared primitive.
- No unrelated benchmark regresses by more than 3% outside measurement noise.
- Hot evaluation remains zero allocation.
- The point loop contains no outlined scalar math call.

### Approximate SIMD promotion

All exact gates that still apply, plus:

- at least 1.50x over the exact-SIMD form;
- energy and every newly exercised response field within `1e-12` against C
  libxc on the retained physical corpus;
- stable behavior near density thresholds and MGGA physical bounds; and
- documented fingerprint change and worst-case input.

### Immediate rejection

Reject or defer a candidate when:

- it wins a microbenchmark but not the consuming functional;
- explicit SIMD loses to LLVM's scalar-source auto-vectorization;
- register spills erase the transcendental gain;
- mixed-lane repair or divergent helper branches dominate;
- correctness depends on an unproved `Finite` promise;
- it changes screening/tail behavior;
- it introduces a new oracle violation; or
- the gain appears only against serial libxc, not `libxc-Nt`.

## 9. Verification matrix

Run after each relevant phase, with the full set before completion:

```bash
# Translator mapping tests
python3 tools/translate_rayon/test_simd.py

# Math facade and adapter bit identity
cargo test --release --manifest-path crates/kernels-rayon/math/Cargo.toml
cargo run --release --manifest-path crates/kernels-rayon/math/Cargo.toml --example simd_bench

# rmath authority at the pinned revision
cargo test --release --manifest-path /home/user/Documents/workspace/rmath/Cargo.toml --test bit_exact
cargo test --release --manifest-path /home/user/Documents/workspace/rmath/Cargo.toml --test accuracy

# Regeneration
python3 tools/translate_rayon/from_maple.py --all
python3 tools/translate_rayon/extract_params.py --json tools/translate_rayon/params.json
python3 tools/translate_rayon/gen_eval.py

# C-libxc and sweep correctness
cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml
cargo run --release -p libxc-reval --bin revalcheck

# End-to-end speed, memory, field errors, and fingerprints
cargo run --release -p bench-vs-libxc --bin xcvs -- --np 100000 --reps 15
XCVS_TAIL=0.4 cargo run --release -p bench-vs-libxc --bin xcvs -- --np 100000 --reps 15
XCVS_TAIL=0.4 XCVS_TAIL_LAYOUT=scatter \
  cargo run --release -p bench-vs-libxc --bin xcvs -- --np 100000 --reps 15
```

Add direct retained C-libxc cases for promoted MGGA and polarized triples because
the existing oracle covers unpolarized LDA/GGA only. Random independent MGGA
arrays are not enough; include physical `tau` and points near the
von-Weizsaecker bound, and report NaN-vs-NaN separately from finite mismatches.

## 10. Expected file changes during implementation

| path | planned change |
|---|---|
| `crates/kernels-rayon/math/src/simd.rs` | complete exact rmath facade |
| `crates/kernels-rayon/math/tests/simd_exact.rs` | unary/binary exact mapping coverage |
| `crates/kernels-rayon/math/examples/simd_bench.rs` | full facade benchmark and robust statistics |
| `crates/kernels-rayon/math/src/{lambert_w,expint_e1,special,bessel,integrate,bspline}.rs` | vector helper entry points, only in measured phases |
| `tools/translate_rayon/simd.py` | exact mappings, helper mappings, eligibility failures, stride-aware emission |
| `tools/translate_rayon/test_simd.py` | full exact/Fast rewrite tests and dimension/tail cases |
| `tools/translate_rayon/from_maple.py` | measured allowlists and candidate-report entry point |
| `bench-vs-libxc` | machine-readable samples, strict per-field output, direct MGGA/polarized cases |
| generated functional crates | regenerated output only; never manual edits |

## 11. Risks and mitigations

| risk | mitigation |
|---|---|
| rmath exactness differs on another platform libm | run host bit-exact tests; fall back to scalar when target contract is not proven |
| sibling path makes builds non-reproducible | pin a published/Git revision or vendor before completion |
| current exact mode still emits approximate `wide` calls | generator hard failure plus mapping tests |
| `pow`/table gathers increase spills | inspect stack traffic and accept only whole-kernel wins |
| SIMD branches evaluate invalid formulas in inactive lanes | repeated-last tail padding, rmath lane repair, helper domain tests, and mask-coherent shortcuts |
| polarized dimension mistakes misalign later points | derive dimensions from core, divide lengths by dimension, test all tail lengths |
| Fast error amplifies in derivatives | complete field-by-field C comparison; separate allowlist per order/spin |
| helper vectorization changes iteration/convergence | preserve per-lane sequence and compare `to_bits()` against scalar helper |
| benchmark noise creates false wins | alternating runs, raw samples, median/MAD, load and environment manifest |
| broad regeneration hides unrelated changes | change one mapping/helper family at a time and review generated diffs by functional |

## 12. Rollback

The scalar kernel remains the universal fallback. Roll back a promoted triple by:

1. removing it from the relevant allowlist;
2. regenerating with `from_maple.py --all` and `gen_eval.py`;
3. rerunning exact adapter tests, oracle/direct-C tests, `revalcheck`, and
   `bench-vs-libxc`; and
4. confirming the stored baseline fingerprint has returned.

A helper SIMD implementation may remain tested but unused when its consumer
does not win. No public API or workspace cache migration is required.

## 13. Completion checklist

- [x] rmath source is pinned and its revision recorded in benchmark metadata.
- [x] Exact mode contains no approximate transcendental mappings.
- [x] `lda_c_vwn` is re-evaluated with exact rmath atan/ln/cbrt and beats
      `libxc-Nt` or is explicitly documented as rejected.
- [x] Candidate report intersects math cost with routing, dimensions, and helper
      eligibility.
- [x] Every exact promotion has an unchanged scalar-Rust fingerprint.
- [x] Every approximate promotion, if any, has retained field-by-field C evidence
      within the stated tolerance and at least 1.50x over exact.
- [x] Direct C tests cover every promoted MGGA/polarized triple.
- [x] Screening, chunk invariance, tails, zero allocation, and known-defect deltas
      are unchanged.
- [x] Native and AVX2+FMA builds are measured; non-FMA fallback is correct.
- [x] Accepted and rejected candidates, build cost, assembly findings, and
      rollback instructions are documented.
- [x] The final production result beats caller-parallelized libxc, not merely
      serial libxc, for every claimed SIMD winner.
