# libxc_rs defect remediation plan

**Status:** draft, 2026-08-27
**Author's context:** written from the outside, while attempting to make
`libxc_rs` the default XC backend of `pyscf_rs` (`crates/pyscf-dft`). That
attempt failed, and the failures below are what it hit. Every claim here is
backed by a command whose output is quoted; nothing is inferred from naming.

---

## 0. The one-paragraph summary

**`libxc_rs` cannot currently evaluate any functional through its public API.**
Not "some functionals" — none. Three independent defects stack:

1. the facade does not depend on the crate that holds the kernels (§1),
2. the layer it *does* depend on has its dispatch permanently stubbed (§2),
3. and the per-functional parameter plumbing is implemented for exactly 1 of
   649 functionals (§3).

Below those sit a translator that leaves 110 functionals unwired for six
mechanical reasons (§5), and the corpus that matters most — `lda_x`,
`gga_x_pbe`, `gga_c_pbe` — is inside that 110.

The good news is that none of this is deep. §1-§3 are wiring and one `match`
arm; §5 is string handling in a Python generator. The kernels themselves
(`crates/kernels-rayon`, 266 crates) are generated, present, and not implicated
in any defect found here.

---

## 1. D-01 — The facade cannot reach the kernels

**Severity: critical. This alone makes the library non-functional.**

`crates/libxc-reval` is where numerical dispatch lives (`AGENTS.md`: *"rayon
eval layer: stride-aware parallel sweep, per-family dispatch, routing"*). The
published facade does not depend on it:

```console
$ sed -n '/^\[dependencies\]/,/^\[/p' Cargo.toml
libxc-core   = { path = "crates/libxc-core" }
libxc-eval   = { path = "crates/libxc-eval", default-features = false }
libxc-compat = { path = "crates/libxc-compat", default-features = false }
```

Nor does the C-ABI shim:

```console
$ sed -n '/^\[dependencies\]/,/^\[/p' crates/libxc-compat/Cargo.toml
libxc-core = { path = "../libxc-core" }
libxc-eval = { path = "../libxc-eval", default-features = false }
```

The only consumers of `libxc-reval` are the workspace member list and two
excluded tools:

```console
$ grep -rln "libxc-reval" --include=Cargo.toml .
Cargo.toml
crates/kernels-rayon/oracle/Cargo.toml
crates/libxc-reval/Cargo.toml
bench-vs-libxc/Cargo.toml
```

So `bench-vs-libxc` measures a path that no library user can take, and the
benchmark numbers in `docs/perf` describe code that ships to nobody.

### Fix

Add `libxc-reval` to the facade and to `libxc-compat`, and route
`api::BatchEvaluator` through `libxc_reval::routing::dispatch_{lda,gga,mgga}`
instead of `libxc_eval::eval::dispatch_*`.

The obvious objection is build cost — `libxc-reval` pulls 156 kernel crates.
Measured, that is **~12 minutes cold** and **~18 seconds incremental**, not the
multi-hour figure that seems to have driven the current split. If the cost is
still unwanted by default, make it a default-ON feature (`kernels`) rather than
an absent dependency, so that turning it off is a deliberate choice and the
default build works.

**Acceptance:** `Functional::new(FunctionalId(1), Unpolarized)` followed by a
`BatchEvaluator::evaluate` on an LDA block returns Slater exchange to the
analytic closed form, through the public facade, with default features.

---

## 2. D-02 — `libxc-eval`'s dispatch is stubbed, and says so with a stale message

**Severity: critical (with D-01). Diagnostic quality: bad.**

`crates/libxc-eval/src/eval/mod.rs` opens:

> *"The per-family CubeCL dispatch modules were deleted with the archived kernel
> tree. What is left are the stubs that already stood in for a family whose
> kernels were not compiled: they return `UnsupportedFunctional`, which is now
> simply always the answer here."*

All three `dispatch_lda` / `dispatch_gga` / `dispatch_mgga` unconditionally
return `UnsupportedFunctional`. Their `reason` strings tell the caller to enable
features that **no longer exist**:

```rust
reason: "LDA family not compiled in this build (enable feature `oracle-lda`)",
```

```console
$ sed -n '/\[features\]/,/^\[/p' Cargo.toml
# The `oracle-{lda,gga,mgga}` and `cubecl-backend` features are gone with the
# archived CubeCL tree they selected.
default     = []
```

A downstream integrator following that message spends real time looking for a
feature that was deleted. It cost me an afternoon.

### Fix

Delete the three stubs along with `libxc_eval::eval`'s dispatch re-exports once
D-01 routes to `libxc-reval`. If they must survive as a transitional shim,
change `reason` to name the actual cause and the actual remedy.

**Acceptance:** `grep -rn "oracle-lda" crates/` returns nothing.

---

## 3. D-03 — `construct_params` is implemented for 1 of 649 functionals

**Severity: critical. This is why every range-separated hybrid fails to
construct.**

`crates/libxc-eval/src/functional/lifecycle.rs:171`:

```rust
pub(crate) fn construct_params(
    id: FunctionalId,
    _defaults: Option<&[f64]>,
) -> Result<Box<dyn FunctionalParams>, LibxcRsError> {
    match id.raw() {
        1 => { let p = params_lda::LdaXParams::from_defaults(); Ok(Box::new(p)) }
        _ => {
            // All other functionals: NoParams. Dispatch arms ignore the
            // trait-object params and use hardcoded libxc defaults.
            Ok(Box::new(NoParams))
        }
    }
}
```

Note `_defaults` is ignored, so even id 1 does not receive the caller's values.

The consequence is not merely "params are defaults". `NoParams` reports a
capacity of zero, so **any** attempt to set an external parameter fails:

```console
$ # Functional 529 = XC_GGA_X_ITYH, whose metadata declares exactly one param
529 ext_params: ["_omega"]
529 set_ext_param(_omega) -> Err(ExtParamCountMismatch { id: FunctionalId(0), expected: 0, actual: 1 })
```

Two bugs visible in one line: the count mismatch, and `id: FunctionalId(0)` —
the error carries the wrong functional id, which makes it untraceable in a log.

### Fix

1. Generate a `FunctionalParams` implementation per functional from the same
   metadata that already produces `meta.ext_params`, or replace the trait object
   with a plain `Box<[f64]>` carried on `Functional` and read by the dispatch
   arms. The latter is smaller and matches what the kernels actually take
   (`param_*: f64` arguments).
2. Honour `_defaults`.
3. Populate the `id` field of `ExtParamCountMismatch` correctly.

**Acceptance:** for every id in the registry, `Functional::new` succeeds and
`set_ext_param(name, v)` succeeds for every name in that functional's
`meta.ext_params`.

---

## 4. D-04 — All 9 propagation rules fail, and the error hides which check failed

**Severity: high. Blocks every CAM/range-separated hybrid, CAM-B3LYP included.**

Every parent in `PROPAGATION_RULES` fails to construct:

```console
 433: PropagationConflict { id: FunctionalId(433), parent_name: "_omega", aux_slot: 1, aux_name: "_omega" }
 470: PropagationConflict { ... }
 395: ...  614: ...  682: ...  490: ...  482: ...  491: ...  478: ...
```

— while every auxiliary they reference constructs fine on its own
(`1, 106, 529, 7, 131, 402, 406` all return `OK`). The data is not at fault:
CAM-B3LYP declares `["_ac", "_alpha", "_beta", "_omega"]` (so
`parent_param_index: 3` is in range), its aux slot 1 is `FunctionalId(529)`, and
529 declares `["_omega"]`. Every precondition the rule states is met.

**The actual cause is D-03**: the final step of `propagate_to_aux` calls
`aux.set_ext_param("_omega", 0.33)`, which fails because 529 was given
`NoParams`. Fixing D-03 should fix this outright.

Independently of that, `propagate_to_aux`
(`crates/libxc-eval/src/functional/lifecycle.rs:109-147`) has **five distinct
failure paths that all produce the identical `PropagationConflict` value**, with
the same four fields. The error cannot distinguish "parent has no ext_params"
from "index out of range" from "aux slot out of range" from "aux rejected the
name" — which is precisely the discrimination a debugger needs, and why locating
this took an instrumented probe rather than a read.

The doc comment also asserts something now false:

> *"Real `PROPAGATION_RULES` (xtask-validated) never trigger these branches; they
> exist as defense-in-depth against snapshot drift."*

All nine trigger them.

### Fix

1. Fix D-03; re-test all nine parents.
2. Give each failure path a distinct error (or add a `cause` discriminant to
   `PropagationConflict`). Defense-in-depth that cannot say what it caught is
   not defense-in-depth.
3. Correct the stale doc comment, and add a test that constructs every
   `PROPAGATION_RULES` parent — this class of breakage should never again be
   discovered downstream.

**Acceptance:** a test iterating `PROPAGATION_RULES` constructs every parent and
asserts the aux received the propagated value.

---

## 5. D-05 — 110 functionals unwired, in six mechanical categories

**Severity: high for adoption. `lda_x`, `gga_x_pbe` and `gga_c_pbe` are in
here, which rules out PBE, LDA/VWN and every PySCF default.**

`crates/libxc-reval/src/routing.rs::UNSUPPORTED` lists 110 entries. Grouped by
stated reason:

| n | category | representative |
|---:|---|---|
| 37 | custom ext-param setter (not `set_ext_params_cpy`) | `lda_x` — "setter is NULL, not a plain copy" |
| 30 | no `xc_func_info_` block found in libxc source | — |
| 20 | param **name** mismatch, kernel vs libxc | `gga_c_pbe` — kernel `param_BB` vs libxc `param_B` |
| 18 | non-literal default value | `gga_x_pbe` — `['MU_PBE']` |
| 4 | kernel tree missing an (order, spin) module | — |
| 1 | names/values array not locatable in the C source | `mgga_x_tau_hcth` |

All six are produced by one file, `tools/translate_rayon/extract_params.py`
(lines 133-166). Its safety rule — *"a functional is emitted only if all hold,
otherwise it is reported as unresolved rather than guessed at"* — is the right
instinct. The problem is that four of the six rejections are string-handling
limitations, not genuine ambiguity.

### D-05a — non-literal defaults (18 functionals)

`_FLOAT_RE` accepts only a bare float literal, so a C macro or an expression is
rejected. But the macros are vendored and trivially resolvable:

```console
$ grep -n "define MU_PBE" libxc-master/src/util.h
215:#define MU_PBE 0.2195149727645171 /* mu = beta*pi^2/3, beta = 0.06672455060314922 */
```

The 18 failures are of three shapes: a `#define` (`MU_PBE`), a `long double`
suffix (`2.804L`, `1.1015L`), and simple arithmetic (`1.745*0.11`,
`0.35/2.29`).

**Fix:** a resolution pass before the literal check — expand `#define`s from
`util.h` plus the functional's own translation unit, strip `L`/`l`/`f` suffixes,
and evaluate `+ - * /` over literals with Python's `ast.literal_eval` on a
whitelisted grammar (never `eval`). Rejecting anything still unresolved keeps
the safety property intact.

### D-05b — param name mismatch (20 functionals)

libxc names array elements with C subscripts; the kernels flatten them:

```console
$ grep -n "pw_names" libxc-master/src/lda_c_pw.c
37:static const char *pw_names[PW_N_PAR] = {"_pp[0]", "_pp[1]", ..., "_a[0]", ...};
```
```
lda_c_pw  UNSUPPORTED: param set mismatch;
          kernel-only=['param_a_0', ...] libxc-only=['param_a[0]', ...]
```

`param_a[0]` and `param_a_0` are the same parameter.

**Fix:** normalise `[i]` → `_i` when building `mapping`. Also check whether
`param_BB` vs `param_B` (`gga_c_pbe`) is the same class or a genuine
double-letter kernel name; if the latter, it needs a small alias table with a
comment per entry.

### D-05c — "setter is NULL" (subset of the 37)

`lda_x` is the important case and it is **not** a real parameter problem:

```c
const xc_func_info_type xc_func_info_lda_x = {
  XC_LDA_X, XC_EXCHANGE, "Slater exchange", XC_FAMILY_LDA, ...
  {0, NULL, NULL, NULL, NULL},          /* zero external parameters */
  lda_x_init, NULL, &work_lda, NULL, NULL
};

static void lda_x_init(xc_func_type *p) { ...; params->alpha = 1.0; }
```

libxc declares **zero** ext params; the default lives in the `_init` function.
The extractor reaches the setter check only because the *kernel* takes
`param_alpha: f64` (maple2c emits the X-alpha generalisation).

**Fix:** when the info block declares `npar == 0`, do not consult the setter at
all — resolve the functional and take defaults from the `_init` body (a narrow
`params->NAME = LITERAL;` scrape), or from a small hand-maintained table with a
citation per entry. The remaining "custom setter" cases are genuinely different
(`set_ext_params_cpy_omega` and friends transform on the way in) and should stay
rejected until each transform is ported deliberately.

### D-05d — no `xc_func_info_` block (30 functionals)

Investigate before fixing: `_INFO_RE` requires the block to end with `\n};`, so
any formatting variation drops it silently. Confirm whether these 30 are a regex
miss or genuinely absent from the vendored tree.

### Priority within D-05

Fix in the order that unblocks real users: **`lda_x` → `gga_x_pbe` →
`gga_c_pbe` → `lda_c_pw`**. Those four make LDA, PBE, PBE0 and B3LYP work, which
is the entire default corpus of PySCF, Psi4 and most downstream consumers.
`gga_x_b88`, `gga_c_lyp` and `lda_c_vwn` are already wired, so BLYP should work
today once D-01..D-03 are fixed — that makes BLYP the cheapest end-to-end
acceptance test.

**Acceptance:** `UNSUPPORTED` no longer contains `lda_x`, `gga_x_pbe`,
`gga_c_pbe`, `lda_c_pw`; each newly wired functional matches C libxc 7.0.0 to
1e-14 on a fixed density block.

---

## 6. Cross-cutting: the test suite does not cover the public path

Every defect above is reachable from a three-line program against the public
API, yet the suite is green. That is the finding behind the findings.

`docs/rust_crate_test_guideline.md` should grow one rule: **at least one test
must exercise the shipped facade end to end, with default features, and assert a
number.** A single `assert!((slater(rho) - facade_eval(rho)).abs() < 1e-14)`
would have caught D-01, D-02, D-03 and D-05c simultaneously, years earlier than
a downstream integrator did.

Suggested additions:

| test | catches |
|---|---|
| facade LDA/GGA eval vs analytic Slater + a C-libxc reference block | D-01, D-02, D-05 |
| construct every id in the registry | D-03 |
| construct every `PROPAGATION_RULES` parent, assert propagated value | D-04 |
| `set_ext_param` round-trip for every declared param of every functional | D-03 |
| assert `UNSUPPORTED` does not contain a named "tier-1 corpus" list | D-05 regressions |

---

## 7. Suggested order of work

| step | defect | why first | rough size |
|---|---|---|---|
| 1 | D-01 | nothing else is observable until the kernels are reachable | wiring + feature |
| 2 | D-02 | falls out of 1; delete the stubs | small |
| 3 | D-03 | unblocks D-04 and all parameterised functionals | medium |
| 4 | D-04 | verify; then split the error | small after 3 |
| 5 | D-05b, D-05c | the four functionals that unblock the real corpus | medium |
| 6 | D-05a | 18 more functionals, same generator | small |
| 7 | D-05d | needs investigation before it can be sized | unknown |
| 8 | §6 | prevents recurrence | small |

Steps 1-4 are the ones that turn `libxc_rs` from "does not evaluate" into
"evaluates the wired set". Step 5 is what makes it usable as a PySCF backend.

---

## 8. What is NOT wrong

Recorded so the plan is not read as a general indictment:

* The **kernels** (`crates/kernels-rayon`, 266 crates) are present and are not
  implicated in any defect above.
* The **metadata** is right where it was checked: ext-param names, defaults,
  auxiliary lists, hybrid terms and propagation indices for CAM-B3LYP all match
  libxc 7.0.0.
* The **registry** resolves ids and names correctly (`lookup_by_id`,
  `FunctionalId::family`, `by_name`).
* The **hybrid-coefficient surface** (`exx_coefficient`, `cam_coefficients`) is
  correct and was verified against upstream PySCF from the outside: `b3lyp` →
  0.2, `pbe0` → 0.25, exact to 1e-12.
* The **build cost** is fine — ~12 min cold, ~18 s incremental. It is not a
  reason to keep the kernels out of the dependency graph.
* The **`UNSUPPORTED` discipline** — refusing to guess rather than running with
  wrong constants — is the correct design and should be preserved through every
  fix in §5.

---

## 9. Reproducing the evidence

```bash
# D-01: the facade cannot see the kernels
grep -rln "libxc-reval" --include=Cargo.toml .

# D-02: stale feature names in live error strings
grep -rn "oracle-lda" crates/

# D-03 / D-04: construct the propagation parents
#   (from a crate that depends on libxc_rs)
for id in 433 470 395 614 682 490 482 491 478; do
  # Functional::new(FunctionalId(id), Spin::Unpolarized) -> PropagationConflict
done

# D-05: the unsupported set and its categories
python3 - <<'EOF'
import re
s = open('crates/libxc-reval/src/routing.rs').read()
body = re.search(r'UNSUPPORTED: &\[\(&str, &str\)\] = &\[(.*?)\n\];', s, re.S).group(1)
e = re.findall(r'\("([^"]+)",\s*"((?:[^"\\]|\\.)*)"\)', body)
print(len(e), "unsupported")
for n in ('lda_x', 'gga_x_pbe', 'gga_c_pbe', 'lda_c_pw'):
    print(n, dict(e).get(n))
EOF
```

Downstream context, including the working libxc UKS eval and hybrid-coefficient
extraction that are already written and waiting on these fixes:
`pyscf_rs/crates/pyscf-dft/src/xc_backend.rs` (`XcBackend::default` documents
this blocker), `pyscf_rs/crates/pyscf-dft/tests/libxc_hybrid_coeff.rs`, and
`pyscf_rs/crates/pyscf-dft/tests/xc_eval_bitexact.rs`
(`libxc_backend_cannot_evaluate_yet` fails the day D-01..D-03 land).
