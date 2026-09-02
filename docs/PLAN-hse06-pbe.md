# HSE06 and PBE: memory and speed plan (2026-09-02)

> **Status (2026-09-03): executed. Results and revisions in
> [`hse06-pbe-results.md`](hse06-pbe-results.md).**
>
> Read that first. Three findings in this plan were wrong or incomplete, and one
> of its explicit exclusions turned out to be the biggest single win:
>
> * **F6 was wrong.** The build is not cold; `.cargo/config.toml` redirects
>   `target-dir` to `.cache/cargo-target`, which was already warm. Corrected in
>   place below.
> * **F4 understated the problem.** `gga_x_wpbeh` did not merely fail to
>   vectorise -- the two helper functions it needs on the screened path,
>   `xc_erfcx` and `xc_E1_scaled`, were both numerically wrong (the first was the
>   wrong algorithm entirely). No amount of plumbing would have made HSE06 right.
> * **F5 was stale, and in the other direction.** The plan recorded
>   `gga_x_pbe`'s 0.55x SIMD rejection as settled and said "nothing to do at the
>   kernel level". That rejection predates the bit-exact `rmath::cbrt` switch,
>   which turned an inline arithmetic sequence into an opaque call and cost the
>   whole tree its loop vectorisation. Re-qualifying PBE was worth 1.8-2.6x.
>
> The workstream ordering (correctness, then memory, then speed) held up.

Companion to `docs/PLAN-speed-optimisation.md` (the tree-wide SIMD plan). This one is
scoped to two functionals real DFT codes call constantly, and it is ordered by what was
found reading the tree: for HSE06 the first job is not speed at all.

Every claim below was checked against the code on 2026-09-02; line numbers are for that
tree (with the uncommitted formatting-only regen in the working copy).

## 1. What the two names resolve to

| name | libxc id | in this tree | how it is evaluated |
|---|---|---|---|
| PBE | `gga_x_pbe` 101 + `gga_c_pbe` 130 | two routed semilocal kernels | one `par_sweep` each, no scratch |
| HSE06 | `hyb_gga_xc_hse06` 428 | `xc_mix_init` composite: **1.0·wpbeh(ω=0) − 0.25·wpbeh(ω=0.11) + 1.0·PBEc** (`crates/libxc-core/src/meta/generated.rs:10177`), plus a 0.25 short-range HF term the caller adds via `exx_coefficient`/`cam_coefficients` | `Functional::evaluate_gga` → `evaluate_mixed_gga` (`crates/libxc-eval/src/eval/mix.rs:366`): three aux sweeps, each into a scratch buffer, then a serial weighted add |

`gga_x_wpbeh` is the whole cost of HSE06. Per grid point its `vxc_unpol` body makes
3 `exp`, 4 `ln`, 8 `sqrt`, 5 `cbrt`, 1 `erf`, 2 `xc_erfcx`, 3 `xc_e1_scaled` calls and
33 `piecewise3` selects (520 lines; `fxc_unpol` is 1,280 lines, `lxc_pol` 2.9 MB).
`gga_x_pbe vxc_unpol` has **no** transcendental but 5 `cbrt`; `gga_c_pbe` has 1 `exp`,
3 `ln`, 2 `sqrt`, 3 `cbrt`. So HSE06 ≈ 2 × wpbeh + PBEc, an order of magnitude more
arithmetic than PBE, and the ω=0 leg costs exactly as much as the ω=0.11 leg.

## 2. Findings

### F1. HSE06 is numerically wrong today — ω never reaches the kernel

Three independent gaps, any one of which is enough:

1. `crates/libxc-reval/src/funcs/gga_x_wpbeh.rs:15` hard-codes
   `PARAM_HYB_OMEGA_0 = 0.0` and the generated `dispatch` takes no parameters.
2. `crates/libxc-eval/src/eval/mod.rs:37-46` — `dispatch_gga_by_id` receives the aux's
   `params` and discards it (`_params`).
3. `crates/libxc-core/src/meta/generated_propagation.rs` holds 9 propagation rules and
   none for 427/428/479/480/481 (HSE03/06/12/12s/sol). libxc's
   `hse03_set_ext_params` (`libxc-master/src/hyb_gga_xc_hse.c:81-97`) does three things
   the metadata does not encode: `mix_coef[1] = -beta`, `aux[0]._omega = 0`,
   `aux[1]._omega = omega_PBE`.

Net effect: both wpbeh legs run at ω=0 and HSE06 evaluates to
`0.75·wpbeh(0) + PBEc` — a PBE0-like semilocal part with no screening. Nothing catches
it: `verify/tests/hybrid_oracle.rs` covers B3LYP/CAM-B3LYP only, `gga_oracle.rs` tests
wpbeh at its default ω=0, and the rayon oracle has no hybrids. **No timing of HSE06 is
meaningful until this is fixed**, because the fix changes what is computed (and the
two legs stop being identical calls, which any cache/dedupe experiment would otherwise
exploit falsely).

### F2. The mix scratch is 23–128× oversize and is zeroed three times per call

`EvaluationWorkspace::new` (`crates/libxc-eval/src/eval/workspace.rs:170-174`)
allocates the **MGGA superset for all four derivative orders** whatever the family or
order requested, and `zero_scratch` (`:185`) fills all of it before every aux
(`mix.rs:478`). Measured with `Dimensions::total_output_components()`:

| spin | GGA exc+vxc needs | workspace allocates | ratio |
|---|--:|--:|--:|
| unpolarized | 3 doubles/pt | 70 | 23× |
| polarized | 6 doubles/pt | 767 | 128× |

A 1 M-point polarized HSE06 Vxc call therefore allocates **6.1 GB** of scratch and
writes zeros over it three times, to hold 48 MB of results. The C-ABI path makes it
worse: `crates/libxc-compat/src/legacy_eval.rs:402,740,1027` build a fresh
`EvaluationWorkspace` on **every** `xc_gga` call, so the "zero heap allocation per
evaluation" result in `docs/perf/vs-libxc.md` is true of the semilocal cases it
measured and false for every composite. libxc's own `xc_mix_func` mallocs per call
too, but only `np × dims(order)` — we allocate 128× that.

### F3. The mix path serialises three extra passes over every output

`mix.rs:414-428` fills every caller output with zero (one serial pass), and after each
aux `add_opt_n` (`mix.rs:275`) does a serial `d[i] += coeff * src[i]` over every field
(three more passes). Only the kernel sweeps themselves run under rayon. For HSE06 Vxc
polarized that is 4 serial passes × 6 doubles/pt of memory traffic on top of the
arithmetic — invisible on a 100 k grid, dominant on the tail of a large one.

### F4. `gga_x_wpbeh`'s grid loop cannot vectorise as emitted

`xc_erfcx` (`crates/kernels-rayon/math/src/special.rs:211`) and `xc_e1_scaled`
(`expint_e1.rs:228`) are out-of-line scalar functions with 7-way `if/else` region
selection and `exp`/`ln` inside; they are calls in the loop, the same barrier
`docs/perf/kernel-codegen.md` documents for libm. They are also **absent from
`FREE_EXACT`** (`tools/translate_rayon/simd.py:54`), so `simd_body` would refuse to
emit a SIMD wpbeh today. This is the one place in these two functionals where the
tree-wide plan's "6.2× libm headroom" applies, and it needs new bit-exact vector helpers
before the qualification driver can even try it.

### F5. PBE state, from the ledger and docs

- `gga_x_pbe`: standing rejection for explicit SIMD, **0.55×** (`docs/perf/simd-kernels.md:92`)
  — LLVM already loop-vectorises it 8-wide. Nothing to do at the kernel level.
- `gga_c_pbe` `exc`/`vxc` unpol: fingerprint identical, single-thread ratio 4.27×/7.66×,
  verdict **`deferred-contention`** (`docs/perf/simd-ledger.json`, `minforeign` 13.9).
  Never re-measured on a quiet box. Polarized and `fxc` triples untried.

### F6. No benchmark case for either functional (corrected 2026-09-03)

`bench-vs-libxc` (`bench-vs-libxc/src/main.rs:241`) has no `gga_x_pbe`, `gga_c_pbe`
or HSE06 case, and its Rust leg calls the reval dispatch directly, so it cannot time a
composite at all. `xcqual` accepts any `fam:name:order:spin` but likewise only reaches
semilocal kernels.

**Build cost is not a constraint.** An earlier draft of this plan claimed a cold
>40 min build because `target/` holds 936 KB. That was wrong: `.cargo/config.toml`
redirects `build.target-dir` to `.cache/cargo-target`, which holds **41 GB** of warm
artifacts including built `xcvs`, `xcqual` and `revalcheck`. A no-op
`cargo build --release -p bench-vs-libxc` takes **2.3 s**, and the C libxc oracle is
already compiled. Iteration is cheap; the effort estimates below are dominated by the
code, not the toolchain.

### F7. The working tree carries an unrelated 2,613-file diff

It is the emitter's formatting versus a rustfmt'd commit (one `#![allow]` line and
line-wrapping; no arithmetic changes) plus the `load_strided`/`store_strided` unroll
in `tools/translate_rayon/simd.py` (WS-D.1 of the tree-wide plan). Land or stash it
**before** WS-0 so every later diff is real.

## 3. Goals and gates

| goal | gate |
|---|---|
| HSE06 correct | `zk`/`vrho`/`vsigma` (and `fxc`) within 1e-12 of C libxc `xc_gga` on `hyb_gga_xc_hse06`, unpol and pol; wpbeh at ω=0.11 within 1e-12 of libxc after `xc_func_set_ext_params` |
| HSE06 memory | second and later calls on a handle: **0 heap allocations**; scratch ≤ `np × dims(order, spin)` for one aux, or zero if WS-3a lands |
| PBE memory | unchanged: 0 allocations (already true) |
| HSE06 speed | ≥ 2.5× caller-parallel libxc, the band the rest of the `vs-libxc.md` table sits in |
| PBE speed | `gga_x_pbe` and `gga_c_pbe` in the 3–4× band of `gga_x_b88`/`gga_c_lyp` |
| bit-exactness | every semilocal change keeps its `xcvs`/`xcqual` fingerprint; mix-path changes are gated on the C oracle, because the mix arithmetic order is libxc's (`mix_func.c:66`), not maple's |

## 4. Workstreams, in order

### WS-0 — Baseline harness (prerequisite)

1. Resolve F7: commit the formatting-only regen as its own commit (bit-neutral, and the
   fingerprints will prove it) so later diffs are legible.
2. Add `xcvs` cases: `gga_x_pbe` vxc unpol/pol + fxc unpol, `gga_c_pbe` vxc unpol/pol +
   fxc unpol, `hyb_gga_xc_hse06` vxc unpol/pol. The HSE06 Rust leg must go through
   `Functional::new(428)` + `evaluate_gga` with a workspace, since that is the only path
   that exists; the C leg is plain `xc_gga`, libxc mixes natively. Wire the existing
   counting allocator and `mallinfo2` deltas to the HSE06 case — they will show F2
   immediately and are the memory gate for WS-2.
3. One cold `cargo build --release -p bench-vs-libxc` (`jobs = 12`). Record the table
   with `minforeign` filtering, as the tree-wide plan's measurement rules require.

Expected: PBEx/PBEc land near b88/lyp (2–3 ns/pt Nt). HSE06 is the number to beat and
is currently measuring the wrong function (F1), so record it as "pre-fix" only.

### WS-1 — Correctness: route ω into `gga_x_wpbeh` (blocks all HSE06 work)

1. **`gen_eval.py`**: give every generated `dispatch` an explicit
   `params: Option<&[f64]>` argument in `params.json` order, defaulting to the emitted
   constants when `None`. Mechanical regen of the 156 `funcs/*.rs` files and the
   `ten_arm_dispatch_*` macros; bit-neutral because the values are the same constants.
2. **`libxc-eval`**: stop discarding `_params` in `dispatch_{lda,gga,mgga}_by_id`
   (`eval/mod.rs`). `GenericParams` already stores the ext_param defaults per id
   (`functional/params.rs:35`); add the ext_param-name → kernel-param-name map
   (`_omega` → `param_hyb_omega_0`, `_kappa` → `param_kappa`, …) derived from
   `params.json` + libxc's `ext_params` names, and a generic
   `set_ext_params_cpy_omega`-style identity copy. Refuse (return `None` params →
   constants, and log) for any functional whose setter is not an identity copy, exactly
   as `routing.rs::UNSUPPORTED` already refuses to guess.
3. **HSE family rules**: encode `hse03_set_ext_params` as data — a small hand-written
   table in `libxc-core` (`{parent: 428, mix_coef[1] = -p[0], aux[0]._omega = 0,
   aux[1]._omega = p[2]}`) applied in `Functional::propagate_to_aux`, and re-applied on
   `set_ext_params` so changing `_beta`/`_omega_PBE` on the parent works like libxc.
   Same rule serves HSE03/12/12s/sol for free.
4. **Tests**: add HSE06 (unpol + pol, exc/vxc/fxc) to `verify/tests/hybrid_oracle.rs`
   against `xc_gga`; add a wpbeh-at-ω=0.11 case to `gga_oracle.rs` using
   `xc_func_set_ext_params` on the C side. Gate 1e-12 on every field.

Risk to watch: parameter order. Kernel arguments follow `params.json`, which follows
maple2c's parameter struct, **not** the `ext_params` index order — take the mapping
from names, never positions, and add a test that dispatches every routed functional
with its explicit defaults and checks the fingerprint equals the constant path.

### WS-2 — Memory: right-size the workspace and stop re-zeroing

1. **Lazy, exact sizing.** Replace `EvaluationWorkspace::new(np, spin)`'s superset
   allocation with `ensure(family, order, np, spin)` that grows a single `Vec` to
   `np × total_output_components(family, order)` only when a larger request arrives.
   HSE06 Vxc pol drops from 767 to 6 doubles/pt (128×), unpol 70 → 3.
2. **Delete the per-aux `zero_scratch`.** `prepare` + `par_sweep` already zero every
   chunk of every output they write (`sweep_gga.rs:187`, and that zeroing is documented
   as bit-neutral and cache-friendly). The mix-level fill is a leftover from the
   pre-rayon path; removing it removes three full serial passes and changes no value.
3. **One workspace per handle in `libxc-compat`.** Keep an `EvaluationWorkspace` inside
   the `xc_func_type` shim state (`RefCell`; the C API is not thread-safe per handle
   anyway) so repeated `xc_gga` calls allocate nothing after the first. This is the
   project's "reuse workspaces on hot paths" constraint applied to the C ABI, and it
   beats libxc, which mallocs per mix call.
4. **Flagged, not built unasked:** partial-output entry points (skip `zk` on a
   response calculation) — already WS-D.4 in the tree-wide plan; same item.

Gate: counting allocator reports 0 allocations on the second HSE06 call; peak RSS delta
equals the caller's buffers plus `6 × np × 8` B; C-oracle parity unchanged.

### WS-3 — Speed: HSE06 mix structure

Ordered by value per risk; (a) alone removes the scratch entirely.

**(a) Accumulate straight into the caller's outputs.** Kernels already `+=` into
their outputs. Give `par_sweep` an `accumulate` mode (no `zero_outputs`, and
`screened_call` *skips* screened points instead of zeroing them) and a per-sweep
coefficient applied as `out[ip] += coef * value` in the dispatch closure. libxc computes
`out[i] += coef * x[i]` where `x[i]` is `0 + value` (`mix_func.c:54`), and `0 + v == v`
exactly, so `coef * value` then `+=` reproduces libxc's operation order bit-for-bit;
`1.0 * v == v` keeps the semilocal path untouched. Result: three parallel sweeps, zero
scratch, zero serial passes. The C-oracle test from WS-1 is the gate (the fingerprint
gate does not apply — the old path's order was `(0 + v)` then `d += c·v`, which is the
same, so the fingerprint should not move either; if it does, stop and find out why).

**(b) Do not replace the ω=0 leg with `gga_x_pbe`.** It is tempting — wpbeh(ω=0) is
"PBE exchange" — but the wpbeh formula is the Ernzerhof–Perdew hole fit, whose
integrated enhancement factor only approximates PBE's F(s). libxc evaluates the real
thing at ω=0; substituting PBEx changes physics beyond 1e-12. Recorded here so nobody
retries it.

**(c) Fuse the two wpbeh legs into one kernel.** Everything that does not depend on ω
(ρ, s, k_F, `aux2(s)`, `wpbeh_EG`, the s-only piecewise chains) is computed twice today.
An emitter variant that takes `(ω₀, c₀, ω₁, c₁)`, computes ω-independent bindings once
and the ω-dependent tail twice, and accumulates `c₀·v₀ + c₁·v₁` per output keeps every
binding's operation order (a shared binding is the identical expression over identical
inputs), so it is bit-exact against (a) by construction and checkable by fingerprint.
Expected 1.3–1.6× on the wpbeh share. Do this only after (a) and after profiling shows
the shared prefix is a large fraction — `maple2c` output tells you: count bindings
before the first use of `param_hyb_omega_0`.

**(d) SIMD-emit wpbeh (the libm lever, F4).** Needs `simd::erfcx` and
`simd::e1_scaled` in `crates/kernels-rayon/math/src/simd.rs`, bit-exact to the scalar
forms: evaluate all seven Chebyshev regions per lane and select by mask (the
polynomials are fixed, so per-lane results are identical to the branchy scalar; the
`exp`/`ln` inside go through the `BitExact` rmath kernels), and add both to
`FREE_EXACT`. Then `simd_qualify.py --func gga_x_wpbeh` for exc/vxc unpol, then pol,
then fxc unpol. Calibrated expectation from the tree-wide plan: **1.5–2.3× per accepted
triple, not 4–5×**. This is the only item here that touches the math crate; run
`simd_exact.rs` (extended with the two new functions over the wpbeh input range).

### WS-4 — Speed: PBE

1. `gga_x_pbe`: measure only (WS-0). Confirm with `objdump` that the vxc loop has zero
   `call`s and indexed `zmm` loads; if it sits in the b88 band, it is done. The explicit
   SIMD rejection stands.
2. `gga_c_pbe`: re-run the two pending ledger triples on a quiet box
   (`python3 tools/translate_rayon/simd_qualify.py --tier 1 --func gga_c_pbe`), then
   tier 2 (pol exc/vxc) and tier 3 (fxc unpol — SCF response needs it). `--apply`,
   regen, fingerprint + oracle. Expected: accepts in the 1.5–2.3× band.
3. Nothing else. A fused "PBE x+c" sweep is not a libxc functional and would be an API
   invention; callers that want PBE call two functionals in libxc too.

### WS-5 — Verification and record

Per change: `xcvs`/`xcqual` fingerprint (semilocal), C-oracle 1e-12 (mix path and the
HSE06 tests from WS-1), `revalcheck`, `cargo test --release -p libxc-rkernel-math`,
and the rayon oracle (`jobs = 1`, GGA family only on this box). Then extend the
`docs/perf/vs-libxc.md` tables with the six new cases and the HSE06 memory line, and
note in `AGENTS.md` that the zero-allocation claim now includes composites.

## 5. Expected outcome (estimates, replaced by WS-0/WS-5 numbers)

| item | before | after | from |
|---|---|---|---|
| HSE06 result | wrong (ω=0 both legs) | 1e-12 vs libxc | WS-1 |
| HSE06 scratch, pol, 1 M pts | 6.1 GB, zeroed 3× | 48 MB (WS-2) → 0 (WS-3a) | WS-2, WS-3a |
| `xc_gga` on HSE06 handle | 1 large alloc per call | 0 after first call | WS-2.3 |
| HSE06 serial passes per call | 4 | 0 | WS-2.2, WS-3a |
| HSE06 kernel time | 2 × wpbeh + PBEc | ×0.6–0.75 (WS-3c) · ×0.45–0.65 (WS-3d) | WS-3 |
| `gga_c_pbe` exc/vxc | scalar loop, 1 exp + 3 ln | 1.5–2.3× | WS-4.2 |
| `gga_x_pbe` | already 8-wide | unchanged | — |

## 6. Sequence and effort

| step | depends on | size |
|---|---|---|
| WS-0 harness + cold build | — | ½ day + 40 min build |
| WS-1 ω plumbing + HSE tests | WS-0 | 1–2 days (regen of 156 dispatch files is mechanical; the mapping table is the work) |
| WS-2 workspace | — (independent of WS-1, but measure after it) | ½ day |
| WS-3a accumulate mode | WS-1 (oracle gate) | 1 day |
| WS-4.2 `gga_c_pbe` sweep | WS-0 | quiet-box hours, mostly build |
| WS-3d vector erfcx/E1 + wpbeh SIMD | WS-1, WS-3a | 2–3 days |
| WS-3c fused two-ω kernel | WS-3a, profile | 2 days, only if profile says so |

## 7. Risks

| risk | mitigation |
|---|---|
| Parameter order mismatch silently feeds a wrong constant to some other functional | names not positions; fingerprint test of explicit-defaults vs constant path for all 156 routed functionals |
| Accumulate mode changes a bit somewhere | it must not: `0 + v == v`, `1.0 · v == v`; if the fingerprint of any semilocal case moves, the change is wrong, not the gate |
| Someone "optimises" the ω=0 leg to PBEx | §WS-3b, in writing |
| Vector `e1_scaled` differs from scalar on region boundaries | mask-select the scalar's exact comparisons (`<=` chain in the same order); `simd_exact.rs` over 1e7 inputs spanning all seven regions plus the boundaries themselves |
| Contention corrupts `gga_c_pbe` verdicts again | the driver's `minforeign` guard; a `deferred-contention` verdict is never overridden by hand |
| Cold build cost blocks iteration | build once; every later step is incremental. For kernel-only iteration on wpbeh, a standalone `--manifest-path` build of `gga_x_wpbeh` (does not see workspace profiles — CGU 16 by default anyway) is minutes, not 40 |
