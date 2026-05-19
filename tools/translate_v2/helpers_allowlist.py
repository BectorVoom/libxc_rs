#!/usr/bin/env python3
"""Hardcoded D-09 generic-helper allowlist for the chunk-body emit path.

Phase 11.1 D-08 / D-09 / D-19 single source of truth — the translator's
``per_functional._wrap_f64_literals_v2`` consults this module to decide

  1. Which math/ public functions take a ``::<F>`` turbofish at call sites
     inside generic chunk bodies (every entry in :data:`GENERIC_HELPERS`).
  2. Which math/ modules are NOT generic over F and must be left untouched
     at call sites (every entry in :data:`NON_GENERIC_MODULES`).
  3. The hoisted-binding name map for f64 named-constants that appear in
     chunk bodies (:data:`NAMED_CONSTS`).

This file is HAND-CURATED, not auto-derived from ``crates/kernels/math/src/``.
Refresh policy (D-09):

    If ``crates/kernels/math/src/`` ships a NEW generic helper after
    Phase 11.1 lands, this file must be edited manually to add the new
    function name to the right module's set. Out of scope for the
    translator to discover this drift automatically — Plan 02's G1 / G2
    compile sweep is the empirical drift detector (a missing entry shows
    up as an E0277 / "expected F, found f64" error in the failing
    subcrate).

Build env source of truth: ``.cargo/config.toml`` (do not duplicate here).
"""

from collections import OrderedDict


# --- D-09 GENERIC_HELPERS: 14 modules x ~40 fns total ---------------------
# Every entry is a grep-confirmed `pub fn NAME<F: Float>` in the on-disk
# crates/kernels/math/src/<module>.rs file as of 2026-05-19. To regenerate,
# run:
#     for f in $(ls crates/kernels/math/src/*.rs); do
#         grep -nE '^pub fn ([a-z_][a-z_0-9]*)<F: Float>' "$f"
#     done
GENERIC_HELPERS: dict = {
    # --- Phase-1 (5 modules) — generic since commits 466e074d0, d8cc4da0c ---
    "powers": {
        "safe_cbrt",
        "pow_1_3", "pow_2_3", "pow_4_3", "pow_5_3", "pow_3_2",
        "pow_1_4", "pow_7_3", "pow_2", "pow_3",
    },
    "piecewise": {
        "piecewise3", "piecewise5",
    },
    "lambert_w": {
        "lambert_w",
    },
    "polynomials": {
        "poly_eval", "rational_eval",
    },
    "spin": {
        "compute_total", "compute_zeta", "to_total_zeta_total",
        "spin_scaling", "clamp_zeta",
    },
    # --- Phase-2 (9 modules) — generic since 11-06 5th-iter Session 2 ---
    "bspline": {
        "case21_xbspline", "case21_cbspline",
    },
    "dft_quantities": {
        "wigner_seitz_rs", "reduced_gradient_s", "tf_kinetic",
        "dimensionless_alpha",
    },
    "erf": {
        "erf_approx", "erf_cube", "erfc_approx",
    },
    "special": {
        "xc_dilogarithm", "xc_erfcx",
    },
    "mbrxc": {
        "xc_mgga_x_mbrxc_get_x",
    },
    "br89": {
        "xc_mgga_x_br89_get_x",
    },
    "integrate": {
        "xc_integrate_func0", "xc_integrate_func1",
        # lda_x_1d_soft integrands (added 2026-05-19, Phase 11.1 translator fix)
        "xc_integrate_lda_soft_func1", "xc_integrate_lda_soft_func2",
        # lda_x_1d_exponential integrands (added 2026-05-19 follow-up)
        "xc_integrate_lda_exponential_func1", "xc_integrate_lda_exponential_func2",
    },
    "expint_e1": {
        "xc_e1_scaled",
    },
    "bessel": {
        "xc_bessel_I0_scaled", "xc_bessel_I0",
        "xc_bessel_I1_scaled", "xc_bessel_I1",
        # Modified Bessel of the 2nd kind (added 2026-05-19, Phase 11.1 translator fix)
        "xc_bessel_K0_scaled", "xc_bessel_K0",
        "xc_bessel_K1_scaled", "xc_bessel_K1",
    },
}


# --- D-09 NON_GENERIC_MODULES: modules whose call sites never get turbofish ---
# deferred: `pub fn is_deferred(id: u16) -> bool` — concrete-typed.
# constants: data-only (no callables).
# lib: module-decl-only (no callables).
NON_GENERIC_MODULES: frozenset = frozenset({"deferred", "constants", "lib"})


# --- D-19 NAMED_CONSTS: hoisted-binding name map ---
# OrderedDict so the emitted prelude is in a stable, declaration-aligned order
# (M_PI first, f64::MAX last). Existing chunk-body locals are `t1..t40620`, so
# any short lowercase non-`t<digit>` name is collision-free.
NAMED_CONSTS: "OrderedDict[str, str]" = OrderedDict([
    # constants.rs public symbols (chunk bodies reference these via
    # `use libxc_kernel_math::constants::*;`)
    ("M_PI",                "pi"),
    ("M_SQRTPI",            "sqrt_pi"),
    ("M_CBRTPI",            "cbrt_pi"),
    ("M_SQRT3",             "sqrt3"),
    ("M_CBRT2",             "cbrt2"),
    ("M_CBRT3",             "cbrt3"),
    ("M_CBRT4",             "cbrt4"),
    ("M_CBRT5",             "cbrt5"),
    ("M_CBRT6",             "cbrt6"),
    ("M_CBRT7",             "cbrt7"),
    ("M_CBRT9",             "cbrt9"),
    ("M_SQRT2",             "sqrt2"),
    ("M_C",                 "c_const"),  # `c` collides with chunk locals
    ("KF_CONST",            "kf_const"),
    ("RS_CONST",            "rs_const"),
    # Helper-module-private but Maple-translated chunk bodies sometimes
    # reference them via fully-qualified paths or via `use` re-exports;
    # listed here so the translator can hoist consistently regardless of
    # the call-site form.
    ("ERX",                 "erx"),
    ("SQRT_DBL_EPSILON",    "sqrt_eps"),
    ("LOG_DBL_MAX",         "log_dbl_max"),
    ("TWO_DBL_MIN",         "two_dbl_min"),
    ("PI_TWO_THIRDS",       "pi_two_thirds"),
    ("POW_32PI_TWO_THIRDS", "pow_32pi_two_thirds"),
    # f64::MAX is a stdlib associated const, not a math/-defined symbol —
    # included so any Maple-translated saturation expression can be hoisted
    # uniformly.
    ("f64::MAX",            "f64_max"),
])


def is_generic_helper_call(name: str) -> bool:
    """Return True iff ``name`` is a known generic-over-F helper.

    Used by ``translate_v2/per_functional.py`` at chunk-body emit time
    to decide whether to insert ``::<F>`` turbofish at a call site.

    O(M*K) — M=14 modules, K=average fns per module (~3). Acceptable for
    the per-line call-site rate; if it ever becomes hot, flatten into a
    single ``frozenset`` at module load.
    """
    for fns in GENERIC_HELPERS.values():
        if name in fns:
            return True
    return False
