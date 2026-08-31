# libxc_rs remediation — plan v4 (the last 83)

**Status:** 2026-08-28
**Supersedes:** v3. v1 records what the original defects were; v2/v3 record the
path from "cannot evaluate anything" to where we are now.
**Context:** written from `pyscf_rs`, which wants `libxc_rs` as its default XC
backend and is now blocked only on its own decision, not on this crate.

---

## 0. State

v3 is complete except R-06.6. The numbers:

| metric | v1 | v2/v3 | now |
|---|---|---|---|
| wired kernels | — | 187 | **423** |
| `UNSUPPORTED` | 110 | 80 | **83** |
| own test suite | (facade untested) | 8 pass | **34 pass, 0 fail** |
| agreement with C libxc | *could not evaluate* | ≤2.14e-16 | **≤2.14e-16, unchanged** |

Wiring 236 additional functionals moved **nothing** in the existing ones: every
`exc`/`vrho`/`vsigma` re-measured after R-02 is **bit-identical** to the value
measured before it, not merely close. `cam-b3lyp` still returns
`(0.33, 0.65, 0.19)`.

### The ratchet needs a comment, not a change

`MAX_UNSUPPORTED = 83` was raised from 80, which reads as a 3-functional
regression and is not one. R-02 replaced single *base-kernel* entries with
several *concrete functional* entries, so the two numbers count different things.
Add a line to `regression_guards.rs` saying so, with the date and the reason —
otherwise the next reader draws the wrong conclusion, and the ratchet's whole
value is that it is trusted.

The same applies to category E (6 → 9): `gga_x_lbm`, `mgga_x_bj06`,
`mgga_x_rpp09` are Becke-Johnson/LB variants that resolution split out of their
base kernels. Benign — but this is the third time it has needed explaining, so
record it once (V4-04).

---

## 1. What is left is essentially one shape

83 entries, but not 83 problems:

| n | group | plan item |
|---:|---|---|
| **34** | `setter is NULL` — libxc declares **0** ext params; defaults live in `_init` | **V4-01** |
| 28 | one-off `*_set_ext_params` transforms | V4-02 |
| 11 | param name mismatch | V4-03 |
| 9 | kernel tree missing an (order, spin) module | V4-04 |
| 1 | `gga_xc_hcth_93` — no ext_params tuple | V4-05 |

**V4-01 alone is 41% of what remains, and it is fully characterised below.**

---

## V4-01 — The `_init` defaults (34 functionals)

**Severity: high value, low risk. Do this first.**

This is the `lda_x` pattern from v1 §D-05c, generalised. It is not a parameter
*problem* at all — the extractor is looking in the wrong place.

### Confirmed shape

Every sampled case has `{0, NULL, NULL, NULL, NULL}` in its `xc_func_info_`
(zero external parameters) and a `_init` function holding the constants:

```console
$ grep -A12 "xc_func_info_gga_k_llp =" libxc-master/src/gga_k_llp.c
  {0, NULL, NULL, NULL, NULL},
  gga_k_llp_init, NULL,
```

The extractor reaches its `setter != "set_ext_params_cpy"` branch only because
the *kernel* takes `param_*: f64` arguments (maple2c emits the generalised form),
while libxc declares none. The setter being `NULL` is a consequence, not the
cause, and the current message points the reader at the wrong thing.

### The `_init` bodies are uniform and scrapeable

Every one sampled (`gga_k_llp_init`, `gga_c_zpbeint_init`, `gga_k_apbe_init`,
`gga_k_ol2_init`, `gga_x_vmt_init`, `gga_x_vmt84_init`) has the same structure:

```c
gga_k_apbe_init(xc_func_type *p)
{
  ...
  params->lambda = 0.0;                 /* (1) unconditional base assignments */

  switch(p->info->number){              /* (2) per-functional overrides       */
  case XC_GGA_K_APBE: params->kappa = 0.8040; params->mu = 0.23889; break;
  case XC_GGA_K_TW1:  params->kappa = 0.8209; params->mu = 0.2335;  break;
  case XC_GGA_K_TW2:  params->kappa = 0.6774; params->mu = 0.2371;  break;
  ...
```

Note this also explains why one `_init` serves several `UNSUPPORTED` entries:
`gga_k_apbe_init` covers `apbe`, `apbeint`, `revapbe`, `revapbeint`, `tw1..tw4`
— eight of the 34 from a single scrape.

### Algorithm

1. When the info block's `npar == 0`, **do not consult the setter.** Resolve
   from `_init` instead.
2. Locate the `_init` named in the info block; parse its body for
   `params->NAME = EXPR;`.
3. Assignments outside the `switch` form the base map; each
   `case XC_<UPPER_NAME>:` arm overrides it for that functional, up to `break`.
4. Map `NAME` → `param_NAME` and match against the kernel's parameter list,
   reusing the existing name-normalisation.
5. Evaluate `EXPR` with the **same const-expression pass built for category A** —
   these need it: `X_FACTOR_C*0.0044188` and `0.0253/(X_FACTOR_C*0.0044188)`
   appear, and `X_FACTOR_C` is a plain literal macro
   (`util.h:211`, `0.9305257363491000250020102180716672510262`).
6. Anything that does not parse cleanly stays in `UNSUPPORTED` with a **new,
   accurate** reason (`"defaults are in <fn>_init and could not be scraped: …"`),
   never a guess.

### Also fix the message

`"setter is NULL, not a supported copy set"` describes a symptom. For
`npar == 0` it should say the defaults are in `_init` — that is what sent v1's
analysis down the wrong path for `lda_x`, and 34 entries currently repeat it.

**Acceptance:** `gga_k_llp`, `gga_k_apbe`, `gga_k_tw1` and `gga_c_zpbeint`
evaluate and match C libxc 7.0.0 to 1e-14 on a fixed block; `UNSUPPORTED` drops
by ~34.

---

## V4-02 — One-off `*_set_ext_params` transforms (28)

**Severity: medium. Genuine work, correctly deferred until now.**

| n | setter | functionals |
|---:|---|---|
| 7 | `N_set_ext_params` | `gga_k_absp1..4`, … |
| 3 | `T_set_ext_params` | `lda_xc_ksdt`, `lda_xc_corrksdt`, `lda_xc_gdsmfb` |
| 3 | `pbe_lambda_set_ext_params` | `gga_x_lambda_{ch,lo,oc2}_n` |
| 3 | `ssb_set_ext_params` | `gga_x_ssb`, `gga_x_ssb_d`, `gga_x_revssb_d` |
| 3 | `s12h_set_ext_params` | `hyb_gga_x_{cam_s12g,cam_s12h,s12h}` |
| 3 | `lcgau_set_ext_params` | `hyb_gga_x_lc{,2}gau`, `…_core` |
| 2 | `mpw91_set_ext_params` | `gga_x_mpw91`, `gga_x_pw91_mod` |
| 2 | `scan0_set_ext_params` | `hyb_mgga_x_{scan0,revscan0}` |
| 1 | `hyb_mgga_xc_b94_hyb_set_ext_params` | — |
| 1 | `lc_tmlyp_set_ext_params` | — |

These genuinely *transform* values on the way into the params struct, so
refusing rather than guessing remains correct.

Port highest-multiplicity first (`N_set_ext_params` = 7). Each gets a named Rust
function with the C source quoted in its doc comment, plus an extractor
whitelist entry mapping setter name → transform. **No generic "looks like a copy"
fallback.**

**Acceptance:** every ported setter has a test against C libxc for at least one
functional that uses it.

---

## V4-03 — Param name mismatches (11)

Now fully diagnosable — the truncation fix in v3 did its job. Three distinct
sub-shapes:

**(a) Trivial rename — 2.**
`gga_k_pg1`: kernel `param_pg_mu` vs libxc `param_mu` (kernel prefixes with the
functional stem). `hyb_mgga_x_js18`: `param_hyb_coeff_0` vs `param_a`.
One alias each, with a comment citing the C line.

**(b) Array-flattening family — 7.** `hyb_gga_xc_wb97{,x,x_d,x_d3,x_v}`,
`hyb_mgga_xc_gas22`, `hyb_mgga_xc_wb97m_v`: kernel `param_c_ab_0..4`,
`param_c_ss_0..4`, `param_c_os_0..5`. Same class as the `param_a[0]` fix already
shipped — likely one more normalisation rule covers all seven.

**(c) ⚠️ Index skew — 2. Handle individually, do not alias in bulk.**

* `hyb_mgga_x_m06_sx`: kernel `param_d_0..d_5` (six) vs libxc
  `param_b0, b1, b2, b4, b5, …` — **`b3` is absent from that list.** If the
  kernel and libxc disagree about which slot holds which constant, a positional
  alias produces silently wrong numbers. This is the one failure mode this
  category must not have.
* `gga_x_s12g`: kernel needs `param_bx`, libxc declares **none**
  (`libxc-only=[]`). That is the V4-01 shape, not a rename — check whether its
  default lives in `_init`.

(The `gga_k_lgap` / `param_mu3` case flagged in v3 is **resolved** — good, that
was the other dangerous one.)

**Acceptance:** each of the 11 either matches C libxc to 1e-14, or carries a
reason specific to it. For (c), the justification must cite the C source line
that establishes the pairing.

---

## V4-04 — Missing (order, spin) modules (9) — confirm, then record

`gga_x_lb`, `gga_x_lbm`, `lda_c_pk09`, `lda_xc_tih`, `mgga_c_b94`,
`mgga_x_2d_prhg07_prp10`, `mgga_x_bj06`, `mgga_x_rpp09`, `mgga_x_tb09`.

This category has grown twice (4 → 6 → 9) and each time the benign explanation
held: functionals clearing an earlier check and reaching the module check, plus
base-kernel splits. **Settle it once** rather than re-explaining:

1. Diff membership against the v1/v2 lists (recover from `routing.rs` history).
2. Confirm each is genuinely missing a generated module rather than a
   regeneration having dropped one.
3. Record the finding in a comment next to the category, so growth here is
   self-explaining next time.

Then decide: regenerate the missing modules, or accept them as permanently
unsupported with a citation. Several (`bj06`, `rpp09`, `tb09`) are
Becke-Johnson potentials that have no energy functional at all — for those,
"missing exc module" may be *correct* and should be stated as such rather than
looking like a gap.

---

## V4-05 — `gga_xc_hcth_93` (1)

"no ext_params tuple in `xc_func_info_`". Singleton; check whether the info
block genuinely omits the tuple or the regex misses a formatting variant. Likely
five minutes.

---

## V4-06 — The C-libxc oracle test *(carried from v3 R-06.6 — now the top gap)*

**Severity: high. This is the only v3 item still open, and it guards the crate's
strongest claim.**

libxc_rs agrees with C libxc to **≤2.14e-16, four quantities bit-identical**.
Nothing in this repository tests that. The `oracle_*.rs` files test *invariants*
(signs, monotonicity, shapes) — the name suggests otherwise, which is worth
renaming.

The measurement currently lives in `pyscf_rs`, an unrelated downstream project.
That is a fragile home for libxc_rs's headline property.

`libxc-sys` is already a workspace member. Add an oracle-gated test comparing
the facade against it over a fixed `(rho, sigma)` block for the tier-1 corpus,
asserting **1e-15** — matching the tolerance `facade_eval.rs` now uses, and
~100× above the observed error.

**Then put the claim in the README.** "Bit-exact against C libxc 7.0.0" is this
crate's most defensible property and nothing currently states it.

---

## 2. Suggested order

| step | item | why here | size |
|---|---|---|---|
| 1 | **V4-06** C-libxc oracle + README claim | protects everything already won; nothing defends it today | small |
| 2 | ratchet + category-E comments (§0, V4-04) | two comments; stops recurring misreadings | trivial |
| 3 | **V4-01** `_init` defaults | 34 functionals, fully specified, low risk | medium |
| 4 | **V4-03 (a)+(b)** renames + array flattening | 9 of 11, mechanical | small |
| 5 | **V4-05** singleton | five minutes | trivial |
| 6 | **V4-02** one-off setters, by multiplicity | 28, genuine per-setter work | medium-large |
| 7 | **V4-03 (c)** the two index-skew cases | needs C-source justification, not speed | small, careful |
| 8 | **V4-04** decide regenerate vs document | needs the step-2 finding first | unknown |

Steps 1–5 are a day and take `UNSUPPORTED` from 83 to roughly **39**. Step 6 is
the long tail.

---

## 3. What is not wrong

* **The kernels** reproduce C libxc's arithmetic to ~1 ulp — and R-02's 236 new
  wirings changed no existing value by a single bit.
* **The metadata, registry and hybrid-coefficient surface** are correct where
  checked; ≥600 ids construct in both spins.
* **The `UNSUPPORTED` discipline** — refuse rather than guess — is the reason
  this crate is trustworthy. Every item above must preserve it. A functional that
  cannot be resolved safely belongs in that list with a *specific, accurate*
  reason. V4-01's message fix matters for exactly that reason: a wrong reason is
  worse than a vague one, because it sends the next reader somewhere false.

---

## 4. Downstream

`pyscf_rs` is ready and waiting on a decision only:

* `crates/pyscf-dft/src/xc_backend.rs` — `XcBackend::default` documents the
  switch criteria; the libxc arms of `eval_uks`, `parse`, `family` and
  `rsh_and_hybrid_coeff` are written and verified.
* `crates/pyscf-dft/tests/xc_eval_bitexact.rs::libxc_backend_cannot_evaluate_yet`
  — a deliberate trip-wire that **is now firing**, asserting a failure that no
  longer occurs. `--features libxc`-gated, so the default build is unaffected.

Because the two libraries agree to ~1 ulp, switching should **eliminate** rather
than shrink the 4.7e-7 Ha functional-parameterisation gap that currently
separates `pyscf_rs` from a default-configured upstream PySCF — which would let
its Phase-12 periodic-DFT gate compare like with like for the first time.
