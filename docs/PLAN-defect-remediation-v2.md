# libxc_rs defect remediation — plan v2 (remaining work)

**Status:** draft, 2026-08-28
**Supersedes:** `docs/PLAN-defect-remediation.md` (v1, 2026-08-27) for everything
still open. v1 remains the record of what the defects *were*.
**Context:** written from `pyscf_rs`, which wants `libxc_rs` as its default XC
backend. v1's D-01…D-04 have been fixed; this plan covers what is left.

---

## 0. Where things stand

| v1 defect | status | evidence |
|---|---|---|
| **D-01** facade cannot reach kernels | **fixed** | `libxc-reval` now a dep of both the facade and `libxc-compat` |
| **D-02** stale `oracle-*` in live errors | **fixed** | `grep -rn "oracle-lda" crates/` → empty |
| **D-03** `construct_params` 1-of-649 | **fixed** | `defaults` honoured; `GenericParams::new(id, defs)`; `NoParams::new(id)` carries the id |
| **D-04** 5 failure paths, 1 error value | **fixed** | `PropagationConflictCause` enum with 4 discriminants |
| **D-05** 110 functionals unwired | **partial: 110 → 80** | tier-1 corpus (`lda_x`, `gga_x_pbe`, `gga_c_pbe`, `lda_c_pw`) now wired |
| **§6** no facade-level test | **partial** | `facade_eval.rs`, `propagation.rs`, `ext_params.rs`, `registry_all.rs`, `regression_guards.rs` added — but not reachable by a bare `cargo test`, see R-01 |

**Not yet independently confirmed at the time of writing:** that the library
*evaluates a number* through the public facade. The structural fixes are all
verified; the end-to-end check (Slater exchange vs its closed form, via
`XcBackend::Libxc` in `pyscf_rs`) was still compiling the 156-crate kernel tree.
Everything below assumes it passes; if it does not, that becomes R-00 and
outranks the rest.

**Headline coverage number:** 187 wired kernels against a 649-entry registry.
The gap is not all D-05 — many registry ids are aliases or compounds that
delegate to wired primitives — but it is the number to drive down and the one
worth reporting in the README.

---

## R-01 — A bare `cargo test` skips the entire facade test suite

**Severity: high. This is the process defect that let D-01 ship.**

The root package is not in `default-members`:

```console
$ sed -n '/^default-members/,/\]/p' Cargo.toml
default-members = [
    "crates/libxc-core",
    "crates/libxc-compat",
    "crates/libxc-reval",
    "crates/kernels-rayon/math",
]
```

So `cargo test` at the repo root does not build or run `tests/facade_eval.rs`,
`tests/propagation.rs`, `tests/ext_params.rs` or anything else in the root
`tests/` directory. Confirmed:

```console
$ cargo test --test facade_eval
error: no test target named `facade_eval` in default-run packages
```

`cargo test -p libxc_rs --test facade_eval` is required. A contributor — or a CI
job — that runs the obvious command sees green while the tests that matter most
never execute. That is exactly the condition under which D-01 survived: the
facade was untested *and* untestable by default.

### Fix

1. Add the root package to `default-members`, **or** add an explicit CI job that
   runs `cargo test -p libxc_rs --all-targets`. Prefer the former; the exclusion
   was a rust-analyzer/indexing optimisation (per the comment above
   `default-members`) and should not silently change what `cargo test` means.
2. If the root must stay excluded, put a one-line note in `CONTRIBUTING`/`AGENTS.md`
   and in the `default-members` comment saying which command actually runs the
   facade tests.

**Acceptance:** a bare `cargo test` at the repo root executes `facade_eval.rs`.

---

## R-02 — D-05 category C (30 functionals): base kernels, not a regex miss

**Severity: medium-high — the single largest remaining category.**

v1 guessed this was `_INFO_RE` failing to match. **That guess was wrong**, and
the real cause changes the fix entirely.

These 30 kernel directories have no `xc_func_info_<dirname>` in the C source
because they are **base kernels shared by several concrete functionals**:

```console
$ grep -ohE "xc_func_info_gga_x_hjs\w*" libxc-master/src/*.c | sort -u
xc_func_info_gga_x_hjs_b88
xc_func_info_gga_x_hjs_b88_v2
xc_func_info_gga_x_hjs_b97x
xc_func_info_gga_x_hjs_pbe
xc_func_info_gga_x_hjs_pbe_sol

$ grep -ohE "xc_func_info_gga_x_kt\w*" libxc-master/src/*.c | sort -u
xc_func_info_gga_x_kt1
```

There is no `xc_func_info_gga_x_hjs` and there never will be. Same shape for
`gga_xc_b97` (→ `gga_xc_b97_{3c,d,gga1}`, `hyb_gga_xc_b97*`), `gga_x_vmt`,
`gga_x_s12`, `gga_x_lcgau`, and the rest of the 30.

`extract_params.py` keys its lookup on the **kernel directory name**
(`infos.get(func)` where `func = d.name`), which for a base kernel matches
nothing.

### Fix

Change the extractor's unit of work from *kernel directory* to *concrete
functional*:

1. Build the reverse map first: for every `xc_func_info_<name>` in the vendored
   source, determine which kernel directory implements it. The kernel dir is a
   prefix of the functional name in every case sampled, but that must be
   **verified, not assumed** — resolve it from the `#include "maple2c/..."` line
   in each `.c` file rather than by string prefix, so a mis-prefixed name cannot
   silently bind to the wrong kernel.
2. Emit one param set per concrete functional, all sharing the base kernel.
3. Where a base kernel serves N functionals, that is N registry entries newly
   reachable — so this category is worth more than its count of 30 suggests.

Keep the existing safety rule: any concrete functional that still fails the
literal/name/setter checks stays in `UNSUPPORTED` with its own reason.

**Acceptance:** `gga_x_hjs_pbe` and `gga_xc_b97_d` evaluate and match C libxc
7.0.0 to 1e-14 on a fixed block; `UNSUPPORTED` no longer contains bare base-kernel
names.

---

## R-03 — D-05 category D (33): custom ext-param setters

**Severity: medium. Largest category after C.**

Down from 37. The remainder are genuine: `set_ext_params_cpy_omega`,
`set_ext_params_cpy_cam`, `set_ext_params_cpy_cam_sr`, `set_ext_params_cpy_lc`,
`set_ext_params_cpy_exx`, and a dozen one-off functional-specific setters
(`bn05_set_ext_params`, `case21_set_ext_params`, `csc_set_ext_params`,
`lsrpbe_set_ext_params`, `lspbe_set_ext_params`, `T_set_ext_params`,
`N_set_ext_params`, …).

These are **not** plain copies: they transform values on the way into the params
struct, so v1's instinct to reject rather than guess remains correct.

### Fix

Port them in tiers, highest-value first:

* **Tier 1 — the `_cpy_*` family (≈17).** These are a small, fixed set of
  transforms in `libxc-master/src/*.c`, each a handful of lines. Port each as a
  named Rust function, with the C source quoted in a doc comment, and extend the
  extractor with a whitelist mapping setter name → transform. `_cpy_omega` alone
  covers 14 functionals.
* **Tier 2 — one-off setters (≈16).** Port on demand, driven by what downstream
  actually asks for. Each needs its own reading of the C.

Do **not** add a generic "assume plain copy if it looks like one" fallback. The
whole value of this category's current state is that it is honest.

**Acceptance:** each ported setter has a test comparing against C libxc 7.0.0
for at least one functional that uses it.

---

## R-04 — D-05 category B (9): remaining param-name mismatches

**Severity: low-medium. Small, mechanical, individually inspectable.**

Down from 20 — the `param_a[0]` → `param_a_0` subscript class is fixed. The
9 survivors are a mixed bag and need individual judgement, not another blanket
rule:

| functional | kernel-only | libxc-only | likely |
|---|---|---|---|
| `gga_k_lgap`, `gga_k_lgap_ge` | `param_mu_0` | `param_mu3` | index/name skew — verify which |
| `lda_c_pz` | `param_beta1_0`, `param_beta2_0` | (truncated) | subscript variant |
| `mgga_c_tpss`, `mgga_c_revtpss` | `param_C0_c_0…3` | (truncated) | subscript variant |
| `gga_xc_th1` | `param_omega_0…11` | (truncated) | subscript variant |
| `gga_c_bmk`, `gga_c_sogga11`, `mgga_x_task` | `param_*_ss_*`, `param_*_a_*` | (truncated) | genuine naming divergence |

`param_mu_0` vs `param_mu3` is the one to look at first — those are not the same
index, and if the kernel and libxc disagree about *which* parameter a slot holds,
a naive alias would silently produce wrong numbers. That is the failure mode this
category must not have.

### Fix

Resolve one at a time. For each, print the full kernel and libxc name lists side
by side (the current message truncates at 4, which hides the evidence), confirm
the pairing against the C source, and add an explicit alias entry with a comment
citing the line that justifies it.

**Sub-fix, cheap and worth doing first:** stop truncating. `only_k[:4]` /
`only_l[:4]` in `extract_params.py` throws away exactly the information needed to
diagnose these.

**Acceptance:** each of the 9 either evaluates and matches C libxc to 1e-14, or
carries a reason that is specific to it rather than a truncated list.

---

## R-05 — D-05 categories A, E, F (8 functionals)

**Severity: low. Finish for completeness.**

**A — one left.** `gga_k_dk`: `['0.95*KINX', '14.281111*KINX*KINX', '-19.57962*KINX*KINX*KINX']`.
The macro-expansion pass that fixed the other 17 handles `#define`s and
arithmetic over literals; this needs macro expansion *then* multiplication of the
expanded value by itself. If the const-expression evaluator is already a
whitelisted-AST walker, this is one more node type.

**E — six, and it grew from four.** `gga_x_lb`, `lda_c_pk09`, `lda_xc_tih`,
`mgga_c_b94`, `mgga_x_2d_prp10`, `mgga_x_tb09` — "kernel tree is missing one of
the 10 (order, spin) modules".

The count going **up** (4 → 6) during a round of fixes is the one thing in this
plan I would check before anything else in R-05. The benign explanation is funnel
movement: two functionals that previously failed an earlier check (A or D) now
pass it and reach the module check. The non-benign explanation is that a
regeneration dropped two modules. **Confirm which** by diffing the category-E
membership against the v1 list (v1's four are not individually recorded — recover
them from git history of `routing.rs`).

**F — one.** `mgga_x_tau_hcth`: names/values arrays `pure_names`/`tHCTH_val` not
found in `mgga_x_tau_hcth.c`. Likely the same class as R-02 (the arrays are named
for a variant, not the file) — check before treating it as its own problem.

---

## R-06 — Verify the fixes hold end to end, and keep them held

**Severity: high. This is what turns "structurally fixed" into "works".**

The v1 §6 tests landed, which is the right shape. Three gaps remain:

1. **Make them run** (R-01).
2. **Add a numeric floor.** `facade_eval.rs` should assert against a *value*, not
   just `is_ok()` — Slater exchange has a closed form
   (`f = -(3/4)(3/π)^(1/3) ρ^(4/3)`) and needs no external oracle, so it is the
   ideal always-on assertion.
3. **Guard the tier-1 corpus by name.** Add to `regression_guards.rs`:

   ```rust
   const TIER1: &[&str] = &["lda_x", "lda_c_vwn", "lda_c_pw",
                            "gga_x_pbe", "gga_c_pbe",
                            "gga_x_b88", "gga_c_lyp"];
   // assert none appear in routing::UNSUPPORTED
   ```

   These seven are what LDA, PBE, PBE0, B3LYP and BLYP are built from — i.e. the
   default functional of essentially every downstream consumer. They should never
   silently regress into `UNSUPPORTED` again.

4. **Publish the coverage number.** A test that asserts
   `UNSUPPORTED.len() <= N` with `N` ratcheted downward turns this plan's progress
   into something CI defends. Start at `N = 80`.

---

## 7. Suggested order

| step | item | why here | size |
|---|---|---|---|
| 0 | confirm end-to-end evaluation | everything else assumes it | minutes |
| 1 | **R-01** default-members / CI | cheapest, and prevents recurrence of the whole class | small |
| 2 | **R-06.2–4** numeric floor + tier-1 guard + ratchet | locks in what has been won | small |
| 3 | **R-05 E** confirm 4→6 is funnel, not regression | it is the only possible backslide on the board | small |
| 4 | **R-02** base-kernel resolution | largest category; unlocks more than its 30 | medium |
| 5 | **R-03 tier 1** the `_cpy_*` setters | ~17 functionals from one whitelist | medium |
| 6 | **R-04** the 9 name mismatches (untruncate first) | needs care, not bulk | small each |
| 7 | **R-05 A, F** | completeness | small |
| 8 | **R-03 tier 2** one-off setters | on demand | open-ended |

Steps 0–3 are a day's work and make the current state durable. Steps 4–5 are
where the remaining coverage actually comes from.

---

## 8. What is not wrong (carried forward from v1, re-checked)

* The **kernels** are present and not implicated in any defect found.
* The **metadata** is correct where checked: ext-param names, defaults, auxiliary
  lists, hybrid terms, propagation indices.
* The **registry** resolves ids and names correctly.
* The **hybrid-coefficient surface** (`exx_coefficient`, `cam_coefficients`) was
  verified against upstream PySCF from outside: `b3lyp` → 0.2, `pbe0` → 0.25,
  exact to 1e-12.
* **Build cost is fine** — ~12 min cold, ~18 s incremental.
* The **`UNSUPPORTED` discipline** — refuse rather than guess — is the correct
  design. Every fix above must preserve it. A functional that cannot be resolved
  safely belongs in that list with a specific reason, not in the wired set with
  approximate constants.

---

## 9. Reproducing the evidence in this plan

```bash
cd /home/user/Documents/workspace/libxc_rs

# R-01
cargo test --test facade_eval          # error: no test target
cargo test -p libxc_rs --test facade_eval   # works

# R-02: base kernels have no bare info block, only variants
grep -ohE "xc_func_info_gga_x_hjs\w*" libxc-master/src/*.c | sort -u

# R-02..R-05: the remaining set, by category
python3 - <<'EOF'
import re
from collections import Counter
s = open('crates/libxc-reval/src/routing.rs').read()
body = re.search(r'UNSUPPORTED: &\[\(&str, &str\)\] = &\[(.*?)\n\];', s, re.S).group(1)
e = re.findall(r'\("([^"]+)",\s*"((?:[^"\\]|\\.)*)"\)', body)
def cat(r):
    if r.startswith('non-literal'):        return 'A non-literal default'
    if 'param set mismatch' in r:          return 'B param name mismatch'
    if 'no xc_func_info_' in r:            return 'C base kernel / no info block'
    if 'not a plain copy' in r:            return 'D custom setter'
    if 'kernel tree is missing' in r:      return 'E missing (order,spin)'
    if 'names/values array' in r:          return 'F table not locatable'
    return 'Z ' + r[:40]
print(len(e), 'unsupported')
for k, v in sorted(Counter(cat(r) for _, r in e).items()):
    print(f'  {v:3d}  {k}')
EOF

# R-06.3: tier-1 corpus must stay wired
#   lda_x lda_c_vwn lda_c_pw gga_x_pbe gga_c_pbe gga_x_b88 gga_c_lyp
```

Downstream consumer, ready and waiting on R-02..R-05 for full corpus coverage:
`pyscf_rs/crates/pyscf-dft/src/xc_backend.rs` — `XcBackend::default` documents
the switch criteria; `tests/xc_eval_bitexact.rs::libxc_backend_cannot_evaluate_yet`
fails the moment evaluation starts working, which is the intended signal to flip
the default and drop the `#[ignore]`s.
