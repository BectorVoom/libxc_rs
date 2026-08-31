# libxc_rs defect remediation — plan v3 (remaining work)

**Status:** 2026-08-28
**Supersedes:** v2 (`PLAN-defect-remediation-v2.md`), written before end-to-end
evaluation was confirmed. v1 remains the record of what the defects were.
**Context:** written from `pyscf_rs`, which wants `libxc_rs` as its default XC
backend.

---

## 0. Verified state — the library works, and it is bit-exact

v2 was written with one thing unconfirmed: whether the library *evaluates a
number*. It does, and better than expected.

### Cross-check against C libxc 7.0.0

Measured from outside libxc_rs entirely (`pyscf_rs`'s `XcBackend::Libxc` against
upstream PySCF's C libxc), same density block, values compared at full f64
precision:

| xc | exc | vrho | vsigma |
|---|---|---|---|
| slater, | 1.9e-16 | 1.7e-16 | — |
| lda,vwn | 1.7e-16 | 1.5e-16 | — |
| pbe | **0** | 1.5e-16 | **0** |
| blyp | 1.7e-16 | 1.6e-16 | 1.5e-16 |
| b3lyp | 2.1e-16 | 1.9e-16 | 2.1e-16 |
| pbe0 | 2.1e-16 | **0** | **0** |

**Worst relative deviation: 2.14e-16 — under one ulp. Four quantities are
bit-identical.**

Also confirmed: Slater exchange matches its analytic closed form to **2.8e-17**
(no oracle involved); UKS returns distinct `vrho_a`/`vrho_b` on an asymmetric
density; `cam-b3lyp` returns `(omega, alpha, hyb) = (0.33, 0.65, 0.19)`, exactly
upstream's values.

**This is a stronger property than "compatible reimplementation".** At ~1 ulp,
libxc_rs reproduces C libxc's arithmetic, which makes it a drop-in for
bit-reproducibility work, not merely for accuracy work. It is worth stating in
the README — it is the crate's most defensible claim and nothing currently says
it.

### v1/v2 defect status

| defect | status |
|---|---|
| D-01 facade cannot reach kernels | **fixed & verified** |
| D-02 stale `oracle-*` messages | **fixed** |
| D-03 `construct_params` 1-of-649 | **fixed & verified** (`registry_all` constructs ≥600) |
| D-04 propagation, indistinguishable errors | **fixed & verified** (`PropagationConflictCause`; all 9 parents construct) |
| D-05 unwired functionals | **110 → 80**; tier-1 corpus wired |
| v2 R-00 (eval might not work) | **void** — see above |

### Test suite status — more complete than v2 credited

Re-read after the fact; v2 undersold this:

| test | asserts | v2 item |
|---|---|---|
| `facade_eval.rs` (4) | Slater `zk`/`vrho` vs **analytic closed form at 1e-12**; PBE sign invariants; B3LYP aux count + `exx_coefficient == 0.2` | **R-06.2 DONE** |
| `regression_guards.rs` (2) | 9-name `TIER1_CORPUS` absent from `UNSUPPORTED`, **and** each evaluates without error | **R-06.3 DONE** |
| `registry_all.rs` | every registered id constructs, both spins; ≥600 constructible | covers D-03/D-04 |
| `propagation.rs` (3), `ext_params.rs` (1) | pass | — |

All pass: **8/8** across `facade_eval` + `propagation` + `ext_params`.

Note `TIER1_CORPUS` includes `mgga_x_tpss`, which is more ambitious than the
tier-1 set v2 proposed — good.

---

## R-01 — `cargo test` still skips the facade suite *(unchanged, now urgent)*

**Severity: high. Still the top item, and it has now cost real time twice.**

The root package is not in `default-members`, so:

```console
$ cargo test --test facade_eval
error: no test target named `facade_eval` in default-run packages
```

Everything in §0's "test suite status" — the analytic Slater floor, the tier-1
guard, the registry sweep — is invisible to a bare `cargo test`.

**New evidence that this is not merely theoretical.** Verifying your fixes
required `cargo test -p libxc_rs --release …`, which builds the 156-crate kernel
tree in libxc_rs's own target directory. That **timed out twice at 40 minutes**
before reaching a single test. The same suite in debug completed in seconds.
A contributor who types the obvious command sees green having run nothing; a
contributor who types the correct one may wait 40+ minutes.

### Fix

1. Add the root package to `default-members`. The exclusion was a
   rust-analyzer indexing optimisation (per the comment above `default-members`)
   and should not silently redefine what `cargo test` means.
2. Add a CI job running `cargo test -p libxc_rs --all-targets` **in debug**.
   These are correctness tests; the release profile buys nothing and costs
   40+ minutes.
3. Correct the build-cost note in `Cargo.toml`/`AGENTS.md`: a cold **release**
   build of the kernel tree is **>40 min**, not the ~12 min previously recorded.
   Debug is minutes. The ~12 min figure came from a partially-cached build and
   should not be quoted as the cold cost.

**Acceptance:** a bare `cargo test` at the repo root executes `facade_eval.rs`.

---

## R-02 — D-05 category C (30): base kernels, not a regex miss

**Severity: medium-high. Largest remaining category.** *(unchanged from v2 —
restated because the root cause was corrected there and it matters.)*

v1 guessed `_INFO_RE` was failing to match. **Wrong.** These 30 kernel
directories are **base kernels shared by several concrete functionals**, so
`xc_func_info_<dirname>` does not and will never exist:

```console
$ grep -ohE "xc_func_info_gga_x_hjs\w*" libxc-master/src/*.c | sort -u
xc_func_info_gga_x_hjs_b88     xc_func_info_gga_x_hjs_pbe
xc_func_info_gga_x_hjs_b88_v2  xc_func_info_gga_x_hjs_pbe_sol
xc_func_info_gga_x_hjs_b97x

$ grep -ohE "xc_func_info_gga_x_kt\w*" libxc-master/src/*.c | sort -u
xc_func_info_gga_x_kt1
```

Same shape for `gga_xc_b97` (→ `gga_xc_b97_{3c,d,gga1}`, `hyb_gga_xc_b97*`),
`gga_x_vmt`, `gga_x_s12`, `gga_x_lcgau`, and the rest.
`extract_params.py` keys on the kernel directory name (`infos.get(func)` where
`func = d.name`), which for a base kernel matches nothing.

### Fix

Change the extractor's unit of work from *kernel directory* to *concrete
functional*:

1. Build the reverse map first: for every `xc_func_info_<name>` in the vendored
   source, determine which kernel directory implements it. Resolve it from the
   `#include "maple2c/..."` line in each `.c` file — **not** by string prefix, so
   a mis-prefixed name cannot silently bind to the wrong kernel.
2. Emit one param set per concrete functional, all sharing the base kernel.
3. Where a base kernel serves N functionals, that is N newly reachable registry
   entries — so this category is worth more than its count of 30.

Keep the existing safety rule: any concrete functional still failing the
literal/name/setter checks stays in `UNSUPPORTED` with its own reason.

**Acceptance:** `gga_x_hjs_pbe` and `gga_xc_b97_d` evaluate and match C libxc to
1e-14; `UNSUPPORTED` no longer contains bare base-kernel names.

---

## R-03 — D-05 category D (33): custom ext-param setters

**Severity: medium.** Down from 37. The remainder are genuine — these setters
*transform* values on the way into the params struct, so refusing rather than
guessing remains correct.

* **Tier 1 — the `_cpy_*` family (~17).** A small fixed set:
  `set_ext_params_cpy_omega` (14 functionals on its own), `_cpy_cam`,
  `_cpy_cam_sr`, `_cpy_lc`, `_cpy_exx`. Port each as a named Rust function with
  the C source quoted in a doc comment, and extend the extractor with a
  whitelist mapping setter name → transform.
* **Tier 2 — one-off setters (~16).** `bn05_set_ext_params`,
  `case21_set_ext_params`, `csc_set_ext_params`, `lsrpbe_set_ext_params`,
  `lspbe_set_ext_params`, `T_set_ext_params`, `N_set_ext_params`, … Port on
  demand, each needing its own reading of the C.

Do **not** add a generic "assume plain copy if it looks like one" fallback.

**Acceptance:** each ported setter has a test against C libxc for at least one
functional that uses it.

---

## R-04 — D-05 category B (9): remaining name mismatches

**Severity: low-medium, but one entry needs care.** Down from 20; the
`param_a[0]` → `param_a_0` subscript class is fixed.

**Do this first, it is one line:** stop truncating. `only_k[:4]` / `only_l[:4]`
in `extract_params.py` discards exactly the evidence needed to diagnose the rest.

Then resolve individually:

| functional | kernel-only | libxc-only | note |
|---|---|---|---|
| `gga_k_lgap`, `gga_k_lgap_ge` | `param_mu_0` | `param_mu3` | **look here first** |
| `lda_c_pz` | `param_beta1_0`, `param_beta2_0` | (truncated) | likely subscript |
| `mgga_c_tpss`, `mgga_c_revtpss` | `param_C0_c_0…3` | (truncated) | likely subscript |
| `gga_xc_th1` | `param_omega_0…11` | (truncated) | likely subscript |
| `gga_c_bmk`, `gga_c_sogga11`, `mgga_x_task` | `param_*_ss_*`, `param_*_a_*` | (truncated) | genuine divergence |

`param_mu_0` vs `param_mu3` are **not the same index**. If kernel and libxc
disagree about which slot holds which parameter, a naive alias produces silently
wrong numbers — the one failure mode this category must not have. Confirm each
pairing against the C source and add an explicit alias with a citing comment.

**Acceptance:** each of the 9 either matches C libxc to 1e-14, or carries a
reason specific to it rather than a truncated list.

---

## R-05 — D-05 categories A, E, F (8)

**A — one left.** `gga_k_dk`:
`['0.95*KINX', '14.281111*KINX*KINX', '-19.57962*KINX*KINX*KINX']`. Needs macro
expansion *then* repeated multiplication. If the const-expression evaluator is
already a whitelisted-AST walker, this is one more node type.

**E — six, up from four. Check this before anything else in R-05.**
`gga_x_lb`, `lda_c_pk09`, `lda_xc_tih`, `mgga_c_b94`, `mgga_x_2d_prp10`,
`mgga_x_tb09`. The count *rising* during a round of fixes is the only possible
backslide on the board. Benign reading: funnel movement — two functionals that
previously failed an earlier check now pass it and reach the module check.
Non-benign: a regeneration dropped two modules. Diff against the v1 membership
(recover it from git history of `routing.rs`) to settle it.

**F — one.** `mgga_x_tau_hcth`: arrays `pure_names`/`tHCTH_val` not found. Likely
the same class as R-02 — check before treating it as its own problem.

---

## R-06 — Lock in what has been won

**R-06.2 (numeric floor) and R-06.3 (tier-1 guard) are DONE.** What remains:

**R-06.1 — make them run.** See R-01. Without it the rest of R-06 is decorative.

**R-06.4 — add the coverage ratchet.** No assertion currently bounds
`UNSUPPORTED`'s size, so the count can drift upward unnoticed — which is
precisely the ambiguity R-05's category E now sits in. Add:

```rust
// Ratchet: lower this as functionals are wired; never raise it.
// 2026-08-28: 80 (was 110).
const MAX_UNSUPPORTED: usize = 80;

#[test]
fn test_unsupported_count_does_not_regress() {
    let n = libxc_reval::routing::UNSUPPORTED.len();
    assert!(n <= MAX_UNSUPPORTED,
        "UNSUPPORTED grew to {n}, ratchet is {MAX_UNSUPPORTED}");
}
```

**R-06.5 — assert bit-exactness, not just correctness.** `facade_eval.rs` uses
`1e-12` against the analytic form. The measured agreement is **2.8e-17** — five
orders tighter. A 1e-12 tolerance would not notice a real regression to 1e-13.
Tighten the analytic assertions toward the achieved precision (`1e-15` is safe
and still ~100× above the observed error), so the suite defends the crate's
strongest property instead of a much weaker one.

**R-06.6 — a C-libxc oracle test.** The bit-exactness in §0 was measured from
`pyscf_rs`, which is not a durable home for it. `libxc-sys` exists in this
workspace; an oracle-gated test comparing the facade against it over a fixed
block would keep the claim honest inside the crate that makes it.

---

## 7. Suggested order

| step | item | why here | size |
|---|---|---|---|
| 1 | **R-01** default-members + debug CI + fix build-cost note | cheapest; already cost 80 min of wall clock | small |
| 2 | **R-06.4** ratchet, **R-06.5** tighten tolerances | locks in the win; R-06.4 disambiguates R-05-E | small |
| 3 | **R-05 E** confirm 4→6 is funnel, not regression | only possible backslide | small |
| 4 | **R-04** untruncate, then the 9 by hand | one line, then care | small each |
| 5 | **R-02** base-kernel resolution | largest category; unlocks >30 | medium |
| 6 | **R-03 tier 1** the `_cpy_*` setters | ~17 functionals from one whitelist | medium |
| 7 | **R-05 A, F**; **R-06.6** oracle test | completeness | small |
| 8 | **R-03 tier 2** one-off setters | on demand | open-ended |

Steps 1–3 are an afternoon and make the current state durable. Steps 5–6 are
where the remaining coverage comes from.

---

## 8. What is not wrong

* **The kernels** are correct to ~1 ulp against C libxc (§0) — the strongest
  evidence in this document, and it is about the part nobody was worried about.
* **The metadata** is correct where checked: ext-param names, defaults,
  auxiliary lists, hybrid terms, propagation indices.
* **The registry** resolves ids and names correctly; ≥600 ids construct.
* **The hybrid-coefficient surface** matches upstream exactly: `b3lyp` → 0.2,
  `pbe0` → 0.25, `cam-b3lyp` → `(0.33, 0.65, 0.19)`.
* **The `UNSUPPORTED` discipline** — refuse rather than guess — is the correct
  design and must survive every fix above. A functional that cannot be resolved
  safely belongs in that list with a specific reason, never in the wired set with
  approximate constants.

---

## 9. Downstream

`pyscf_rs` is ready to switch its default XC backend to libxc and is waiting only
on a decision, not on libxc_rs:

* `crates/pyscf-dft/src/xc_backend.rs` — `XcBackend::default` documents the
  switch criteria; `eval_uks`, `parse`, `family`, `rsh_and_hybrid_coeff` all have
  working libxc arms.
* `crates/pyscf-dft/tests/xc_eval_bitexact.rs::libxc_backend_cannot_evaluate_yet`
  — a deliberate trip-wire that **is now firing**: it asserts a failure that no
  longer occurs. It is `--features libxc`-gated, so the default build is
  unaffected.

Because the two libraries now agree to ~1 ulp, switching should **eliminate**
rather than merely shrink the 4.7e-7 Ha functional-parameterisation gap that
currently separates `pyscf_rs` from a default-configured upstream PySCF.
