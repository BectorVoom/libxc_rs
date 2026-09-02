# HSE06 and PBE: what the plan found and what changed

Execution record for `docs/PLAN-hse06-pbe.md`, 2026-09-03. Every number here was
measured on this box (AMD Ryzen AI 7 350, 16 threads, `-C target-cpu=native`)
against C libxc 7.0.0 built by the project's own `libxc-sys` at `-march=native
-O3`.

The plan was written to make two functionals faster and leaner. Most of what it
actually bought was correctness, and the largest speed win came from somewhere
the plan had explicitly ruled out.

## 1. Headline

| | before | after |
|---|---|---|
| HSE06 `zk` vs libxc, unpolarized | 8.2e-3 relative | **3.4e-14** |
| HSE06 `zk` vs libxc, polarized | 2.8e-3 | **4.2e-15** |
| `gga_x_wpbeh` at `omega=0.11`, `zk` | 2.4e0 (237 %) | **2.3e-11** |
| `xc_erfcx` vs libxc, worst | ~1.4e13 ulp | **1 ulp** |
| `xc_E1_scaled` vs libxc, worst | ~2.2e13 ulp | **3 ulp** |
| `gga_x_pbe` Vxc unpol sweep | 7.32 ns/pt | **4.04** |
| `gga_c_pbe` Vxc unpol sweep | 15.30 ns/pt | **5.97** |
| HSE06 scratch, polarized, 1 M points | 6.1 GB, per call | **48 MB, once** |

## 2. Four defects, all of them silent

### 2.1 HSE06 had no screening in it

HSE06 is `1.0*wpbeh(w=0) - beta*wpbeh(w=omega_PBE) + 1.0*PBEc`. The two
exchange legs differ only in the screening parameter, so if that parameter does
not reach the kernel they are the *same function* and the whole thing collapses
to `(1-beta)*wpbeh(0) + PBEc` — a PBE0-shaped semilocal part with no screening
anywhere in it.

Three independent gaps each sufficed:

1. the generated dispatch took no runtime parameters and hard-coded
   `PARAM_HYB_OMEGA_0 = 0.0`;
2. `libxc-eval`'s `dispatch_gga_by_id` accepted the auxiliary's `params` and
   discarded it (`_params`);
3. no propagation rule existed for the HSE family, because
   `cargo xtask generate-metadata` deliberately rejects setters that do
   anything but copy — and `hse03_set_ext_params` assigns a constant, a negated
   parent value, and a `mix_coef`.

Nothing caught it. `hybrid_oracle.rs` only queries coefficients,
`gga_oracle.rs` exercises `gga_x_wpbeh` at its own default `omega = 0`, and the
rayon oracle has no hybrids at all.

### 2.2 `xc_erfcx` was the wrong algorithm

`math/src/special.rs` documented its `erfcx_y100` as "the Faddeeva/libxc
algorithm" and a "100-point Chebyshev expansion". It was Abramowitz & Stegun
7.1.26: the five-coefficient `t = 1/(1 + 0.3275911x)` rational fit for `erf`,
whose error bound is 1.5e-7 **absolute**.

`erfcx(x) = exp(x^2) * erfc(x)`, so that absolute error is multiplied by
`exp(x^2)`. At `x = 3.99` the factor is 8.2e6 and the answer was wrong in the
third decimal place: 0.13770 against libxc's 0.13732. A quarter of all sampled
inputs differed.

Replaced by a transcription of libxc's real 100-interval Chebyshev table
(`math/src/erfcx_table.rs`, generated from `faddeeva.c`).

### 2.3 Six `E11_data` coefficients were 1000x too small

`expint_e1.rs`'s hand-written Chebyshev tables had indices 5 through 10 of
`E11_data` written with three extra leading zeros:

| index | libxc | previous transcription |
|---|---|---|
| 5 | 0.00721107776966009185 | 0.00000721107776966009185 |
| 6 | -0.00078104901449841593 | -0.00000078104901449841593 |
| 7 | 0.00007388093356262168 | 0.00000007388093356262168 |
| 8 | -0.00000620286187580820 | -0.00000000620286187580820 |
| 9 | 0.00000046816002303176 | 0.00000000046816002303176 |
| 10 | -0.00000003209288853329 | -0.00000000003209288853329 |

`E11` covers `-4 <= x <= -1`, and the error put `xc_E1_scaled` about 3e-3 out
across that whole interval. All six tables are now generated from
`libxc-master/src/expint_e1.c`.

**Both helpers are reachable only on the screened-exchange path.** At `omega =
0` every screening term drops out, so the one wpbeh test the tree had could not
touch them. They are now compared directly against libxc's own C by
`verify/tests/screening_helpers.rs` — which is the check that should have
existed all along, and does not depend on any functional exercising them.

### 2.4 The SIMD ledger's PBE rejection had gone stale

`gga_x_pbe` carried a standing "do not use explicit SIMD" rejection at 0.55x,
on the stated grounds that LLVM already vectorises it. That was true of the tree
it was measured on.

`pow_1_3` used to resolve to `powers.rs::cbrt_f64` — a branch-free inline
polynomial plus Halley plus Newton sequence, no call in it, which LLVM packed
8-wide along with the rest of the grid loop. Commit 31fd1ff47f repointed
`safe_cbrt` at `rmath::cbrt` and 4395787e90 pinned it to `BitExact`. That is
right numerically — measured over 2 M physical inputs by
`math/examples/cbrt_check.rs`, `rmath::cbrt` is bit-identical to
`f64::cbrt`/glibc on **100 %** of them, which the inline version was not — but
it is an opaque ~9.6 ns/elem **call**, and a call in the grid loop stops the
loop vectorising.

Every kernel the inline cbrt had been carrying lost its vectorisation silently.
Against an unchanged libxc (7.86 ns/pt then and now), `gga_x_b88`'s sweep went
from the documented 2.18 to 9.45 ns/pt. **A fingerprint does not move when a
loop stops vectorising**, so the tree's own gate could not see it.

## 3. What was changed

| area | change |
|---|---|
| `tools/translate_rayon/extract_params.py` | emits `ext_names` + `ext_to_kernel`, the libxc-order → kernel-argument permutation, built **by name**; refuses to emit one unless every metadata default lands bit-for-bit on the kernel default it feeds |
| `tools/translate_rayon/gen_eval.py` | every functional gains `dispatch_with(.., ext: Option<&[f64]>)` and `EXT_TO_KERNEL`; `dispatch` is unchanged, and `None` takes the identical path |
| `crates/libxc-core/src/meta/composite_setters.rs` | new: the libxc `set_ext_params` assignments that are not plain copies, for the five HSE ids, each quoting the C line it transcribes |
| `crates/libxc-eval` | `Functional::apply_composite_setters`; `kernel_ext_params()`; the by-id dispatchers pass ext_params instead of discarding them |
| `crates/kernels-rayon/math` | `erfcx_table.rs` (new, generated); `expint_e1.rs` (regenerated); `special.rs::xc_erfcx` rewritten to libxc's branch structure |
| `crates/libxc-eval/src/eval/workspace.rs` | `with_order` / `ensure_order` right-size the scratch; tolerant cursor splits |
| `crates/libxc-eval/src/eval/mix.rs` | the per-auxiliary `zero_scratch()` removed (7 call sites) |
| `crates/libxc-compat/src/legacy_eval.rs` | one thread-local workspace reused across calls, replacing a fresh superset allocation per `xc_lda`/`xc_gga`/`xc_mgga` |
| `tools/translate_rayon/from_maple.py` | 10 PBE triples added to `SIMD_EXACT_FUNCS` |
| `bench-vs-libxc` | 6 PBE cases, `gga_x_wpbeh`, and 2 HSE06 cases (a new composite-functional leg) |
| `verify/tests/` | `hse06_oracle.rs`, `screening_helpers.rs` |

### Why the permutation is built by name

libxc's `copy_params` (`util.c:94`) writes `ext_params[i]` into slot `i` of the
functional's C params struct, so **struct field order is ext_params order** —
that identity is what makes `set_ext_params_cpy` correct. The kernel's argument
order is a different thing: `from_maple.py` takes it from the maple2c body, and
for **160 of 276** functionals it is a permutation of the ext_params order.
`gga_c_pbe`'s kernel takes `[gamma, BB, beta]` where libxc's array is
`[_beta, _gamma, _B]`. Feeding ext_params positionally would have swapped two
constants in the majority of the library.

18 functionals fail the value-consistency gate and keep their compiled-in
constants: `gga_x_lspbe` and `gga_x_lsrpbe` (whose setter does
`mu += alpha*(1+kappa)`, so the metadata default is not the kernel's), the
`lcgau` family, `lda_x_rae`, and eleven `b97`/`hcth` functionals whose metadata
carries 15 ext_params where libxc's array has 16. They now **reject** runtime
ext_params rather than silently applying a wrong one.

## 4. Measured performance

### PBE, `xcvs`, np = 100 000, minforeign-filtered, ns per grid point

| case | libxc-Nt | rust-Nt before | rust-Nt after | vs libxc |
|---|--:|--:|--:|--:|
| `gga_x_pbe` exc+vxc unpol | 4.84 | 7.32 | **4.04** | 1.20x |
| `gga_c_pbe` exc+vxc unpol | 15.49 | 15.30 | **5.97** | 2.59x |
| `gga_x_pbe` exc+vxc pol | 21.26 | 15.33 | **7.53** | 2.82x |
| `gga_c_pbe` exc+vxc pol | 30.19 | 25.67 | **10.05** | 3.00x |
| `gga_x_pbe` exc+vxc+fxc unpol | 7.82 | 9.29 | **4.81** | 1.63x |
| `gga_c_pbe` exc+vxc+fxc unpol | 34.65 | 22.34 | **12.71** | 2.73x |

Every output fingerprint is byte-identical before and after; the `simd::`
surface is bit-exact by construction, so these are not a tolerance trade.

`gga_x_pbe` unpolarized is the weakest of the six at 1.20x, and that is
expected: it is pure arithmetic plus five cbrts, so once the cbrts vectorise
there is very little else for the library to win on. libxc runs the same
formula with the same glibc cbrt.

### HSE06

| case | libxc-Nt | rust-Nt | vs libxc |
|---|--:|--:|--:|
| `hyb_gga_xc_hse06` exc+vxc unpol | 133.48 | 126.02 | 1.06x |
| `hyb_gga_xc_hse06` exc+vxc pol | 309.37 | 298.05 | 1.04x |

Parity, not a win, and the reason is `gga_x_wpbeh`: HSE06 evaluates it twice,
it is 10x the arithmetic of `gga_c_pbe`, and it is still scalar. Making it
faster is WS-3c/3d below, both of which were out of reach until the kernel was
correct at a non-zero screening parameter. Note this is the first HSE06 timing
that means anything -- before this session both legs were evaluating the same
unscreened function, so any earlier number would have been for a cheaper and
wrong functional.

### HSE06 memory

| | before | after |
|---|--:|--:|
| scratch, unpolarized, np = 100 k | 7 000 000 elems / 56.0 MB | **500 000 / 4.00 MB** |
| scratch, polarized, np = 100 k | 76 700 000 elems / 613.6 MB | **1 000 000 / 8.00 MB** |
| allocations per evaluation | one full scratch | **0 rust allocs / 0 B** |

14x unpolarized, 77x polarized, and the counting allocator reports zero
allocations and a zero `mallinfo2` delta for the second and later calls on a
reused workspace -- the same standard the semilocal cases already met, now met
by a composite. libxc's own side is 0/0 too; it reuses its per-handle buffers.

The polarized figure is the one that mattered: 613.6 MB of scratch per call at
100 000 points, which is 6.1 GB at the million-point grid a real calculation
uses, to hold 4.8 MB of results.

## 5. One thing this uncovered and did not fix

`bench-vs-libxc` reports `gga_x_wpbeh` disagreeing with libxc by 4.5e-7 on
`vsigma`, and HSE06 by 1.2e-6, while the rayon oracle passes 1221 of 1221
fields and `hse06_oracle.rs` sees 5e-11. `verify/tests/wpbeh_domain.rs` was
written to settle which was right, by sweeping `(rho, s)` directly instead of
trusting a random grid. At `omega = 0`, `vsigma` relative error against libxc:

| s | 1e-8 | 1e-5 | 1e-3 | 1e-2 | 5e-2 | >= 0.1 |
|---|---|---|---|---|---|---|
| rel err | 6.3e0 | 1.6e-6 | 1e-9 | 1e-11 | 4e-13 | <= 2e-13 |

`zk` and `vrho` hold 2e-15 across the whole domain. So the disagreement is
confined to `vsigma` at very small reduced gradient, the bench grid draws `s`
uniformly from [0, 3] and lands there, and the oracle grid does not.

It is **pre-existing**: the `gga_x_wpbeh` output fingerprint is byte-identical
before and after this session's erfcx/E1 fixes. It is recorded in `AGENTS.md`
rather than fixed, because diagnosing it properly means going into the
`wpbeh_EG` piecewise on `s` in the maple source, which is a different piece of
work from the one this plan scoped.

## 6. What is not done

- **WS-3a (accumulate straight into the caller's outputs).** Would remove the
  mix scratch entirely rather than shrinking it, and delete the remaining
  serial accumulation passes. Not attempted; it needs an accumulate mode in the
  generated `par_sweep` and a bit-exactness argument about `0 + v == v`, which
  the plan sets out.
- **WS-3c (fusing the two `gga_x_wpbeh` legs).** Everything not depending on
  `omega` is computed twice per HSE06 point. Worth doing only after 3a and only
  if a binding count says the shared prefix is large.
- **WS-3d (SIMD `gga_x_wpbeh`).** Blocked on bit-exact vector `erfcx` and
  `e1_scaled`; both are branch-heavy region selections that would need
  mask-select rewrites. Now that the scalar versions are correct and tested
  against C, this is a well-defined next step rather than a guess.
- **The rest of the SIMD allowlist has not been re-swept** against the stale
  rejection finding in §2.4. `gga_x_b88` alone is sitting at 1.20x where the
  docs claim 3.69x.
- **The other 23 aux-parameter assignments** in libxc (§ `AGENTS.md` known
  gaps) have not been audited for the HSE06 defect.
