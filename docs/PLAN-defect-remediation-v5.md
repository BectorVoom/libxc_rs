# libxc_rs remediation — plan v5

**Status:** 2026-08-28
**Supersedes:** v4. v1–v4 record the path from "cannot evaluate anything" to
"evaluates the wired set, bit-exactly".
**Context:** written from `pyscf_rs`, verifying each round of fixes from outside.

---

## 0. The headline: 481 working kernels, 219 reachable

v4 closed everything it set out to. The extraction pipeline is in good shape:
`UNSUPPORTED` is down to **24**, wired kernels up to **482**, the suite is
**36/36**, the C-libxc parity test lives in the repo, and three rounds of new
wiring have moved **no existing value by a single bit**.

But a sweep over the whole registry — construct every id, evaluate it, tally the
outcome — says this:

```
SWEEP total_ok=219  ctor_err=0  eval_err=400
```

**Only 219 of 619 registered functionals actually evaluate.** `UNSUPPORTED`
lists 24. The list understates the real gap by a factor of ~16.

And the 400 failures are not missing kernels. Sampling five of them:

| functional | kernel file | by-name arm | in `UNSUPPORTED` |
|---|---|---|---|
| `gga_x_pbe_r` | ✅ | ✅ | no |
| `gga_c_pbe_sol` | ✅ | ✅ | no |
| `lda_c_pw_mod` | ✅ | ✅ | no |
| `gga_x_mpw91` | ✅ | ✅ | no |
| `gga_c_pw91` | ✅ | ✅ | no |

Every one has a working kernel and a working dispatch arm, is absent from
`UNSUPPORTED`, and still fails at runtime with *"functional N is not yet
supported by libxc_rs: GGA functional not yet translated"* — a message that is
simply false.

**The work is done; the wiring to reach it is not.** That is V5-01, and it
dwarfs everything else in this plan.

---

## V5-01 — Production dispatch uses the partial enum, not the complete name table

**Severity: critical. ~326 functionals are unreachable despite being finished.**

### The two dispatch tables

`crates/libxc-reval/src/routing.rs` has two parallel routes:

| route | arms | completeness |
|---|---:|---|
| `dispatch_{lda,gga,mgga}` (by enum) | 37 + 94 + 24 = **155** | partial |
| `dispatch_{lda,gga,mgga}_by_name` | 69 + 259 + 153 = **481** | complete |

The enum route is partial *by design* — `LdaFunctional`/`GgaFunctional`/
`MggaFunctional` carry only 38/105/25 variants for a 649-entry registry, and
`dispatch_gga`'s own doc says so: *"56 of the 70 wired GGA functionals have an
enum variant; the rest are reachable only by name."* Every dispatch has an
`other =>` / `_ =>` catch-all that returns `UnsupportedFunctional`.

### Nothing in production calls the complete route

```console
$ grep -rn "dispatch_gga_by_name\|dispatch_lda_by_name" --include=*.rs crates/ \
    | grep -v routing.rs
crates/kernels-rayon/oracle/tests/rayon_oracle.rs:155: …
crates/libxc-reval/src/bin/revalcheck.rs:171: …
```

A test and a diagnostic binary. That is all. Meanwhile the mixing path — which
is how every compound and hybrid functional evaluates — takes the enum route:

```console
$ grep -n "use crate::eval::dispatch" crates/libxc-eval/src/eval/mix.rs
15:use crate::eval::dispatch_lda;
16:use crate::eval::dispatch_gga;
17:use crate::eval::dispatch_mgga;
```

So a functional with no enum variant is unreachable through the facade, no
matter how completely it is wired. This is also why three of v4's remaining 24
fail: `gga_xc_kt1`, `gga_xc_kt2` and `gga_x_ssb` all mix in `gga_x_kt1` (id 145
— `XC_GGA_X_SSB_AUX` is `[LDA_X(-1.0), ssb_sw, kt1]`), and `gga_x_kt1` has a
kernel and a by-name arm but **no enum variant**.

### Failure tally, by the message the user sees

| n | message |
|---:|---|
| 159 | `GGA functional not yet translated…` |
| 138 | `MGGA functional not yet translated…` |
| 49 | (family "not yet …") |
| 16 | `LDA functional not yet translated…` |
| 12 | MGGA deferred / LDA "is tra…" variants |
| **22** | `not wired to a rayon kernel; see routing::UNSUPPORTED` ← the only honest ones |

**378 of 400 failures carry a message that is false**, and 22 are accurate. The
false ones send readers to look for untranslated kernels that were translated
long ago. This is the third instance of the same class this project has hit
(D-03's `FunctionalId(0)`, D-05c's `"setter is NULL"`) — a wrong diagnostic is
worse than a vague one.

### Fix

1. **Route production dispatch through the complete table.** Either make
   `dispatch_{lda,gga,mgga}` fall back to the by-name route when the enum has no
   variant, or have `mix.rs` and the facade call an id-keyed dispatch that covers
   the full registry. An id-keyed table is preferable to name-keyed: it avoids a
   string compare per call in the mixing inner loop.
2. **Generate both tables from one source** so they cannot diverge again. The
   present split is not a design decision anyone would make twice; it is drift.
3. **Fix the catch-all message.** It must distinguish "no kernel exists" from
   "kernel exists but this route cannot see it" — and after (1), the latter
   should be unreachable.

**Acceptance:** the registry sweep reports `total_ok >= 480` (i.e. every
name-wired functional evaluates); `gga_x_kt1`, `gga_xc_kt1`, `gga_xc_kt2` and
`gga_x_ssb` all evaluate; no failure message says "not yet translated" for a
functional that has a kernel file.

---

## V5-02 — Make the sweep a permanent test, and ratchet on *reachability*

**Severity: high. Without this, V5-01 is undetectable from inside the repo.**

`MAX_UNSUPPORTED = 24` guards the **extraction** pipeline: can the generator
resolve this functional's parameters? That is a real property and worth keeping.

It does not guard **reachability**: can a user evaluate this functional? Those
two numbers currently differ by 16×, and only the flattering one is tested. Every
test in the suite either targets a hand-listed corpus (`TIER1_CORPUS`,
`REMEDIATION_V4_CORPUS`, `LDA_CORPUS`) or constructs without evaluating
(`registry_all.rs` asserts ≥600 *constructible* — and construction succeeds for
all 619, which is why it never caught this).

Add the sweep as a test:

```rust
// Reachability ratchet: every registered functional that has a kernel must
// EVALUATE, not merely construct. registry_all.rs checks construction; that
// passed at 619/619 while only 219 could evaluate.
const MIN_EVALUABLE: usize = 219;   // raise as V5-01 lands; target ~482

#[test]
fn test_registry_evaluability_does_not_regress() { /* sweep, count, assert */ }
```

Start at the measured 219 so it is honest today, and raise it with V5-01 —
the number going 219 → ~482 is the acceptance evidence for that item.

**Also fix the ratchet comment.** It currently says the 15 non-E entries are
*"auxiliary mixed functionals (evaluated via mix_func / auxiliary mixing)"*,
which implies they work. Probed: `gga_xc_ncap` and `hyb_gga_xc_hflyp` evaluate;
`gga_xc_kt1`, `gga_xc_kt2`, `gga_x_ssb`, `hyb_mgga_x_ms2h`, `mgga_x_mk00b`,
`hyb_mgga_x_scan0` and `hyb_mgga_xc_lc_tmlyp` do not. True for some, not all.

---

## V5-03 — The 24 in `UNSUPPORTED`

Unchanged from v4 and correctly deferred; these are genuine extraction gaps.
Note V5-01 may resolve several of the aux-mix entries as a side effect, so **do
V5-01 first and re-measure** before spending effort here.

**9 — missing (order, spin) module.** `gga_x_lb`, `gga_x_lbm`, `lda_c_pk09`,
`lda_xc_tih`, `mgga_c_b94`, `mgga_x_2d_prhg07_prp10`, `mgga_x_bj06`,
`mgga_x_rpp09`, `mgga_x_tb09`.

Several — `bj06`, `rpp09`, `tb09`, `lb`, `lbm` — are **potential-only**
functionals: Becke-Johnson and van Leeuwen-Baerends define `vxc` with no
corresponding `exc`. For those, a missing energy module is **correct**, and they
should be recorded as *"potential-only functional; no exc by construction"*
rather than sitting in a list that reads like a gap. That likely converts 5 of
the 9 from "to fix" to "to document", which is the honest outcome and stops this
category growing mysteriously (it has now grown twice: 4 → 6 → 9, both times
benignly).

**8 — auxiliary mix** (`gga_xc_kt{1,2,3}`, `gga_xc_ncap`, `hyb_gga_xc_cap0`,
`hyb_gga_xc_hflyp`, `hyb_mgga_x_ms2h`, `mgga_x_mk00b`). Defaults live in `_init`
but the functional is composed via `xc_mix_init`, so its "parameters" are mixing
coefficients. Re-measure after V5-01.

**7 — one-off setters.** `ssb_set_ext_params` (3), `scan0_set_ext_params` (2),
`hyb_mgga_xc_b94_hyb_set_ext_params`, `lc_tmlyp_set_ext_params`. Port as in
v4-02: a named Rust function per setter with the C source quoted, plus a
whitelist entry. No generic fallback.

---

## V5-04 — Two smaller items

**Rename the invariant tests.** `oracle_lda.rs` / `oracle_gga.rs` /
`oracle_mgga.rs` / `oracle_hybrid.rs` test *invariants* (finite, non-NaN, sign),
not oracle values. Now that `oracle_c_libxc_parity.rs` exists and is a real
oracle, the naming actively misleads. `invariants_*.rs` would be accurate.

**Widen the parity corpus.** `oracle_c_libxc_parity.rs` covers `TIER1_CORPUS` (9)
and `REMEDIATION_V4_CORPUS`. Once V5-01 lands and ~482 functionals are reachable,
extend it to sweep every reachable functional against C libxc at 1e-14. That is
the test that would let the README claim "bit-exact" about the *library* rather
than about a sampled corpus.

---

## 1. Suggested order

| step | item | why here | size |
|---|---|---|---|
| 1 | **V5-02** reachability sweep test at `MIN_EVALUABLE = 219` | makes V5-01 measurable before you start it | small |
| 2 | **V5-01** route dispatch through the complete table | 219 → ~482; the entire remaining value | medium |
| 3 | raise `MIN_EVALUABLE`; re-measure `UNSUPPORTED` | V5-01's acceptance evidence | trivial |
| 4 | **V5-03** potential-only reclassification (5 of the 9) | converts "gap" to "documented" | small |
| 5 | **V5-04** rename invariant tests; widen parity corpus | naming honesty; then the real bit-exactness claim | small |
| 6 | **V5-03** remaining setters + aux mixes | genuine per-item work, much reduced after step 2 | medium |

Steps 1–3 are the plan. Everything else is tidy-up.

---

## 2. What is not wrong

Re-verified this round:

* **The kernels are correct and stable.** Three rounds of wiring (187 → 423 →
  482) changed **no existing value by a single bit** — `exc`, `vrho` and
  `vsigma` for slater/lda,vwn/pbe/blyp/b3lyp/pbe0 are byte-identical across all
  three measurements, and still ≤2.14e-16 from C libxc.
* **Construction is solid.** `ctor_err = 0` across the whole registry, both
  spins. D-03/D-04 are thoroughly closed.
* **The extraction pipeline is in good shape** — 110 → 24, with accurate reasons.
  v4's message fix (`"setter is NULL"` → `"defaults are in X_init, but functional
  is an auxiliary mix"`) is exactly the right kind of change.
* **The C-libxc parity test is real** and passes: `test_tier1_corpus_parity…`
  and `test_remediation_v4_corpus_parity…` both run against `libxc_rs_verify`.
* **`UNSUPPORTED` discipline** remains correct and must survive V5-01: a
  functional that cannot be resolved safely belongs in that list with a specific
  reason.

The distinction worth holding onto: **v1–v4 fixed whether the library is
correct. V5-01 is about whether users can get at it.** The hard part is done.

---

## 3. Reproducing

```bash
cd /home/user/Documents/workspace/libxc_rs

# the two dispatch tables
python3 - <<'EOF'
import re
s = open('crates/libxc-reval/src/routing.rs').read()
for p in re.split(r'\n(?=pub fn )', s):
    h = p.split('\n', 1)[0]
    if not h.startswith('pub fn dispatch'): continue
    print(f"  {h.split('(')[0].replace('pub fn ',''):26} "
          f"enum={len(re.findall(r'Functional::\w+\s*=>', p)):4} "
          f"name={len(re.findall(chr(34)+r'\w+'+chr(34)+r'\s*=>', p)):4}")
EOF

# nothing in production calls the complete route
grep -rn "dispatch_gga_by_name" --include=*.rs crates/ | grep -v routing.rs

# mixing takes the enum route
grep -n "use crate::eval::dispatch" crates/libxc-eval/src/eval/mix.rs

# the sweep (see V5-02 for the permanent version)
#   construct + evaluate every registry id -> total_ok=219, eval_err=400
```

---

## 4. Downstream

`pyscf_rs` needs only the tier-1 corpus, which works today — it is not blocked by
V5-01 and is waiting on a decision only:

* `crates/pyscf-dft/src/xc_backend.rs` — `XcBackend::default` documents the
  switch criteria; the libxc arms are written and verified.
* `crates/pyscf-dft/tests/xc_eval_bitexact.rs::libxc_backend_cannot_evaluate_yet`
  — a deliberate trip-wire that **is now firing**, asserting a failure that no
  longer occurs.

Because the two libraries agree to ~1 ulp, switching should **eliminate** rather
than shrink the 4.7e-7 Ha functional-parameterisation gap that separates
`pyscf_rs` from a default-configured upstream PySCF.
