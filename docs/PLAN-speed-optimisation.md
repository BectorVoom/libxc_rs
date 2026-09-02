# Speed Optimisation Plan (2026-08-31)

Successor to the approved 2026-08 plan (`~/.claude/plans/recursive-tinkering-adleman.md`),
updated for what has since been built and measured. Everything here is grounded in the
current tree; file/line references were verified against the code on 2026-08-31.

## 1. Where the tree stands (measured, do not re-derive)

- **2.4–4.9x faster than caller-parallelised C libxc** on GGA/MGGA
  (`docs/perf/vs-libxc.md`); zero heap allocation per evaluation on both sides.
- **The remaining headroom is the scalar libm transcendentals — 6.2x, measured**
  (`docs/perf/kernel-codegen.md`). 78% of kernel files call a transcendental; a libm
  call in the grid loop prevents 8-wide loop vectorisation.
- That headroom is claimed per `(functional, order, spin)` triple via the explicit-SIMD
  allowlist `SIMD_EXACT_FUNCS` (`tools/translate_rayon/from_maple.py:72`). Current
  count: **52 triples / 24 functionals — all exc/vxc, 48 unpol + 4 pol, zero
  fxc/kxc/lxc.**
- **Bit-exact only.** Every SIMD transcendental resolves through
  `libxc_rkernel_math::simd` / the `rmath_bitexact` shadow module to
  `<BitExact, FullRange>` rmath kernels; a SIMD kernel's output is bit-identical to its
  scalar form, so the qualification gate is *fingerprint unchanged + any measured
  speedup*. `SIMD_RMATH_FAST_FUNCS` stays empty (user decision, and the 2026-08-31
  fast-policy incident is why — see `AGENTS.md`).
- **Realistic expectation per accepted triple: 1.5–2.3x, not 4–5x.** Every pre-2026-08-31
  SIMD ratio was accidentally measured on rmath's fast path. The bit-exact win comes from
  removing the libm *calls* so the loop vectorises, not from faster transcendentals
  (bit-exact vector-vs-scalar is only ln 1.5x / exp 2.8x / cbrt 1.8x / atan 1.5x).

### Tooling already built and validated (2026-08-31) — reuse, don't rebuild

| tool | what it does |
|---|---|
| `bench-vs-libxc/src/qual.rs` → `xcqual` | Rust legs + fingerprint, **no C side** — so any family/order/spin is benchable, including fxc/kxc/lxc which `xcvs` cannot time (no matching C entry points wired) |
| `tools/translate_rayon/simd_qualify.py` | Batched, resumable, ledger-backed qualification driver. Applies a batch via `LIBXC_RS_SIMD_EXTRA` (never half-edits the allowlist); `--apply` writes winners into `SIMD_EXACT_FUNCS` |
| `tools/translate_rayon/candidate_profiler.py` | Candidate tiers: **1** unpol exc/vxc (426 triples), **2** pol exc/vxc, **3** unpol fxc, **4** kxc/lxc; ordered hot-functional-first, filtered to ≥2 libm calls/pt |
| `docs/perf/simd-ledger.json` | Every verdict with its numbers — currently 14 entries: 6 accept (1.50–2.31x), 8 `deferred-contention` (pending, retryable) |

Build cost for planning: baseline build ~305 s, a 4-candidate batch ~267 s. A `--batch 12`
sweep of the full tier-1 pool is roughly 36 batches ≈ 3.5–4 h of build+bench wall clock,
resumable at any point.

### Measurement rules (violating these produced two bogus runs)

1. **Measure the baseline beside the batch, not before it.** The driver keeps
   `xcqual-baseline` and re-times it seconds before each comparison. A baseline taken
   before a 5-minute build once reported 26.1x for a kernel that is really 2.04x.
2. **Trust `xcqual`'s `minforeign`** (per-rep, from `/proc/stat` minus own time), not
   `uptime`. `minforeign > ~2` ⇒ verdict `deferred-contention`, which stays pending
   rather than being recorded as a result.
3. Fingerprint comparison is exact (`to_bits()` over every output), not a tolerance.

## 2. Closed levers — do not revisit (all measured ~0 or negative)

- Bounds-check elimination (1–4%, drift-level), LICM hoisting, register-pressure
  scheduling, `powf`→cbrt rewrites (none of the surviving exponents are POW_n_3):
  `docs/perf/kernel-codegen.md`.
- Any function boundary inside a merged kernel body (2.7x regression on `gga_c_gapc`);
  splitting merged kernels; `--cap` grouping. Also `#[inline]` instead of
  `#[inline(always)]` on `simd::` functions (1.47x regression).
- Forcing explicit SIMD on kernels LLVM already vectorises (`gga_x_pbe` 0.55x,
  `gga_x_b88` 0.96x — standing rejections).
- `mul_add` → `a*b+c` (changes the value), bypassing `screened_call` (100% error on
  unguarded correlation functionals), per-array zeroing in `prepare` (5–10% loss).
- Vector width tuning: `f64x8`/AVX-512 already wins by 8% over `ymm` alternatives.
- `libm` (rust-lang/compiler-builtins) as a runtime replacement: 0.14x. AOCL-LibM:
  4-wide max, C in the production path, accuracy traded away — evaluated and rejected.
- `rmath_fast` mode (user decision; no accuracy gate designed).
- Any reassociation of maple2c arithmetic — violates the operation-order contract.

## 3. Workstreams

Ordered by expected value per unit of risk. WS-A and WS-B are independent and can
interleave; WS-C depends on WS-B accepts; WS-D/E are independent of all of them.

### WS-A — Fix the `lda_c_vwn` bit-exact regression (open defect, highest single-case value)

After the fast→bit-exact swap, `lda_c_vwn` SIMD went **13.9 → 31.42 ns/pt**
single-thread (2.5 → 5.70 sweep), back to ~1.11x vs libxc-Nt. It is the only
allowlisted kernel that got *slower* than its recorded best, and it is the classic hot
LDA correlation functional.

Likely causes, in order of probability:
1. Bit-exact `atan` is genuinely expensive (only 1.5x vector-vs-scalar, and vwn makes
   2 atans + 4 lns per point). Note `simd::atan` was wide's ~1-ulp version in the 13.9
   measurement — so part of the delta is the honest price of exactness.
2. Lost inlining through the rmath wrappers — `#[inline(always)]` is known load-bearing;
   check every hop `libxc_rkernel_math::simd::{ln,atan,cbrt}` →
   `rmath_bitexact` → rmath kernel for a plain `#[inline]` or a generic boundary LLVM
   declines to flatten.
3. The near-1.0 / out-of-range patch paths in the bit-exact routines being taken
   per-lane on physical inputs (they should be rare-branch, mask-gated).

Method:
- `objdump` the built `libxc-rkernel-lda-c-vwn` rlib: count `call` instructions inside
  the grid loop of `lda_c_vwn_vxc_unpol`. Zero calls is the required end state.
- Microbench `simd::atan`/`simd::ln` in isolation (the harness in
  `crates/kernels-rayon/math` already exists) against the 2026-08-18 numbers
  (exp 0.91, ln 0.72 ns/elem).
- Fix belongs in `crates/kernels-rayon/math/src/simd.rs` or rmath
  (`~/Documents/workspace/rmath`, path dep) — **not** by falling back to an approximate
  atan.

Gate: fingerprint unchanged; either the timing recovers toward ~14–20 ns/pt, or the
residual is attributed instruction-by-instruction and recorded in
`docs/perf/simd-kernels.md` as the price of bit-exact atan. Both are acceptable
outcomes; an unexplained 2.3x is not.

### WS-B — Finish the tier-1 SIMD sweep (the main aggregate lever)

~412 of the 426 tier-1 (unpol exc/vxc) candidates are undecided. The 6 decided accepts
ran 1.50–2.31x. Procedure per session, on a **quiet box**:

```bash
python3 tools/translate_rayon/simd_qualify.py --tier 1 --dry-run   # what's next
python3 tools/translate_rayon/simd_qualify.py --tier 1 --batch 12  # resumable sweep
python3 tools/translate_rayon/simd_qualify.py --apply              # write winners
python3 tools/translate_rayon/from_maple.py --all && python3 tools/translate_rayon/gen_eval.py
```

- First re-run the 8 `deferred-contention` triples already in the ledger
  (`gga_c_pbe`, `mgga_c_revtpss`, `mgga_c_tpss`, `mgga_x_scan` exc, …) — their
  fingerprints already passed; only the speed verdict is missing.
- Prioritise the hot-functional head of the profiler order: PBE family, B88/LYP
  remainder, SCAN/r2SCAN/TPSS family gaps, PW91/PZ LDA correlation, wB97/HCTH GGAs.
  An interrupted sweep must have covered what real DFT workloads call.
- Requires the untracked symlink before any regen:
  `ln -sfn ../crates/libxc-core/src/model src/model`.
- Watch per-crate build time in the driver's log; skip (record `deferred-build`) any
  triple whose SIMD variant blows the "kernel crates stay cheap to compile" budget.
- After each `--apply` + regen: full verification battery (§4).

Expected outcome: allowlist grows from 52 to whatever measurement supports; each accept
individually lands in the 1.5–2.3x band; every reject is a recorded number, never
retried blind.

### WS-C — Tiers 2–4: polarized, fxc, kxc/lxc

- **Tier 2 (pol exc/vxc):** run for every tier-1-accepted functional. Only 4 pol
  triples exist today. Pol kernels pay the strided-gather cost (see WS-D) — measure
  tier 2 twice if WS-D lands mid-stream, and note which variant a verdict used.
- **Tier 3 (fxc unpol):** hot set only (SCF response uses fxc). Bodies are much larger;
  register pressure is the known killer (the non-vectorised 70% of kernel functions are
  dominated by huge high-order bodies where 8 lanes don't fit 32 zmm registers). The
  measured gate decides; expect a meaningful rejection rate.
- **Tier 4 (kxc/lxc pilot):** 2–3 mid-sized functionals only, to establish viability.
  A clean negative result with numbers closes the tier. Do not attempt the
  100k–400k-instruction `lxc_pol` monsters.

All tiers go through `xcqual`, which is exactly why it was built — `xcvs` has no C-side
fxc/kxc/lxc cases (its MGGA bencher is Vxc-only).

### WS-D — Eval-layer data movement (bit-neutral by construction)

1. **Deinterleave the strided SIMD loads/stores.** Every polarized/multi-dim SIMD
   kernel gathers lane-by-lane in a scalar loop — `load_strided`/`store_strided` as
   emitted by `simd.py` (see e.g. `crates/kernels-rayon/lda/lda_c_vwn/src/exc_pol.rs:41-57`:
   an 8-iteration scalar loop with a `min(np-1)` clamp per lane, per input, per step).
   For stride 2 (pol rho: `rho0`/`rho1` interleaved) two contiguous `f64x8` loads +
   one shuffle produce both vectors; same shape for stores. Pure data movement — the
   lane values are identical, so the fingerprint cannot move. Implement in `simd.py`
   (fast path for stride 2, keep the scalar loop as the general/tail case), regenerate,
   measure on `mgga_x_scan` pol and `lda_c_vwn` pol via `xcqual`. Keep only if the
   number moves; either way, record it.
2. **`lda_c_vwn` scatter-tail 0.66x case** (the one benchmark libxc wins: scattered
   below-threshold points, where libxc's per-point `continue` skips the transcendentals
   and our compute-then-re-zero route cannot). Options: per-lane density mask inside
   the SIMD kernel, or a cheaper `MIN_RUN` for SIMD kernels. Cap the effort — real
   quadratures order points by radial shell, so this layout is synthetic; a documented
   "known trade" is an acceptable close.
3. **Not touched:** `screened_call` semantics and its two-route structure, `par_sweep`'s
   recursive `rayon::join` split, per-chunk zeroing (`crates/libxc-reval/src/sweep_gga.rs:331-409`
   and the lda/mgga twins) — all correctness-bound or already measured at their best.
4. **Flag for user decision, do not build unasked:** partial-output entry points
   (libxc's `xc_gga_vxc` shape) to skip `zk` buffers on response calculations —
   an API-surface addition saving `np*(1+nspin+3)` doubles of traffic.

### WS-E — Bench and record hygiene

1. **`xcvs` C-side coverage:** add fxc to the MGGA bencher and kxc/lxc legs where a C
   entry point exists, so headline *vs-libxc* numbers (not just vs-scalar `xcqual`
   ratios) exist for higher orders. Lower priority than WS-B — `xcqual` already
   unblocks qualification.
2. **Refresh the docs to one consistent story:** `docs/perf/vs-libxc.md`'s results
   table predates the 2026-08-21/31 changes; `docs/perf/simd-kernels.md` still carries
   fast-path-era ratios (kept deliberately, but the header should point at the ledger
   as the source of truth); `AGENTS.md`'s SIMD paragraph should cite the ledger and the
   52-triple (then final) count. Re-run the full `xcvs` table once WS-A/B settle and
   replace the stale numbers in one commit.
3. **Ledger is the source of truth for verdicts** — every doc table should be
   derivable from `docs/perf/simd-ledger.json`.

### Out-of-scope observations worth their own investigation (not speed)

- 48/1237 oracle field comparisons over 1e-12 are structural and pre-existing; hybrids
  top the list (`hyb_gga_xc_apbe0` vsigma 2.4e2). Correctness work, separate plan.
- `revalcheck`'s pre-existing 4-value `gga_c_op_pw91 Lxc Polarized` delta.

## 4. Verification battery (every accepted change)

1. **Fingerprint gate** (primary, exact): `xcqual` per triple — bit-identical to the
   scalar baseline or the triple is rejected.
2. **Math-crate exactness:** `cargo test --release -p libxc-rkernel-math`
   (`simd_exact.rs` ~7M inputs, plus `rmath_free_functions_are_bit_exact_against_platform_libm`).
3. **Oracle parity:** `cargo test --release --manifest-path crates/kernels-rayon/oracle/Cargo.toml`
   — the offender set must not grow (currently 9/1221 fields, all v2rho2/vsigma, none zk).
   On this box: `jobs=1` and narrow the family for MGGA (30 GB limit).
4. **Chunk invariance:** `cargo run --release -p libxc-reval --bin revalcheck` — only
   the pre-existing `gga_c_op_pw91` 4-value delta allowed.
5. **Spot disassembly** on WS-A and one accepted triple per tier: no `call` into libm
   inside the grid loop; loads are indexed `(%reg,%reg,8)` zmm ops.
6. Final before/after timing table into the docs, same box, `minforeign`-filtered.

## 5. Risks

| risk | mitigation |
|---|---|
| Contention corrupts verdicts | `minforeign` guard + baseline-beside-batch are already in the driver; never override a `deferred-contention` by hand |
| SIMD variants of big bodies blow build time | driver skips over-budget builds and records it; monster kxc/lxc bodies are explicitly out of scope |
| Register spill makes big-body SIMD slower | that is what the measured gate exists for; a reject with numbers is a successful outcome |
| Regen drift (emitter vs committed tree, e.g. the `#![allow]` line) | restore via `git checkout`, not a fresh regen; commit emitter and regenerated tree together |
| WS-D shuffle path subtly reorders lane math | it must not touch arithmetic at all — loads/stores only; fingerprint gate catches any slip |

## 6. Definition of done

- `lda_c_vwn` recovered or its bit-exact cost fully attributed in the docs (WS-A).
- Tier-1 pool fully decided in the ledger — no pending triples except documented
  build-budget skips (WS-B); tiers 2–3 decided for the hot set, tier 4 piloted (WS-C).
- Strided-load fast path measured, kept or reverted with numbers (WS-D).
- Docs internally consistent, all tables regenerated from current measurements (WS-E).
