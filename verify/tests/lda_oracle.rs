//! Per-functional Rust-vs-C oracle comparison for all compiled LDA functionals.
//!
//! Drives `libxc_rs::eval::dispatch_lda` for every functional in `LDA_FUNCTIONALS`
//! and compares against the libxc C oracle (`oracle_lda_all`) at the
//! tolerance tiers locked in 04-CONTEXT D-10-R:
//!   - exc (zk):    rel_err <= 1e-12
//!   - vxc (vrho):  rel_err <= 1e-10
//!   - fxc (v2rho2): rel_err <= 1e-8
//!   - kxc (v3rho3): rel_err <= 1e-6
//!   - lxc (v4rho4): rel_err <= 1e-4
//!
//! The 4 deferred LDA functionals (lda_c_pk09=554, lda_xc_ksdt=259,
//! lda_c_pw_erf=654, lda_c_pmgb06=590) are skipped via
//! `libxc_kernel_lda::deferred::is_deferred`. Functionals whose registry IDs
//! aren't routable through `LdaFunctional::from_id` are also skipped (e.g.
//! `lda_c_xalpha`, `lda_x_rae`) — these aren't compiled into kernel-lda yet.
//! Skipped counts are surfaced in test output for coverage drift detection.
//!
//! **B1 fix (04-02 plan):** `FunctionalId` is constructed exclusively via
//! the public `FunctionalId::from_raw(id)` constructor — never via tuple
//! syntax `FunctionalId(id)`, which fails to compile from this verify
//! crate due to the inner field's `pub(crate)` visibility.

// 11-12 (G-2): gate so a single-family verify build compiles only its oracle.
#![cfg(feature = "oracle-lda")]

use libxc_rs::eval::{dispatch_lda, LdaFunctionalParams};
use libxc_rs::input::LdaInput;
use libxc_rs::model::{DerivativeOrder, FunctionalId, LdaFunctional, Spin, Thresholds};
use libxc_rs::output::LdaOutput;
use libxc_rs_verify::{
    oracle_func_flags, oracle_lda_all, FLAGS_HAVE_EXC, FLAGS_HAVE_FXC, FLAGS_HAVE_KXC,
    FLAGS_HAVE_LXC, FLAGS_HAVE_VXC, LdaOracleOutput,
};

use libxc_kernel_math::deferred::lda::is_deferred;

struct FunctionalTestCase {
    id: i32,
    name: &'static str,
}

/// All 67 LDA functionals from libxc 7.0.0 (xc_funcs.h).
///
/// Functionals not yet translated into `crates/kernel-lda` are auto-skipped
/// at runtime by `LdaFunctional::from_id`. Deferred functionals are skipped
/// by `is_deferred`. Every entry that maps to a compiled kernel runs the
/// per-derivative comparison.
const LDA_FUNCTIONALS: &[FunctionalTestCase] = &[
    FunctionalTestCase { id: 1, name: "lda_x" },
    FunctionalTestCase { id: 2, name: "lda_c_wigner" },
    FunctionalTestCase { id: 3, name: "lda_c_rpa" },
    FunctionalTestCase { id: 4, name: "lda_c_hl" },
    FunctionalTestCase { id: 5, name: "lda_c_gl" },
    FunctionalTestCase { id: 6, name: "lda_c_xalpha" },
    FunctionalTestCase { id: 7, name: "lda_c_vwn" },
    FunctionalTestCase { id: 8, name: "lda_c_vwn_rpa" },
    FunctionalTestCase { id: 9, name: "lda_c_pz" },
    FunctionalTestCase { id: 10, name: "lda_c_pz_mod" },
    FunctionalTestCase { id: 11, name: "lda_c_ob_pz" },
    FunctionalTestCase { id: 12, name: "lda_c_pw" },
    FunctionalTestCase { id: 13, name: "lda_c_pw_mod" },
    FunctionalTestCase { id: 14, name: "lda_c_ob_pw" },
    FunctionalTestCase { id: 15, name: "lda_c_2d_amgb" },
    FunctionalTestCase { id: 16, name: "lda_c_2d_prm" },
    FunctionalTestCase { id: 17, name: "lda_c_vbh" },
    FunctionalTestCase { id: 18, name: "lda_c_1d_csc" },
    FunctionalTestCase { id: 19, name: "lda_x_2d" },
    FunctionalTestCase { id: 20, name: "lda_xc_teter93" },
    FunctionalTestCase { id: 21, name: "lda_x_1d_soft" },
    FunctionalTestCase { id: 22, name: "lda_c_ml1" },
    FunctionalTestCase { id: 23, name: "lda_c_ml2" },
    FunctionalTestCase { id: 24, name: "lda_c_gombas" },
    FunctionalTestCase { id: 25, name: "lda_c_pw_rpa" },
    FunctionalTestCase { id: 26, name: "lda_c_1d_loos" },
    FunctionalTestCase { id: 27, name: "lda_c_rc04" },
    FunctionalTestCase { id: 28, name: "lda_c_vwn_1" },
    FunctionalTestCase { id: 29, name: "lda_c_vwn_2" },
    FunctionalTestCase { id: 30, name: "lda_c_vwn_3" },
    FunctionalTestCase { id: 31, name: "lda_c_vwn_4" },
    FunctionalTestCase { id: 43, name: "lda_xc_zlp" },
    FunctionalTestCase { id: 50, name: "lda_k_tf" },
    FunctionalTestCase { id: 51, name: "lda_k_lp" },
    FunctionalTestCase { id: 259, name: "lda_xc_ksdt" },
    FunctionalTestCase { id: 287, name: "lda_c_chachiyo" },
    FunctionalTestCase { id: 289, name: "lda_c_lp96" },
    FunctionalTestCase { id: 307, name: "lda_c_chachiyo_mod" },
    FunctionalTestCase { id: 308, name: "lda_c_karasiev_mod" },
    FunctionalTestCase { id: 317, name: "lda_c_w20" },
    FunctionalTestCase { id: 318, name: "lda_xc_corrksdt" },
    FunctionalTestCase { id: 532, name: "lda_x_rel" },
    FunctionalTestCase { id: 536, name: "lda_xc_1d_ehwlrg_1" },
    FunctionalTestCase { id: 537, name: "lda_xc_1d_ehwlrg_2" },
    FunctionalTestCase { id: 538, name: "lda_xc_1d_ehwlrg_3" },
    FunctionalTestCase { id: 546, name: "lda_x_erf" },
    FunctionalTestCase { id: 547, name: "lda_xc_lp_a" },
    FunctionalTestCase { id: 548, name: "lda_xc_lp_b" },
    FunctionalTestCase { id: 549, name: "lda_x_rae" },
    FunctionalTestCase { id: 550, name: "lda_k_zlp" },
    FunctionalTestCase { id: 551, name: "lda_c_mcweeny" },
    FunctionalTestCase { id: 552, name: "lda_c_br78" },
    FunctionalTestCase { id: 554, name: "lda_c_pk09" },
    FunctionalTestCase { id: 573, name: "lda_c_ow_lyp" },
    FunctionalTestCase { id: 574, name: "lda_c_ow" },
    FunctionalTestCase { id: 577, name: "lda_xc_gdsmfb" },
    FunctionalTestCase { id: 578, name: "lda_c_gk72" },
    FunctionalTestCase { id: 579, name: "lda_c_karasiev" },
    FunctionalTestCase { id: 580, name: "lda_k_lp96" },
    FunctionalTestCase { id: 588, name: "hyb_lda_xc_bn05" },
    FunctionalTestCase { id: 590, name: "lda_c_pmgb06" },
    FunctionalTestCase { id: 599, name: "lda_xc_tih" },
    FunctionalTestCase { id: 600, name: "lda_x_1d_exponential" },
    FunctionalTestCase { id: 641, name: "lda_x_yukawa" },
    FunctionalTestCase { id: 654, name: "lda_c_pw_erf" },
    FunctionalTestCase { id: 683, name: "lda_c_upw92" },
    FunctionalTestCase { id: 684, name: "lda_c_rpw92" },
    FunctionalTestCase { id: 692, name: "lda_x_sloc" },
];

// Tolerance tiers per D-10-R.
const TOL_EXC: f64 = 1e-12;
const TOL_VXC: f64 = 1e-10;
const TOL_FXC: f64 = 1e-8;
const TOL_KXC: f64 = 1e-6;
const TOL_LXC: f64 = 1e-4;

// ============================================================================
// Helpers (mirror verify/tests/lda_x_oracle.rs).
// ============================================================================

/// Generate logarithmically spaced density values from 10^min_exp to 10^max_exp.
fn log_spaced_densities(min_exp: f64, max_exp: f64, n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| 10.0_f64.powf(min_exp + (max_exp - min_exp) * i as f64 / (n - 1) as f64))
        .collect()
}

/// Generate polarized density test pairs with various spin ratios.
/// Uses moderate-to-high densities to avoid numerical instability in
/// higher-derivative cross terms.
fn polarized_test_densities() -> Vec<f64> {
    let total_densities = log_spaced_densities(-1.0, 3.0, 6);
    let ratios = [0.5, 0.7, 0.9, 0.99];
    let mut rho = Vec::new();
    for &total in &total_densities {
        for &ratio in &ratios {
            rho.push(total * ratio); // rho_up
            rho.push(total * (1.0 - ratio)); // rho_down
        }
    }
    rho
}

/// Compute relative error, treating values both below `abs_floor` as matching
/// (avoids spurious "infinite relative error" when both Rust and C produce
/// near-zero output at low density / high derivative cross terms).
fn rel_err_with_floor(rust_val: f64, c_val: f64, abs_floor: f64) -> f64 {
    if rust_val.abs() < abs_floor && c_val.abs() < abs_floor {
        return 0.0;
    }
    if c_val.abs() < 1e-300 {
        rust_val.abs()
    } else {
        ((rust_val - c_val) / c_val).abs()
    }
}

/// Run dispatch_lda for one (functional, order, spin) combination and return
/// the populated buffers. Returns `None` if dispatch_lda errors (e.g. unknown
/// derivative order on a vxc-only functional — handled at the caller).
fn run_rust_lda(
    functional: LdaFunctional,
    order: DerivativeOrder,
    spin: Spin,
    rho: &[f64],
) -> Option<LdaOracleOutput> {
    let np = if spin == Spin::Unpolarized { rho.len() } else { rho.len() / 2 };
    let input = LdaInput::new(rho, np, spin).ok()?;

    let (d_vrho, d_v2rho2, d_v3rho3, d_v4rho4) = if spin == Spin::Unpolarized {
        (1, 1, 1, 1)
    } else {
        (2, 3, 4, 5)
    };

    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * d_vrho];
    let mut v2 = vec![0.0f64; np * d_v2rho2];
    let mut v3 = vec![0.0f64; np * d_v3rho3];
    let mut v4 = vec![0.0f64; np * d_v4rho4];

    let mut output = LdaOutput::new(
        Some(&mut zk),
        if order >= DerivativeOrder::Vxc { Some(&mut vrho) } else { None },
        if order >= DerivativeOrder::Fxc { Some(&mut v2) } else { None },
        if order >= DerivativeOrder::Kxc { Some(&mut v3) } else { None },
        if order >= DerivativeOrder::Lxc { Some(&mut v4) } else { None },
        np,
        spin,
    )
    .ok()?;

    dispatch_lda(
        functional,
        &input,
        order,
        &mut output,
        &LdaFunctionalParams::default(),
        &Thresholds::default(),
    )
    .ok()?;

    Some(LdaOracleOutput {
        zk,
        vrho,
        v2rho2: v2,
        v3rho3: v3,
        v4rho4: v4,
    })
}

/// Compare Rust dispatch output against the C oracle for all derivative
/// orders the functional supports. Returns an error string on the first
/// mismatch outside tolerance, or Ok(()) if every supported order matches.
fn compare_lda_functional(
    tc: &FunctionalTestCase,
    functional: LdaFunctional,
    spin: Spin,
    rho: &[f64],
) -> Result<(), String> {
    let c_spin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let flags = oracle_func_flags(tc.id, c_spin).unwrap_or(0);

    // CRITICAL: oracle_lda_all calls xc_lda_exc_vxc_fxc_kxc + xc_lda_lxc.
    // libxc's xc_lda_exc_vxc_fxc_kxc exits(1) if the functional doesn't
    // implement Exc (e.g. lda_xc_tih). So we MUST skip this comparison
    // entirely when the C oracle has no EXC support, even if our Rust
    // kernel populates Vxc/Fxc/etc. The Rust output for vxc-only
    // functionals is exercised by the eval::dispatch::tests::test_lda_xc_tih_*
    // unit tests instead.
    if (flags & FLAGS_HAVE_EXC) == 0 {
        return Ok(());
    }

    let oracle = oracle_lda_all(tc.id, c_spin, rho)
        .map_err(|e| format!("oracle_lda_all({}={}, spin={c_spin}): {e}", tc.name, tc.id))?;

    // exc — only when the functional has exc kernels and the oracle reports HAVE_EXC.
    if functional.has_exc() && (flags & FLAGS_HAVE_EXC) != 0 {
        let rust = run_rust_lda(functional, DerivativeOrder::Exc, spin, rho)
            .ok_or_else(|| format!("{} exc {:?}: dispatch_lda returned error", tc.name, spin))?;
        for i in 0..rust.zk.len() {
            let e = rel_err_with_floor(rust.zk[i], oracle.zk[i], 1e-12);
            if e > TOL_EXC {
                return Err(format!(
                    "{} exc {:?}.zk[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                    tc.name, spin, rust.zk[i], oracle.zk[i], e
                ));
            }
        }
    }

    // vxc — compare zk at exc tolerance and vrho at vxc tolerance.
    if (flags & FLAGS_HAVE_VXC) != 0 {
        let rust = run_rust_lda(functional, DerivativeOrder::Vxc, spin, rho)
            .ok_or_else(|| format!("{} vxc {:?}: dispatch_lda returned error", tc.name, spin))?;
        if functional.has_exc() {
            for i in 0..rust.zk.len() {
                let e = rel_err_with_floor(rust.zk[i], oracle.zk[i], 1e-12);
                if e > TOL_EXC {
                    return Err(format!(
                        "{} vxc {:?}.zk[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                        tc.name, spin, rust.zk[i], oracle.zk[i], e
                    ));
                }
            }
        }
        for i in 0..rust.vrho.len() {
            let e = rel_err_with_floor(rust.vrho[i], oracle.vrho[i], 1e-10);
            if e > TOL_VXC {
                return Err(format!(
                    "{} vxc {:?}.vrho[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                    tc.name, spin, rust.vrho[i], oracle.vrho[i], e
                ));
            }
        }
    }

    // fxc — v2rho2 at TOL_FXC.
    if (flags & FLAGS_HAVE_FXC) != 0 {
        let rust = run_rust_lda(functional, DerivativeOrder::Fxc, spin, rho)
            .ok_or_else(|| format!("{} fxc {:?}: dispatch_lda returned error", tc.name, spin))?;
        for i in 0..rust.v2rho2.len() {
            let e = rel_err_with_floor(rust.v2rho2[i], oracle.v2rho2[i], 1e-8);
            if e > TOL_FXC {
                return Err(format!(
                    "{} fxc {:?}.v2rho2[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                    tc.name, spin, rust.v2rho2[i], oracle.v2rho2[i], e
                ));
            }
        }
    }

    // kxc — v3rho3 at TOL_KXC.
    if (flags & FLAGS_HAVE_KXC) != 0 {
        let rust = run_rust_lda(functional, DerivativeOrder::Kxc, spin, rho)
            .ok_or_else(|| format!("{} kxc {:?}: dispatch_lda returned error", tc.name, spin))?;
        for i in 0..rust.v3rho3.len() {
            let e = rel_err_with_floor(rust.v3rho3[i], oracle.v3rho3[i], 1e-6);
            if e > TOL_KXC {
                return Err(format!(
                    "{} kxc {:?}.v3rho3[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                    tc.name, spin, rust.v3rho3[i], oracle.v3rho3[i], e
                ));
            }
        }
    }

    // lxc — v4rho4 at TOL_LXC.
    if (flags & FLAGS_HAVE_LXC) != 0 {
        let rust = run_rust_lda(functional, DerivativeOrder::Lxc, spin, rho)
            .ok_or_else(|| format!("{} lxc {:?}: dispatch_lda returned error", tc.name, spin))?;
        for i in 0..rust.v4rho4.len() {
            let e = rel_err_with_floor(rust.v4rho4[i], oracle.v4rho4[i], 1e-4);
            if e > TOL_LXC {
                return Err(format!(
                    "{} lxc {:?}.v4rho4[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                    tc.name, spin, rust.v4rho4[i], oracle.v4rho4[i], e
                ));
            }
        }
    }

    Ok(())
}

/// Resolve a functional test-case row into a routable `LdaFunctional`.
///
/// Returns one of:
///   - `Ok(Some(functional))` — the ID is compiled and routable.
///   - `Ok(None)` — the ID is not in this family / not routable / not yet
///                  translated. Caller should skip with a log line.
///   - `Err(reason)` — internal error (e.g. registry lookup failure for an
///                     ID we expected to be valid).
///
/// **B1 fix:** uses the public `FunctionalId::from_raw` constructor instead
/// of tuple syntax. Tuple construction `FunctionalId(...)` would fail to
/// compile here because the inner field is `pub(crate)` and this is a
/// separate crate.
fn resolve_functional(tc: &FunctionalTestCase) -> Result<Option<LdaFunctional>, String> {
    let fid = match FunctionalId::from_raw(tc.id as u16) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };
    match LdaFunctional::from_id(fid) {
        Ok(f) => Ok(Some(f)),
        Err(_) => Ok(None),
    }
}

// ============================================================================
// Tests
// ============================================================================

/// Per-functional Rust-vs-C oracle comparison for every compiled LDA
/// functional in unpolarized mode, at all five derivative tolerance tiers.
#[test]
fn test_all_lda_oracle_unpol() {
    // Use a moderate density range that all functionals can evaluate without
    // tripping low-density divergence in higher-derivative cross terms.
    let rho = log_spaced_densities(-1.0, 3.0, 8);

    let mut failures: Vec<String> = Vec::new();
    let mut skipped_no_exc = 0_usize;
    let mut skipped_deferred = 0_usize;
    let mut skipped_not_compiled = 0_usize;
    let mut tested = 0_usize;

    for tc in LDA_FUNCTIONALS {
        if is_deferred(tc.id as u16) {
            eprintln!(
                "SKIP {} (id={}): deferred (CubeCL proc-macro stack)",
                tc.name, tc.id
            );
            skipped_deferred += 1;
            continue;
        }
        let functional = match resolve_functional(tc) {
            Ok(Some(f)) => f,
            Ok(None) => {
                eprintln!(
                    "SKIP {} (id={}): not compiled in crates/kernel-lda",
                    tc.name, tc.id
                );
                skipped_not_compiled += 1;
                continue;
            }
            Err(msg) => {
                failures.push(format!("{} (id={}): resolve_functional: {msg}", tc.name, tc.id));
                continue;
            }
        };
        let flags = oracle_func_flags(tc.id, 1).unwrap_or(0);
        if functional.has_exc() && (flags & FLAGS_HAVE_EXC) == 0 {
            eprintln!("SKIP {} (id={}): oracle reports no EXC", tc.name, tc.id);
            skipped_no_exc += 1;
            continue;
        }

        match compare_lda_functional(tc, functional, Spin::Unpolarized, &rho) {
            Ok(()) => tested += 1,
            Err(msg) => failures.push(msg),
        }
    }

    eprintln!(
        "LDA unpol summary: tested={tested} \
         skipped_no_exc={skipped_no_exc} \
         skipped_deferred={skipped_deferred} \
         skipped_not_compiled={skipped_not_compiled} \
         failures={}",
        failures.len()
    );
    assert_eq!(
        skipped_deferred, 4,
        "expected exactly 4 deferred LDA functionals (lda_c_pk09, lda_xc_ksdt, lda_c_pw_erf, lda_c_pmgb06)"
    );
    assert!(
        tested >= 30,
        "tested={tested} should be >= 30 (we have ~38 routable functionals minus a few without EXC oracle support)"
    );
    assert!(
        failures.is_empty(),
        "LDA unpolarized oracle failures ({} of {} tested):\n  {}",
        failures.len(),
        tested + failures.len(),
        failures.join("\n  ")
    );
}

/// Per-functional Rust-vs-C oracle comparison for every compiled LDA
/// functional in polarized (spin=2) mode.
#[test]
fn test_all_lda_oracle_pol() {
    let rho = polarized_test_densities();

    let mut failures: Vec<String> = Vec::new();
    let mut skipped_no_exc = 0_usize;
    let mut skipped_deferred = 0_usize;
    let mut skipped_not_compiled = 0_usize;
    let mut tested = 0_usize;

    for tc in LDA_FUNCTIONALS {
        if is_deferred(tc.id as u16) {
            eprintln!(
                "SKIP {} (id={}): deferred (CubeCL proc-macro stack)",
                tc.name, tc.id
            );
            skipped_deferred += 1;
            continue;
        }
        let functional = match resolve_functional(tc) {
            Ok(Some(f)) => f,
            Ok(None) => {
                eprintln!(
                    "SKIP {} (id={}): not compiled in crates/kernel-lda",
                    tc.name, tc.id
                );
                skipped_not_compiled += 1;
                continue;
            }
            Err(msg) => {
                failures.push(format!("{} (id={}): resolve_functional: {msg}", tc.name, tc.id));
                continue;
            }
        };
        let flags = oracle_func_flags(tc.id, 2).unwrap_or(0);
        if functional.has_exc() && (flags & FLAGS_HAVE_EXC) == 0 {
            eprintln!("SKIP {} (id={}): oracle reports no EXC (pol)", tc.name, tc.id);
            skipped_no_exc += 1;
            continue;
        }

        match compare_lda_functional(tc, functional, Spin::Polarized, &rho) {
            Ok(()) => tested += 1,
            Err(msg) => failures.push(msg),
        }
    }

    eprintln!(
        "LDA pol summary: tested={tested} \
         skipped_no_exc={skipped_no_exc} \
         skipped_deferred={skipped_deferred} \
         skipped_not_compiled={skipped_not_compiled} \
         failures={}",
        failures.len()
    );
    assert_eq!(
        skipped_deferred, 4,
        "expected exactly 4 deferred LDA functionals (lda_c_pk09, lda_xc_ksdt, lda_c_pw_erf, lda_c_pmgb06)"
    );
    assert!(
        tested >= 30,
        "tested={tested} should be >= 30"
    );
    assert!(
        failures.is_empty(),
        "LDA polarized oracle failures ({} of {} tested):\n  {}",
        failures.len(),
        tested + failures.len(),
        failures.join("\n  ")
    );
}
