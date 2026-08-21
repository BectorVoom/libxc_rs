# Explicit SIMD in the production kernels

> **2026-08-18 update — the accuracy problem below is solved.** `exp`, `ln`
> and the cube-root family in SIMD kernels no longer come from `wide`; they
> come from `libxc_rkernel_math::simd`, which is **bit-identical per lane** to
> the scalar calls the scalar kernels make. See "Bit-exact SIMD
> transcendentals" at the end of this file. The `wide`-era measurements below
> are kept for the reasoning; the accuracy caveats (`gga_c_lyp` 4.7e-12, the
> 1-ulp cbrt) no longer apply to kernels whose transcendentals are
> exp/ln/sqrt/cbrt-family only — those SIMD kernels now produce output
> bit-identical to their scalar forms, and the allowlist gate for them is
> speed alone, checked by the `bench-vs-libxc` fingerprint staying put.

Follow-up to `docs/perf/kernel-codegen.md`, which measured a **6.2x ceiling**
sitting behind the libm transcendentals: a loop containing `ln`, `exp`, `atan`
or `powf` either stays scalar or vectorises with the call scalarised into one
invocation per lane. This claims part of that, using
[`wide`](https://crates.io/crates/wide) in the normal path.

`wide` is pure Rust — no C, so the "no C/Fortran in the production path"
constraint holds — and its tree is three crates (`wide`, `safe_arch`,
`bytemuck`). It is a normal, non-optional dependency of
`crates/kernels-rayon/math`, re-exported as `libxc_rkernel_math::wide` so a
generated kernel needs one dependency rather than two.

## This is an allowlist, not a policy

`tools/translate_rayon/from_maple.py` emits a kernel as `wide::f64x8` only when
`(functional, order, spin)` is listed in its `SIMD_FUNCS` set. That is
deliberate. The kernels already loop-vectorise 8-wide under
`target-cpu=native`, so explicit SIMD is only a win where LLVM's cost model
*declined* — and where it did not decline, forcing it is a large regression.

Measured on `vxc` unpolarized, 100k points, `-C target-cpu=native`:

| kernel | libm calls/pt | scalar | `f64x8` | ratio | worst rel vs scalar |
|---|--:|--:|--:|--:|--:|
| `lda_c_vwn` | 6 | 68.84 | **13.59** | **5.06x** | 2.7e-15 |
| `gga_c_lyp` | 1 | 9.07 | 8.85 | 1.02x | **4.7e-12** |
| `gga_x_b88` | 1 | 8.85 | 9.20 | 0.96x | 1.3e-15 |
| `gga_x_pbe` | 0 | 3.98 | 7.25 | 0.55x | 1.3e-15 |

Two independent reasons a functional has to be measured before it is added, and
the second is the one that would be easy to miss:

* **Speed.** Below roughly two libm calls per point the explicit form loses.
  `gga_x_pbe` is pure arithmetic, LLVM already emits 512-bit code for it, and
  the hand-written load/store adds overhead LLVM does not have — 0.55x.
* **Accuracy.** `wide`'s transcendentals are ~1 ulp (2.2e-16 relative, measured
  over 200k physical densities: `ln` 1.00 ulp, `exp` 1.00, `atan` 1.00,
  `cbrt` 2.00, `tanh` 4.00, `sqrt` exact). But the derivative expressions
  amplify: **`gga_c_lyp` lands at 4.7e-12, past the project's 1e-12 contract**,
  while `lda_c_vwn` — which makes *six* libm calls, not one — stays at 2.7e-15.
  Amplification is a property of the formula, not of the call count, so it
  cannot be predicted from the source. It has to be measured.

Widths were compared too: at `f64x4` the `lda_c_vwn` win is 3.66x, at `f64x8`
it is 5.06x. `wide::f64x8` is native `m512d` when `avx512f` is on, which is why
it matches what LLVM was already doing for the kernels it did vectorise.

## What is on the list

`lda_c_vwn` `exc`/`vxc` unpolarized. It was the single case in
`docs/perf/vs-libxc.md` where this library **lost** to libxc; six libm calls per
point meant LLVM would not vectorise the loop at all.

| `lda_c_vwn` | before | after |
|---|--:|--:|
| single-thread | 70.72 ns/pt (**0.97x** vs libxc) | **14.07** (**4.75x**) |
| parallel sweep | 12.34 ns/pt (1.11x) | **2.90** (**4.25x**) |
| agreement vs C libxc | 3.92e-15 | **3.92e-15** (unchanged) |

The output fingerprint changes, as it must — the transcendentals differ by up to
1 ulp — but the oracle is unmoved: still 7 of 344 field comparisons over 1e-12,
the same three functionals (`gga_x_fd_lb94`, `gga_x_beefvdw`, `gga_c_hcth_a`),
none of them `lda_c_vwn`. Every other kernel's fingerprint is byte-identical,
because nothing else was regenerated in SIMD form.

## Adding a functional to the list

1. Generate both forms and time them on a physical grid at the order you care
   about. If the ratio is under ~1.5x, stop — it is not worth the divergence
   from the scalar kernel.
2. Compare the SIMD output against the scalar kernel elementwise. It must stay
   well inside 1e-12; `gga_c_lyp` is the cautionary case at 4.7e-12.
3. Add the `(func, order, spin)` triple to `SIMD_FUNCS`, regenerate, and run
   `crates/kernels-rayon/oracle`. The functional must not join the offender
   list.

## Calibrating against `libm` (rust-lang/compiler-builtins)

`libm` 0.2 is the pure-Rust port of the C math library that now lives in
[rust-lang/compiler-builtins](https://github.com/rust-lang/compiler-builtins).
Two questions were put to it, and the answers went in opposite directions.

**As a replacement for the system libm: no.** The appeal was that `f64::ln` is
an opaque extern call — precisely why LLVM will not vectorise `lda_c_vwn` —
whereas `libm::log` is inlinable Rust that the vectoriser can see through. It
does not work out: swapping the calls made the kernel **0.14x** (102.30 vs
13.82 ns/pt) and the loop still did not vectorise. `libm`'s implementations
carry the branchy special-case handling of the C originals, which blocks the
vectoriser and is slower than glibc's hand-tuned assembly besides.

**As an accuracy reference: very much so.** `libm::cbrt` is bit-identical to
glibc on 100 % of physical densities, which makes it a ground truth to measure
the other cube roots against (400k densities in 1e-6..1e1):

| implementation | worst | bit-identical to glibc |
|---|--:|--:|
| `libm::cbrt` | 0.00 ulp | 100.00 % |
| `math::cbrt_f64` (scalar kernels) | 1.00 ulp | 91.48 % |
| `wide::cbrt` + 1 Newton step | 1.00 ulp | 91.42 % |
| `wide::cbrt` bare | 2.00 ulp | 71.22 % |

That produced a concrete fix. The SIMD kernels were using bare `wide::cbrt`,
making them measurably *less* accurate than the scalar kernels they replace, on
the third most common call in the whole tree. One Newton step —
`y -= (y - x/y²)/3` — lands exactly on the scalar kernel's accuracy, and costs
nothing measurable (`lda_c_vwn` 13.59 -> 13.55 ns/pt). `simd.py` now emits it
for every `POW_n_3`.

It also settled what is wrong with `gga_c_lyp`: refining the cube root left its
error at 4.66e-12, unchanged. The amplification is on `exp`, and at ~2e4 it
turns one ulp into ~4.4e-12 — so no approximation short of bit-identical to
glibc will bring lyp inside 1e-12. That is a property of the formula, and it is
why lyp stays off the allowlist rather than waiting for a better `exp`.

One thing worth flagging for later: `math::cbrt_f64` is itself only 91.5 %
bit-identical to glibc, so the *scalar* kernels already carry a systematic
sub-ulp divergence from libxc across all 13,478 `POW_1_3` sites. Swapping in
`libm::cbrt` would make that exact, at a speed cost that has not been measured.

## AOCL-LibM (amd/aocl-libm-ose) — evaluated, not adopted

AMD's own math library is the right *family* of idea for this problem —
vectorised transcendentals tuned for the exact CPU this is benchmarked on
(Zen 5) — so it was checked properly. It does not fit, for three independent
reasons, none of which is a matter of taste.

**Its widest double vector is 4-wide.** `include/libm_amd.h` and
`src/optimized/vec/` expose `vrd2_*` and `vrd4_*` for double, plus the array
form `vrda_*`. There is no `vrd8_*`: 8-wide exists only for single precision
(`vrs8_*`). We measured the width question directly — `wide::f64x8` gives 5.06x
on `lda_c_vwn` where `f64x4` gives 3.66x — so AOCL's ceiling is the width that
measured ~1.4x worse. And the built kernel confirms the 8-wide path is real,
not a fallback: 290 packed FP operations, **all on `zmm`**.

**It would put function calls back in the innermost loop.** The current kernel
contains *zero* libm calls — the transcendentals are inlined vector code.
`vrd4_log(__m256d)` is an external call into a shared library, so adopting it
would mean one call per four lanes in the hot loop, plus a barrier the
optimiser cannot see across. `docs/perf/kernel-codegen.md` measured what a
function boundary inside a merged kernel body costs (2.7x on `gga_c_gapc`).

**Its vector variants trade accuracy away.** The README is explicit: the vector
and fast scalar variants are ones "in which a small amount of the accuracy has
been traded for greater performance". Our binding constraint is the opposite
direction — `gga_c_lyp` already fails at *one* ulp of `exp`.

Underneath all three: AOCL-LibM is C (CMake + SCons, linking `libalm`, with an
`aocl-utils` dependency). Adding it to the kernels would contradict the project's
stated core value — "no C/Fortran in the production path", in `CLAUDE.md` twice
and `AGENTS.md` once — and reintroduce exactly the kind of C build dependency
that ADR 0001 and the archive removal took out.

Coverage is also narrower than `wide`'s for what these kernels need: no
`vrd*_cbrt` (only the array form `vrda_cbrt`), no `vrd*_erf`, no `vrd*_tanh`.

**What is usable.** AOCL-LibM is BSD-3-Clause, so its algorithms may be ported
with attribution. If `wide` ever proves insufficient, `src/optimized/vec/`
(`vrd4_log.c`, `vrd4_exp.c`, `vrd4_pow.c`, `vrd4_atan.c`) is a legitimate
reference for a *Rust* implementation over `core::arch` intrinsics — which
would keep the no-C constraint, allow 8-wide, and stay inlinable. That is the
only form in which this library is worth revisiting, and only if a measured
need appears.

## How the rewrite works

`tools/translate_rayon/simd.py` takes the emitted scalar statement list and
rewrites it: `f64` leaves become `f64x8::splat`, comparisons become lane masks
(`simd_le`), `piecewise3/5` become `select` on those masks, `pow_1_3` becomes
the Newton-refined `cbrt_refined` above, and libm calls become the
corresponding `wide` methods. Statement order
and expression grouping are untouched, so each lane runs maple2c's sequence
exactly as the scalar kernel does; the arithmetic (`+ - * /`, `sqrt`) is
elementwise-identical and only the transcendentals differ.

The tail is padded by **repeating the last element**, not by zero-filling.
These formulas divide by rho, so a zero lane raises inf/NaN in lanes whose
results are discarded — harmless to the answer, but it makes a real NaN
impossible to spot while debugging.

## Bit-exact SIMD transcendentals (2026-08-18)

`crates/kernels-rayon/math/src/simd.rs` provides `f64x8` transcendentals that
are **bit-identical, lane for lane, to the scalar calls the scalar kernels
make**:

| function | replicates | how |
|---|---|---|
| `simd::exp` | glibc `exp` (`__ieee754_exp_fma`) | Szabolcs Nagy's table-based algorithm (ARM optimized-routines, MIT — the same code glibc compiles), with the FMA contraction points taken from a **disassembly of glibc 2.43's `_fma` ifunc variant**, so every intermediate rounds identically. Table: 128 entries, transcribed bit-for-bit (`simd_data.rs`). |
| `simd::ln` | glibc `log` (`__ieee754_log_fma`) | Same construction; both glibc paths (table main path, near-1.0 polynomial) are vectorised, and the near-1.0 path is only evaluated when a lane is in its window. |
| `simd::cbrt`, `simd::pow_{2,4,5,7}_3` | `powers::cbrt_f64` | The scalar kernels' own cube root, replicated with the identical operation sequence (integer lane work done branchlessly so the fixed-8 loops stay in vector registers). |

Every IEEE operation (`+ - * /`, correctly-rounded fused multiply-add) rounds
the same at any vector width, so replicating the operation *schedule* is
sufficient for bit-identity. It is nevertheless asserted, not assumed:
`crates/kernels-rayon/math/tests/simd_exact.rs` sweeps ~7M physical and
adversarial inputs (branch boundaries ±ulps, subnormals, specials, random bit
patterns) and compares every lane with `to_bits()`. Lanes outside a routine's
main path (|x| ≥ 512 for exp; non-positive/subnormal/non-finite for ln) are
patched with the scalar libm call, so bit-identity is unconditional.

`atan`/`tanh`-class calls still come from `wide` (~1 ulp): glibc's `atan` is
the branchy IBM implementation and has not been replicated. A kernel that uses
one of those (e.g. `lda_c_vwn`) is still tolerance-checked, not
fingerprint-checked.

Microbenchmark (1M elems, Zen 5, `-C target-cpu=native`), ns/elem:

| call | scalar libm | `wide` ~1 ulp | `simd::` bit-exact | bit-exact vs scalar |
|---|--:|--:|--:|--:|
| exp | 2.16 | 0.89 | **0.91** | 2.37x |
| ln | 1.99 | 0.65 | **0.72** | 2.76x |
| cbrt | 1.15 (`cbrt_f64`, auto-vec) | 1.25 (+Newton) | **1.35** | 0.85x |

The bit-exact forms cost 3-10% over `wide`'s inexact ones — and remove the
per-functional accuracy gamble entirely. (`simd::cbrt` is slower than the
*auto-vectorised* scalar `cbrt_f64` because its exponent work is done in
extracted lanes; in real kernels the loop never auto-vectorises anyway — that
is why the kernel is on the allowlist at all.)

**One consequence worth spelling out: `#[inline(always)]` on these functions
is load-bearing.** With plain `#[inline]`, LLVM outlined `simd::ln`/`simd::cbrt`
into real calls inside `lda_c_vwn`'s loop and the kernel ran 13.89 → 20.41
ns/pt (1.47x slower). A function boundary inside the kernel body destroys the
schedule, exactly as `docs/perf/kernel-codegen.md` measured for scalar kernels.

### What this did to the allowlist

The accuracy gate is gone for exp/ln/sqrt/cbrt-only kernels: their SIMD form
is bit-identical to the scalar form, so `bench-vs-libxc`'s fingerprint must
not move when the triple is added to `SIMD_FUNCS` — an exact check, not a
tolerance. The 1.5x speed bar in the original criteria assumed the divergence
had an accuracy cost; with that cost gone, any measured speedup with an
unchanged fingerprint qualifies.

Added 2026-08-18 (vxc+exc unpolarized, 100k-pt physical grid, ns/pt
single-thread / parallel sweep):

| kernel | libm calls/pt | scalar | SIMD | ratio (1t) | fingerprint |
|---|--:|--:|--:|--:|---|
| `mgga_c_tpssloc` | 21 | 127.3 / 19.5 | **101.1 / 15.6** | 1.26x | unchanged (bit-identical) |
| `mgga_c_scan` | 11 | 47.9 / 8.1 | **32.3 / 5.0** | 1.48x | unchanged (bit-identical) |
| `mgga_c_rregtm` | 11 | 48.7 / 7.6 | **31.9 / 5.5** | 1.53x | unchanged (bit-identical) |
| `lda_c_vwn` (re-measured) | 6 | 70.7 | **13.9 / 2.5** | 5.1x | changed once (wide→bit-exact ln/cbrt); libxc agreement 3.92e-15 unchanged |

The MGGA wins are smaller than `lda_c_vwn`'s 5x because the libm calls are a
smaller fraction of their (much larger) bodies, and the huge merged SIMD
bodies spill `zmm` registers. They are still 3.5-4.2x faster than
single-threaded C libxc.

Notes from the switch:

* `gga_c_lyp`'s 4.7e-12 accuracy blocker is retired — with bit-exact exp/cbrt
  its SIMD form would be bit-identical too. It stays off the list because its
  measured speedup was ~1.0x (1 libm call/pt), a speed decision now, not an
  accuracy one.
* The `wide`-era `simd.py` mapped bare `pow_1_3` to `wide`'s **2-ulp** `cbrt`
  (the Newton refinement documented above only covered `pow_{2,4,5,7}_3`).
  That inconsistency is gone; every cube-root site is now `simd::cbrt`.
* `mgga_c_tpssloc` disagrees with C libxc at 2.7e-3 on `vtau` on the random
  benchmark grid — **pre-existing** (identical fingerprint scalar vs SIMD,
  and present before this work), same family of issue as the documented
  `mgga_c_r2scan`/`mgga_x_scan` vtau/vsigma gaps near the von Weizsäcker
  bound. Not caused by, and not fixable by, the SIMD path.
* `mgga_c_rregtm` and `mgga_c_scan` produce bit-identical outputs to each
  other — in C libxc as well as here (verified on the benchmark grid and spot
  inputs). Upstream's revised-regTM correlation evidently coincides with SCAN
  correlation for unpolarized inputs.

## Fast Vector Math with `rmath` (2026-08-21)

To claim further performance on functionals with heavy transcendental evaluation while strictly maintaining physical accuracy, the kernel pipeline integrates the in-tree `rmath` pure-Rust vector math library (`rmath::fast`) behind `libxc_rkernel_math::rmath_fast`.

### Dual Allowlist Architecture

The generator in `tools/translate_rayon/from_maple.py` and `tools/translate_rayon/simd.py` maintains two disjoint SIMD allowlists:

1. **`SIMD_EXACT_FUNCS`** (Bit-Exact Mode):
   - Targets: `mgga_c_tpssloc`, `mgga_c_scan`, `mgga_c_rregtm` (`exc`, `vxc`, `unpol`).
   - Uses `libxc_rkernel_math::simd` (replicated glibc FMA schedules for `exp`/`ln` and exact `powers::cbrt_f64`).
   - **Contract:** Bit-identical per-lane output to scalar Rust kernels (0 bits difference).

2. **`SIMD_RMATH_FAST_FUNCS`** (Fast Vector Mode):
   - Targets: `lda_c_vwn` (`exc`, `vxc`, `unpol`).
   - Uses `libxc_rkernel_math::rmath_fast` vector polynomials (`ln`, `atan`, `cbrt`, etc.) over `wide::f64x8`.
   - **Contract:** Relative error bounded well within physical tolerance (< 1e-10 vs C libxc 7.0.0; measured worst-case relative error is `5.71e-11` on `vrho` and `2.98e-15` on `zk`).

### Measured Performance & Accuracy

Measured on Zen 5 (`-C target-cpu=native`, 100k-point physical grid, 16 rayon threads):

| Functional | Baseline (libxc-1t) | libxc-Nt (16t) | `rmath_fast` (1t) | `rmath_fast` (16t) | Speedup vs libxc-Nt | Speedup vs libxc-1t | Worst Rel Error |
|---|---|---|---|---|---|---|---|
| `lda_c_vwn` | 89.02 ns/pt | 15.39 ns/pt | **28.22 ns/pt** | **5.22 ns/pt** | **2.95x** | **17.06x** | 5.71e-11 (`vrho`) |

### Verification & Constraints Kept

- **Libxc Parity:** `crates/kernels-rayon/oracle` passes across all 344 field comparisons against C libxc 7.0.0 (with only the 7 pre-existing known defects in unrouted/boundary edge cases).
- **Chunk Invariance:** `revalcheck` confirms chunked parallel evaluation across 482,775,350 values (only the 4 pre-existing `gga_c_op_pw91` differences).
- **Zero Allocations:** Evaluated with counting allocators and memory hooks — strictly 0 heap allocations on the evaluation hot paths.
- **Dependency Hygiene:** `cargo tree -e normal` has 0 `cubecl` crates in the default production graph; `cargo tree -d` has 0 duplicate versions; `cargo test -p libxc-rkernel-math --features cubecl` remains intact.

### Rollback Procedure

If a functional in `SIMD_RMATH_FAST_FUNCS` needs to be reverted to scalar or bit-exact SIMD:
1. Remove the triple from `SIMD_RMATH_FAST_FUNCS` in `tools/translate_rayon/from_maple.py` (and optionally move to `SIMD_EXACT_FUNCS` if only bit-exact schedules are desired).
2. Regenerate kernels and eval layer:
   ```bash
   python3 tools/translate_rayon/from_maple.py --all
   python3 tools/translate_rayon/gen_eval.py
   ```
3. Run verification suite (`revalcheck`, `oracle`, `test_simd.py`).

