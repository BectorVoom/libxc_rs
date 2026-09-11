# libxc_rs remediation — plan v6

**Status:** 2026-09-11
**Follows:** v5. v5 made the eval layer reach what the kernel tree could do; this
plan is about what the public API still **refuses**.
**Scope rule (from the project owner):** a functional **without a public libxc
ID** is unsupported by design and stays that way — it is not expected to be
used. The only such functional is `lda_k_gds08_worker` (libxc number 100001,
declared in `xc_funcs_worker.h`, not `xc_funcs.h`). **Every refusal of a
functional that has a public ID is a bug**, and this plan fixes all of them.
**Context:** written from `pyscf_rs` after making the implemented set
bit-identical to PySCF's libxc (see `libxc-sys/build.rs` for the oracle build).

---

## 0. The headline: 57 of 649 functionals refuse something they claim

`routing.rs::UNSUPPORTED` is not the bug list. About two dozen of its rows are
"composed functional" entries (`hyb_mgga_x_scan0`, `hyb_gga_xc_hflyp`,
`gga_xc_kt1`, …) that the *direct* kernel router cannot reach but
`libxc-eval`'s mix layer composes — they evaluate, and they match libxc. The
bug list is defined by what the **public API refuses**, measured by a sweep:
every registered id × both spins × every derivative order the functional's own
flags claim, through the C ABI, return code checked.

```
                 exc   vxc   fxc   kxc   lxc      (each identical for nspin=1 and nspin=2)
refusals          10    17    18    53    50
init refusals      4 functionals (gds08 family), both spins
```

Those collapse to **57 distinct functionals** and **five mechanisms**:

| item | functionals | refused at | mechanism |
|---|---|---|---|
| V6-02 | 36 composite MGGAs | kxc, lxc | the MGGA mix never grew third/fourth-order buffers |
| V6-03 | 1 (`hyb_mgga_xc_b0kcis`) | fxc, kxc, lxc | its own-kernel half is stubbed at vxc |
| V6-04 | 10 | every order | generator demands 10 kernel modules; libxc ships 8 for these |
| V6-05 | 7 (SCAN-L family) | every order | no deorbitalization composite |
| V6-06 | 4 (gds08 family) | construction | the worker they mix has no public ID |

`b0kcis` is counted in V6-02 as well (it is one of the 36 at kxc/lxc).
**The sweep checks return codes, not values.** Every item below carries its
own oracle gate for the values it unlocks.

---

## V6-01 — Make the refusal sweep a permanent, ratcheting test

Do this first; every other item is measured by it.

`verify/tests/refusal_sweep.rs`: for every id in the public registry, both
spins, every order in `meta.flags` (`HAVE_EXC … HAVE_LXC`), construct and
evaluate on a small physical grid; collect `(name, spin, order, error)`.

- An **allowlist** of the refusals above, each row tagged with the V6 item that
  removes it. The test fails if a refusal appears that is not listed, **and**
  if a listed refusal starts working (so the list can only shrink, and must be
  shrunk in the same commit that fixes it).
- `lda_k_gds08_worker` is not in the public registry, so it never enters the
  sweep — the scope rule needs no special case.
- It goes through the **Rust API**, which is what `pyscf_rs` calls. The C-ABI
  sweep in §3 is the reproduction; the two must agree.

v5's V5-02 proposed a sweep over construct + evaluate at one order. The 19-file
`verify` invocation in AGENTS.md contains none; if one exists elsewhere, extend
it rather than adding a second.

**Gate:** the test reproduces exactly the counts in §0 on today's tree.

---

## V6-02 — Composite MGGAs at third and fourth order (36 functionals)

**Refused with:** `output buffer 'v3rho3' size mismatch: expected 4, got 0`
(16 when polarized), at kxc and lxc.

**Cause.** `crates/libxc-eval/src/eval/mix.rs::evaluate_mixed_mgga_into` sizes
and accumulates MGGA scratch only **through second order** — its length
table ends at `v2tau2`. The only third/fourth-order fields it wires are
`v3rho3`/`v4rho4` for LDA auxiliaries, and a comment in that function still
says *"dispatch_mgga currently rejects Kxc/Lxc orders upstream … when MGGA
Kxc/Lxc dispatch lands, add v3rho3/v4rho4 calls"*. It landed: standalone MGGA
kernels evaluate kxc and lxc in the same sweep. The mix therefore hands each
auxiliary a kxc/lxc output whose fields are empty.

**Fix.** Extend scratch sizing and accumulation to all 20 MGGA kxc fields and
35 lxc fields, plus the GGA and LDA auxiliary subsets that feed them (an LDA
leg writes only the rho-derivatives, a GGA leg the rho/sigma ones). Take every
length from `libxc-core/src/dims` — never hand-written (AGENTS.md: polarized
`v3sigma2lapl` is 12, not the 9 that counting suggests). Delete the stale
comment.

**Functionals:** `hyb_mgga_x_ms2h`, `hyb_mgga_x_mvsh`, `hyb_mgga_x_revscan0`,
`hyb_mgga_x_scan0`, `hyb_mgga_xc_b0kcis`, `hyb_mgga_xc_b86b95`,
`hyb_mgga_xc_b88b95`, `hyb_mgga_xc_bb1k`, `hyb_mgga_xc_br3p86`,
`hyb_mgga_xc_edmggah`, `hyb_mgga_xc_lc_tmlyp`, `hyb_mgga_xc_mpw1b95`,
`hyb_mgga_xc_mpw1kcis`, `hyb_mgga_xc_mpwb1k`, `hyb_mgga_xc_mpwkcis1k`,
`hyb_mgga_xc_pbe1kcis`, `hyb_mgga_xc_pw6b95`, `hyb_mgga_xc_pw86b95`,
`hyb_mgga_xc_pwb6k`, `hyb_mgga_xc_r2scan0`, `hyb_mgga_xc_r2scan50`,
`hyb_mgga_xc_r2scanh`, `hyb_mgga_xc_revtpssh`, `hyb_mgga_xc_tpss0`,
`hyb_mgga_xc_tpss1kcis`, `hyb_mgga_xc_tpssh`, `hyb_mgga_xc_x1b95`,
`hyb_mgga_xc_xb1k`, `mgga_c_revscan_vv10`, `mgga_c_scan_rvv10`,
`mgga_c_scan_vv10`, `mgga_x_mk00b`, `mgga_xc_hle17`, `mgga_xc_otpss_d`,
`mgga_xc_tpsslyp1w`, `mgga_xc_vcml_rvv10`.

**Gate:** sweep clean at kxc/lxc for these 36; `composite_oracle.rs` extended
to kxc and lxc for MGGA mixes. AGENTS.md records that third and fourth
derivatives are **uncovered** today — this item closes that for composites.
Bit-exact against the wheel-built oracle wherever the vendored source matches
libxc 7.0.0.

---

## V6-03 — `hyb_mgga_xc_b0kcis` above vxc

**Refused with:** `functional 563 does not support derivative order Fxc (max: Vxc)`.

**Cause.** b0kcis is the one functional that is its own kernel **plus** its mix
(AGENTS.md, "Known gaps"). `mix.rs::add_own_kernel_mgga` returns
`UnsupportedDerivativeOrder { max: Vxc }` for any order ≥ Fxc, although the
functional's metadata says `max_order: Lxc`. It is a stub, not a property of
the functional.

**Fix.** Add the kernel's fxc…lxc contribution through the workspace scratch,
exactly as the vxc half does — **not** straight into the output (`prepare`
takes the caller's buffers; AGENTS.md explains how that silently discards the
mix).

**Depends on:** V6-02 (the mix half has to reach kxc/lxc first).
**Gate:** b0kcis fxc, kxc, lxc against libxc, both spins; `b0kcis_probe.rs`
extended past vxc.

---

## V6-04 — Kernel trees with fewer than ten modules (10 functionals)

**Refused with:** `not wired to a rayon kernel` at every order.

**Cause.** `tools/translate_rayon/gen_eval.py` emits a dispatch only when all
ten (order, spin) modules exist, and records the rest as
"potential-only functional" or "kernel tree is missing one of the 10 (order,
spin) modules" (`gen_eval.py` ~1774–1778). libxc itself ships fewer for these:

| kernel | modules present | why libxc has fewer | functionals |
|---|---|---|---|
| `gga_x_lb` | vxc…lxc | potential-only (`maple2c/gga_vxc`) | `gga_x_lb`, `gga_x_lbm` |
| `mgga_x_tb09` | vxc…lxc | potential-only (`maple2c/mgga_vxc`) | `mgga_x_tb09`, `mgga_x_bj06`, `mgga_x_rpp09` |
| `lda_xc_tih` | vxc…lxc | potential-only (`maple2c/lda_vxc`) | `lda_xc_tih` |
| `mgga_x_2d_prp10` | vxc…lxc | potential-only (`maple2c/mgga_vxc`) | `mgga_x_2d_prhg07_prp10` |
| `mgga_c_b94` | exc…kxc | maple2c has 8 functions; flags do not claim lxc | `mgga_c_b94` |
| `lda_c_pk09` | exc…kxc | same | `lda_c_pk09` |
| — (mix) | — | `mgga_x_br89` + `mgga_c_b94` | `hyb_mgga_xc_b94_hyb` |

The kernels are already generated; only the dispatch is missing.

**Fix.**
1. Emit dispatch arms for the modules present, **driven by the functional's
   flags**. An order the flags do not claim returns
   `UnsupportedDerivativeOrder` — that is libxc's own contract, not a
   refusal. For a potential-only functional asked for exc+vxc, do what
   libxc's `work_*` does: skip `zk` (its guard is `out->zk != NULL &&
   HAVE_EXC`) and write the potentials.
2. Remove `lda_c_pk09` from `libxc-core/src/deferred.rs`. Its recorded blocker
   was CubeCL's proc-macro stack limit on `lxc_pol`; CubeCL is gone, and
   libxc has no pk09 lxc to translate anyway.
3. `hyb_mgga_xc_b94_hyb` then constructs; its kxc joins V6-02's gate.
4. Before claiming LB94, confirm the maple2c body is the whole potential:
   `gga_x_lb.c` carries hand-written code around it (the modified asymptotic
   of `gga_x_lbm`). Same care for TB09's `c`, which libxc users normally set
   at runtime — confirm the ext_params wiring reaches it.

**Gate:** sweep clean for all ten at every claimed order; `kernel_oracle.rs`
extended to potential-only kernels (it compares `zk` first today), both
spins.

---

## V6-05 — The deorbitalized SCAN-L family (7 functionals)

**Refused with:** `not wired to a rayon kernel` at every order.

**Cause.** libxc builds these with `xc_deorbitalize_init(p, base, ked)` and
evaluates them with `xc_deorbitalize_func` (`deorbitalize_func.c`, 470
lines): it evaluates the kinetic-energy functional to get `tau(rho, sigma,
lapl)`, feeds that `tau` to the base MGGA, and chain-rules every derivative
through it, up to lxc. libxc_rs has no deorbitalization composite, and the
metadata snapshot does not record the relationship (it is an init call, not
an `xc_mix_init`).

| functional | id | base MGGA | kinetic functional |
|---|---|---|---|
| `mgga_x_scanl` | 700 | `mgga_x_scan` | `mgga_k_pc07_opt` |
| `mgga_x_revscanl` | 701 | `mgga_x_revscan` | `mgga_k_pc07_opt` |
| `mgga_c_scanl` | 702 | `mgga_c_scan` | `mgga_k_pc07_opt` |
| `mgga_x_r2scanl` | 718 | `mgga_x_r2scan` | `mgga_k_pc07_opt` |
| `mgga_c_r2scanl` | 719 | `mgga_c_r2scan` | `mgga_k_pc07_opt` |
| `mgga_c_scanl_rvv10`, `mgga_c_scanl_vv10` | 703, 704 | `xc_mix_init` over `mgga_c_scanl` | — |

**No new kernels are needed.** `mgga_k_pc07_opt` resolves to the `mgga_k_pc07`
kernel and `mgga_x_revscan` to `mgga_x_scan`; all four bases have all ten
modules.

**Fix.**
1. Metadata: a `Deorbitalized { base, ked }` composition for ids 700, 701, 702,
   718, 719, **read out of libxc's own `xc_func_type`** the way
   `gen_aux_overrides.rs` reads the override table — not scraped from C.
2. Port `xc_deorbitalize_func` operand for operand, one order at a time
   (exc, vxc first; then fxc…lxc), polarized and unpolarized.
3. 703 and 704 then evaluate through the existing mix layer.

**Gate:** a new `deorbital_oracle.rs` against libxc per order and spin; bit-exact
where the vendored source matches 7.0.0.

---

## V6-06 — The gds08 family, through an internal worker (4 functionals)

**Refused with:** `functional 591 is not yet supported by libxc_rs: mixes an
internal libxc worker functional (lda_k_gds08_worker) …` at construction
(since 2026-09-11; before that they evaluated with a component missing and
returned values up to 7× off).

**Cause.** `gga_k_gds08`, `gga_k_ghds10`, `gga_k_ghds10r` and `gga_k_tkvln`
are `xc_mix_init` over `gga_k_vw` or `gga_k_tfvw` **plus**
`lda_k_gds08_worker` (id 100001, beyond the `u16` `FunctionalId`). The
parents have public IDs, so by the scope rule they are bugs; the worker does
not, so it stays private.

**Fix.**
1. Translate `maple2c/lda_exc/lda_k_gds08_worker.c` into a kernel crate
   (`from_maple.py` skips it today because it is unregistered). Its setter is
   a plain copy of `_A`, `_B`, `_C` (defaults 0.860, 0.224, 0.0).
2. Reference it from the four parents through an **internal** auxiliary
   handle (a private enum or a name), never the public registry — so it can
   not be constructed or evaluated on its own.
3. The ghds10/ghds10r/tkvln overrides for slot 1 are already in
   `generated_aux_overrides.rs`; gds08 uses the worker defaults.
4. Replace the construction refusal in `functional/lifecycle.rs` with the real
   mix.
5. AGENTS.md says the worker's "kernel exists and is routed by name"; no such
   kernel crate exists today. Correct that line.

**Gate:** the four return to `composite_oracle.rs` (they were `KNOWN_GAPS`
until 2026-09-11) and pass it, both spins.

---

## 1. Suggested order

1. **V6-01** — the gate, so every later item is measured the same way.
2. **V6-04** — generator-only, the kernels already exist, lowest risk;
   unlocks 10 functionals.
3. **V6-02** — one mechanism, 36 functionals, including the hybrids most
   used from PySCF (TPSSh, SCAN0, r2SCAN0/h/50, PW6B95, the B95 family).
4. **V6-03** — needs V6-02's buffers.
5. **V6-05** — new composite kind; the largest port.
6. **V6-06** — kinetic-energy functionals, least used from PySCF.

**Done when:** the V6-01 sweep reports **zero refusals** for every public ID at
every order its flags claim, both spins, with an empty allowlist — and every
(functional, order, spin) newly enabled here passes its oracle against libxc
built the way PySCF's wheel is.

---

## 2. What is not wrong

- **`lda_k_gds08_worker`** — no public ID; unsupported by the scope rule.
- **The "composed functional" rows in `routing.rs::UNSUPPORTED`** — the direct
  router cannot reach them, but `libxc-eval` composes them and they match
  libxc. They are routing notes, not refusals.
- **The 43 functionals that differ from PySCF because the vendored
  `libxc-master` is a pre-7.0.0 snapshot** (HSE06 among them). They evaluate;
  their values follow the snapshot's source. Closing that is a rebase onto the
  7.0.0 release — a separate decision, not part of this plan.
- **Orders a functional does not claim.** Refusing an order the flags leave
  out is libxc's own behaviour.

---

## 3. Reproducing

Build the C ABI, then sweep it (the V6-01 test is the permanent form):

```bash
cargo build --release -p libxc-compat --features c-abi
python3 - .cache/cargo-target/release/liblibxc_compat.so <<'EOF'
import ctypes, collections, sys
lib = ctypes.CDLL(sys.argv[1]); lib.xc_func_alloc.restype = ctypes.c_void_p
lib.xc_func_get_info.restype = ctypes.c_void_p
lib.xc_functional_get_name.restype = ctypes.c_char_p
lib.xc_rs_last_error_message.restype = ctypes.c_char_p
k = lib.xc_number_of_functionals(); ids = (ctypes.c_int * k)(); lib.xc_available_functional_numbers(ids)
N, BIG = 4, 256; buf = lambda v: (ctypes.c_double * BIG)(*[v] * BIG)
inp = {"rho": buf(0.1), "sigma": buf(0.01), "lapl": buf(0.0), "tau": buf(0.05)}
ARGS = {"lda": ["rho"], "gga": ["rho", "sigma"], "mgga": ["rho", "sigma", "lapl", "tau"]}
NOUT = {"lda": [1, 1, 1, 1, 1], "gga": [1, 2, 3, 4, 5], "mgga": [1, 4, 10, 20, 35]}
fail = collections.Counter()
for fid in ids:
    for nspin in (1, 2):
        F = ctypes.c_void_p(lib.xc_func_alloc())
        if lib.xc_func_init(F, fid, nspin) != 0: fail[("init", nspin)] += 1; continue
        info = ctypes.c_void_p(lib.xc_func_get_info(F))
        fam = {1: "lda", 128: "lda", 2: "gga", 32: "gga", 4: "mgga", 64: "mgga"}.get(lib.xc_func_info_get_family(info))
        flags = lib.xc_func_info_get_flags(info)
        for i, order in enumerate(("exc", "vxc", "fxc", "kxc", "lxc")):
            if fam and flags & (1 << i):
                outs = [(ctypes.c_double * BIG)() for _ in range(NOUT[fam][i])]
                if getattr(lib, f"xc_{fam}_{order}")(F, ctypes.c_size_t(N), *[inp[a] for a in ARGS[fam]], *outs):
                    fail[(order, nspin)] += 1
        lib.xc_func_end(F)
print(sorted(fail.items()))
EOF
```

Expected on the 2026-09-11 tree: `exc` 10, `vxc` 17, `fxc` 18, `kxc` 53,
`lxc` 50 per spin, `init` 4 per spin.

---

## 4. Outcome (2026-09-11)

All six items are implemented. `verify/tests/refusal_sweep.rs` started at
exactly the §0 counts (init 4, exc 10, vxc 17, fxc 18, kxc 53, lxc 50 per spin;
57 functionals) and its allowlist is now empty.

| item | what changed | gate |
|---|---|---|
| V6-01 | `verify/tests/refusal_sweep.rs`, ratcheting allowlist, through `BatchEvaluator` | reproduced §0 exactly before any fix |
| V6-02 | `mix.rs`: one `mgga_scratch_output` / `accumulate_mgga` pair covers all five orders with `mix_func.c`'s lapl/tau gates; LDA and GGA auxiliaries accumulate kxc/lxc too | `composite_oracle.rs::composite_mgga_higher_orders_match_libxc`: every composite MGGA, vxc..lxc, both spins, every field -- bit-identical (23,000 to 849,612 nonzero values per order and spin) |
| V6-03 | `add_own_kernel_mgga` goes through the same helpers at every order | same test (b0kcis is one of the 37) |
| V6-04 | `gen_eval.py` emits a partial tree when its modules are exactly the orders its flags claim (`partial_dispatch!`, `prepare_for(.., have_exc)`); pk09 and b94 left `deferred.rs` | `kernel_oracle.rs` 462 kernels (8 new), `kernel_oracle_fxc.rs` 312 (4 new), both spins, 0 over the gate; potential-only kernels compared through `xc_*_vxc` |
| V6-05 | `meta::DEORBITALIZED` observed from libxc by `gen_deorbitalized.rs`; `deorbitalize.py` translates `maple2c/deorbitalize_{1..4}.c`; `eval/deorbitalize.rs` drives it; 703/704 go through the mix | `deorbital_oracle.rs`: all seven, every claimed order, both spins, and at non-default ext_params -- bit-identical |
| V6-06 | `lda_k_gds08_worker` kernel emitted as an internal worker, routed at 34465, built from `meta::internal_auxiliary` only as an auxiliary | `composite_oracle.rs` (the four are back under the gate), `gds08_worker_metadata_matches_libxc` |

**Found on the way: a libxc defect.** `deorbitalize_func.c:188-189` never
zeroes the base meta-GGA's `v4sigmalapltau2`, and `xc_mgga` only zeroes that
field for functionals needing both the Laplacian and tau, so six lxc outputs
of every deorbitalized functional are computed from uninitialized heap memory
in libxc. The oracles exclude exactly those six fields (`LIBXC_UNZEROED`,
documented in AGENTS.md).
