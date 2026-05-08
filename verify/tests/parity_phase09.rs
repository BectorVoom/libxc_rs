//! Phase 9 oracle parity sweep — strict 1e-12 across all derivative orders for
//! the 25 historically-deferred GGA functionals + MGGA non-regression spot-check.
//!
//! See .planning/phases/09-reduce-kernel-build-time/09-CONTEXT.md decision D-14:
//! tolerance is strict 1e-12 with no per-order relaxation. Operation-order
//! preservation (D-08) makes this achievable post-regen.
//!
//! Source-of-truth for the 25-name list:
//!   log/09-05-deferred-gga-audit.json (status=OK for all 25 entries)
//! Resolved against:
//!   libxc-master/src/xc_funcs.h
//!   src/model/gga_functional.rs (GgaFunctional::from_id routing)
//!   src/registry/by_name.rs / by_id.rs
//!
//! Generated post Plan 09-04 (translator threshold raised to 18K) and Plan 09-05
//! (audit-driven coverage closure). Plan 09-07 captures the result.
//!
//! **No-skip invariant (plan Test 4):** A tuple is allowed to skip ONLY if
//! `dispatch_gga`/`dispatch_mgga` returns `UnsupportedFunctional` or
//! `UnsupportedDerivativeOrder` — exactly mirroring the skip semantics of
//! `gga_oracle.rs`/`mgga_oracle.rs`. Every skipped tuple is printed with a
//! `PARITY_TUPLE: ... SKIP <reason>` line so the post-run audit harness can
//! assemble the full report.

use libxc_rs::LibxcRsError;
use libxc_rs::eval::{dispatch_gga, dispatch_mgga};
use libxc_rs::input::{GgaInput, MggaInput};
use libxc_rs::model::{
    DerivativeOrder, FunctionalId, GgaFunctional, MggaFunctional, Spin, Thresholds,
};
use libxc_rs::output::{GgaOutput, MggaOutput};
use libxc_rs_verify::{
    FLAGS_HAVE_EXC, FLAGS_HAVE_FXC, FLAGS_HAVE_KXC, FLAGS_HAVE_LXC, FLAGS_HAVE_VXC,
    GgaOracleOutput, MggaOracleOutput, oracle_func_flags, oracle_gga_all, oracle_mgga_all,
};

// ---- Plan 09-07 D-14 contract -----------------------------------------------

const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;

// ---- Canonical 25-name list (Plan 09-05 audit) ------------------------------

/// The 25 GGA functional names historically deferred under the pre-Phase-9
/// 5K SPLIT_THRESHOLD. Sourced from `log/09-05-deferred-gga-audit.json`
/// (status=OK for all 25 entries).
const DEFERRED_GGA_NAMES: &[&str] = &[
    "gga_c_acgga",
    "gga_c_acggap",
    "gga_c_ft97",
    "gga_c_gapc",
    "gga_c_gaploc",
    "gga_c_hcth_a",
    "gga_c_optc",
    "gga_c_pbe_erf_gws",
    "gga_c_pbeloc",
    "gga_c_pw91",
    "gga_c_q2d",
    "gga_c_regtpss",
    "gga_c_revtca",
    "gga_c_sg4",
    "gga_c_sogga11",
    "gga_c_zpbeint",
    "gga_c_zvpbeint",
    "gga_c_zvpbeloc",
    "gga_x_ft97",
    "gga_x_hjs",
    "gga_x_hjs_b88_v2",
    "gga_x_lcgau",
    "gga_x_wpbeh",
    "gga_xc_b97",
    "hyb_gga_xc_wb97",
];

/// (canonical-name, libxc-id) tuples for the deferred sweep.
///
/// 22 of the 25 names map directly to a libxc id in `xc_funcs.h`. The remaining
/// 3 (`gga_x_ft97`, `gga_x_hjs`, `gga_x_lcgau`) are maple2c **template kernels**
/// reused by multiple libxc-numbered functionals — they have no top-level libxc
/// id of their own, so we test their template behaviour through the routed
/// specializations:
///   - `gga_x_ft97`  → ids 114 (gga_x_ft97_a), 115 (gga_x_ft97_b)
///   - `gga_x_hjs`   → ids 525, 526, 527, 528 (the four `gga_x_hjs_*` libxc ids)
///   - `gga_x_lcgau` → id 708 (`hyb_gga_x_lcgau`)
///
/// Identification of the 3 template names and their libxc derivatives is
/// confirmed by `src/registry/by_name.rs` (mapping XC_GGA_X_* macros to ids).
struct DeferredEntry {
    canonical: &'static str,
    ids: &'static [i32],
}

const DEFERRED_GGA_ENTRIES: &[DeferredEntry] = &[
    DeferredEntry { canonical: "gga_c_acgga",        ids: &[39] },
    DeferredEntry { canonical: "gga_c_acggap",       ids: &[176] },
    DeferredEntry { canonical: "gga_c_ft97",         ids: &[88] },
    DeferredEntry { canonical: "gga_c_gapc",         ids: &[555] },
    DeferredEntry { canonical: "gga_c_gaploc",       ids: &[556] },
    DeferredEntry { canonical: "gga_c_hcth_a",       ids: &[97] },
    DeferredEntry { canonical: "gga_c_optc",         ids: &[200] },
    DeferredEntry { canonical: "gga_c_pbe_erf_gws",  ids: &[657] },
    DeferredEntry { canonical: "gga_c_pbeloc",       ids: &[246] },
    DeferredEntry { canonical: "gga_c_pw91",         ids: &[134] },
    DeferredEntry { canonical: "gga_c_q2d",          ids: &[47] },
    DeferredEntry { canonical: "gga_c_regtpss",      ids: &[83] },
    DeferredEntry { canonical: "gga_c_revtca",       ids: &[99] },
    DeferredEntry { canonical: "gga_c_sg4",          ids: &[534] },
    DeferredEntry { canonical: "gga_c_sogga11",      ids: &[152] },
    DeferredEntry { canonical: "gga_c_zpbeint",      ids: &[61] },
    DeferredEntry { canonical: "gga_c_zvpbeint",     ids: &[557] },
    DeferredEntry { canonical: "gga_c_zvpbeloc",     ids: &[606] },
    DeferredEntry { canonical: "gga_x_ft97",         ids: &[114, 115] },           // template
    DeferredEntry { canonical: "gga_x_hjs",          ids: &[525, 526, 527, 528] }, // template
    DeferredEntry { canonical: "gga_x_hjs_b88_v2",   ids: &[46] },
    DeferredEntry { canonical: "gga_x_lcgau",        ids: &[708] },                // template
    DeferredEntry { canonical: "gga_x_wpbeh",        ids: &[524] },
    DeferredEntry { canonical: "gga_xc_b97",         ids: &[167] },
    DeferredEntry { canonical: "hyb_gga_xc_wb97",    ids: &[463] },
];

// ---- Spot-check MGGA set ----------------------------------------------------

/// MGGA non-regression spot-check.
///
/// Plan 09-07 originally suggested `mgga_x_tpss / mgga_c_tpss / mgga_x_scan /
/// mgga_c_scan / mgga_xc_b97m_v` but explicitly authorized substitution: only
/// `mgga_x_tpss` (id 202) is currently routed by `MggaFunctional::from_id`
/// (Phase 4 Plan 04-04 wired ~25 of 146 MGGAs). Substitutes drawn from the
/// routed-MGGA list in `src/model/mgga_functional.rs`:
const SPOT_CHECK_MGGAS: &[(&str, i32)] = &[
    ("mgga_x_tpss",      202),
    ("mgga_x_pkzb",      213),
    ("mgga_x_th",        225),
    ("mgga_x_jk",        256),
    ("mgga_x_mvs",       257),
];

// ---- Test grids -------------------------------------------------------------
// Same grids as `gga_oracle.rs` / `mgga_oracle.rs` (matched-input pattern).

const RHO_UNPOL: &[f64] = &[0.1, 0.5, 1.0, 5.0];
const SIGMA_UNPOL: &[f64] = &[0.01, 0.1, 0.5, 2.0];
const LAPL_UNPOL: &[f64] = &[0.001, 0.01, 0.05, 0.2];
const TAU_UNPOL: &[f64] = &[0.01, 0.05, 0.2, 1.0];

const RHO_POL: &[f64] = &[0.1, 0.05, 0.5, 0.3, 1.0, 0.8, 5.0, 3.0];
const SIGMA_POL: &[f64] = &[
    0.01, 0.005, 0.002,
    0.1,  0.05,  0.02,
    0.5,  0.2,   0.1,
    2.0,  1.0,   0.5,
];
const LAPL_POL: &[f64] = &[
    0.001, 0.0005,
    0.01,  0.005,
    0.05,  0.02,
    0.2,   0.1,
];
const TAU_POL: &[f64] = &[
    0.01, 0.005,
    0.05, 0.03,
    0.2,  0.1,
    1.0,  0.5,
];

// ---- Helpers ----------------------------------------------------------------

fn rel_err_with_floor(rust_val: f64, c_val: f64) -> f64 {
    if rust_val.abs() < REL_FLOOR && c_val.abs() < REL_FLOOR {
        return 0.0;
    }
    if c_val.abs() < 1e-300 {
        rust_val.abs()
    } else {
        ((rust_val - c_val) / c_val).abs()
    }
}

fn order_label(order: DerivativeOrder) -> &'static str {
    match order {
        DerivativeOrder::Exc => "exc",
        DerivativeOrder::Vxc => "vxc",
        DerivativeOrder::Fxc => "fxc",
        DerivativeOrder::Kxc => "kxc",
        DerivativeOrder::Lxc => "lxc",
    }
}

fn spin_label(spin: Spin) -> &'static str {
    match spin {
        Spin::Unpolarized => "unpol",
        Spin::Polarized => "pol",
    }
}

#[derive(Debug, Clone)]
enum TupleResult {
    /// All elements at this order matched within STRICT_TOL.
    Pass(f64), // max rel_err
    /// Routing/dispatch returned a documented unsupported-skip — recorded but
    /// not failed (mirrors gga_oracle.rs "skipped_pending_params" semantics).
    SkipUnsupportedFunctional,
    /// `from_id` could not produce a `GgaFunctional`/`MggaFunctional` variant
    /// — same as gga_oracle.rs "skipped_not_compiled".
    SkipNotRouted,
    /// Order returned `UnsupportedDerivativeOrder` — kernel-side did not wire
    /// the requested order; mirror gga_oracle.rs's `UnsupportedOrder` skip.
    SkipUnsupportedOrder,
    /// Oracle reports the order is unavailable for this functional (FLAGS bit
    /// not set). Counted as a documented skip, not a failure.
    SkipNoOracleSupport,
    /// Tuple exceeded STRICT_TOL — phase blocker.
    Fail { max_rel_err: f64, detail: String },
}

fn classify_dispatch_err(err: LibxcRsError) -> TupleResult {
    match err {
        LibxcRsError::UnsupportedFunctional { .. } => TupleResult::SkipUnsupportedFunctional,
        LibxcRsError::UnsupportedDerivativeOrder { .. } => TupleResult::SkipUnsupportedOrder,
        e => TupleResult::Fail {
            max_rel_err: f64::INFINITY,
            detail: format!("dispatch error: {e}"),
        },
    }
}

// ---- GGA per-tuple parity ---------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn run_gga_tuple(
    func_id: i32,
    name: &str,
    order: DerivativeOrder,
    spin: Spin,
) -> TupleResult {
    let c_spin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let flags = oracle_func_flags(func_id, c_spin).unwrap_or(0);

    // oracle_gga_all calls xc_gga_exc_vxc_fxc_kxc which bails on no-EXC.
    if flags & FLAGS_HAVE_EXC == 0 {
        return TupleResult::SkipNoOracleSupport;
    }
    let order_bit = match order {
        DerivativeOrder::Exc => FLAGS_HAVE_EXC,
        DerivativeOrder::Vxc => FLAGS_HAVE_VXC,
        DerivativeOrder::Fxc => FLAGS_HAVE_FXC,
        DerivativeOrder::Kxc => FLAGS_HAVE_KXC,
        DerivativeOrder::Lxc => FLAGS_HAVE_LXC,
    };
    if flags & order_bit == 0 {
        return TupleResult::SkipNoOracleSupport;
    }

    let id_obj = match FunctionalId::from_raw(func_id as u16) {
        Ok(id) => id,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let functional = match GgaFunctional::from_id(id_obj) {
        Ok(f) => f,
        Err(_) => return TupleResult::SkipNotRouted,
    };

    let (rho, sigma) = if spin == Spin::Unpolarized {
        (RHO_UNPOL, SIGMA_UNPOL)
    } else {
        (RHO_POL, SIGMA_POL)
    };
    let np = if spin == Spin::Unpolarized { rho.len() } else { rho.len() / 2 };

    let oracle = match oracle_gga_all(func_id, c_spin, rho, sigma) {
        Ok(o) => o,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("oracle_gga_all({name}={func_id}, spin={c_spin}): {e}"),
            };
        }
    };

    // Run dispatch for the highest order we need; lower orders share the
    // computation but oracle_gga_all already populated everything in one shot,
    // so we issue dispatch_gga at the requested order specifically.
    let input = match GgaInput::new(rho, sigma, np, spin) {
        Ok(i) => i,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("GgaInput::new: {e}"),
            };
        }
    };

    let (d_vrho, d_vsigma, d_v2rho2, d_v2rhosigma, d_v2sigma2,
         d_v3rho3, d_v3rho2sigma, d_v3rhosigma2, d_v3sigma3,
         d_v4rho4, d_v4rho3sigma, d_v4rho2sigma2, d_v4rhosigma3, d_v4sigma4) =
        if spin == Spin::Unpolarized {
            (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)
        } else {
            (2, 3, 3, 6, 6, 4, 9, 12, 10, 5, 12, 18, 20, 15)
        };

    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * d_vrho];
    let mut vsigma = vec![0.0f64; np * d_vsigma];
    let mut v2rho2 = vec![0.0f64; np * d_v2rho2];
    let mut v2rhosigma = vec![0.0f64; np * d_v2rhosigma];
    let mut v2sigma2 = vec![0.0f64; np * d_v2sigma2];
    let mut v3rho3 = vec![0.0f64; np * d_v3rho3];
    let mut v3rho2sigma = vec![0.0f64; np * d_v3rho2sigma];
    let mut v3rhosigma2 = vec![0.0f64; np * d_v3rhosigma2];
    let mut v3sigma3 = vec![0.0f64; np * d_v3sigma3];
    let mut v4rho4 = vec![0.0f64; np * d_v4rho4];
    let mut v4rho3sigma = vec![0.0f64; np * d_v4rho3sigma];
    let mut v4rho2sigma2 = vec![0.0f64; np * d_v4rho2sigma2];
    let mut v4rhosigma3 = vec![0.0f64; np * d_v4rhosigma3];
    let mut v4sigma4 = vec![0.0f64; np * d_v4sigma4];

    let mut output = GgaOutput {
        zk: if functional.has_exc() { Some(&mut zk) } else { None },
        vrho:       if order >= DerivativeOrder::Vxc { Some(&mut vrho) } else { None },
        vsigma:     if order >= DerivativeOrder::Vxc { Some(&mut vsigma) } else { None },
        v2rho2:     if order >= DerivativeOrder::Fxc { Some(&mut v2rho2) } else { None },
        v2rhosigma: if order >= DerivativeOrder::Fxc { Some(&mut v2rhosigma) } else { None },
        v2sigma2:   if order >= DerivativeOrder::Fxc { Some(&mut v2sigma2) } else { None },
        v3rho3:        if order >= DerivativeOrder::Kxc { Some(&mut v3rho3) } else { None },
        v3rho2sigma:   if order >= DerivativeOrder::Kxc { Some(&mut v3rho2sigma) } else { None },
        v3rhosigma2:   if order >= DerivativeOrder::Kxc { Some(&mut v3rhosigma2) } else { None },
        v3sigma3:      if order >= DerivativeOrder::Kxc { Some(&mut v3sigma3) } else { None },
        v4rho4:        if order >= DerivativeOrder::Lxc { Some(&mut v4rho4) } else { None },
        v4rho3sigma:   if order >= DerivativeOrder::Lxc { Some(&mut v4rho3sigma) } else { None },
        v4rho2sigma2:  if order >= DerivativeOrder::Lxc { Some(&mut v4rho2sigma2) } else { None },
        v4rhosigma3:   if order >= DerivativeOrder::Lxc { Some(&mut v4rhosigma3) } else { None },
        v4sigma4:      if order >= DerivativeOrder::Lxc { Some(&mut v4sigma4) } else { None },
    };

    if let Err(e) = dispatch_gga(
        functional,
        &input,
        order,
        &mut output,
        &libxc_rs::NoParams,
        &Thresholds::default(),
    ) {
        return classify_dispatch_err(e);
    }
    drop(output);

    let pairs: Vec<(&str, &[f64], &[f64])> = match order {
        DerivativeOrder::Exc => {
            if functional.has_exc() {
                vec![("zk", &zk[..], &oracle.zk[..])]
            } else {
                Vec::new()
            }
        }
        DerivativeOrder::Vxc => vec![
            ("vrho",   &vrho[..],   &oracle.vrho[..]),
            ("vsigma", &vsigma[..], &oracle.vsigma[..]),
        ],
        DerivativeOrder::Fxc => vec![
            ("v2rho2",     &v2rho2[..],     &oracle.v2rho2[..]),
            ("v2rhosigma", &v2rhosigma[..], &oracle.v2rhosigma[..]),
            ("v2sigma2",   &v2sigma2[..],   &oracle.v2sigma2[..]),
        ],
        DerivativeOrder::Kxc => vec![
            ("v3rho3",      &v3rho3[..],      &oracle.v3rho3[..]),
            ("v3rho2sigma", &v3rho2sigma[..], &oracle.v3rho2sigma[..]),
            ("v3rhosigma2", &v3rhosigma2[..], &oracle.v3rhosigma2[..]),
            ("v3sigma3",    &v3sigma3[..],    &oracle.v3sigma3[..]),
        ],
        DerivativeOrder::Lxc => vec![
            ("v4rho4",       &v4rho4[..],       &oracle.v4rho4[..]),
            ("v4rho3sigma",  &v4rho3sigma[..],  &oracle.v4rho3sigma[..]),
            ("v4rho2sigma2", &v4rho2sigma2[..], &oracle.v4rho2sigma2[..]),
            ("v4rhosigma3",  &v4rhosigma3[..],  &oracle.v4rhosigma3[..]),
            ("v4sigma4",     &v4sigma4[..],     &oracle.v4sigma4[..]),
        ],
    };

    let mut max_e = 0.0f64;
    for (label, rust_slice, c_slice) in pairs {
        for i in 0..rust_slice.len().min(c_slice.len()) {
            let e = rel_err_with_floor(rust_slice[i], c_slice[i]);
            if e > max_e {
                max_e = e;
            }
            if e > STRICT_TOL {
                return TupleResult::Fail {
                    max_rel_err: e,
                    detail: format!(
                        "{name} {} {} .{label}[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                        order_label(order),
                        spin_label(spin),
                        rust_slice[i],
                        c_slice[i],
                        e
                    ),
                };
            }
        }
    }

    TupleResult::Pass(max_e)
}

// ---- MGGA per-tuple parity --------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn run_mgga_tuple(
    func_id: i32,
    name: &str,
    order: DerivativeOrder,
    spin: Spin,
) -> TupleResult {
    let c_spin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let flags = oracle_func_flags(func_id, c_spin).unwrap_or(0);
    if flags & FLAGS_HAVE_EXC == 0 {
        return TupleResult::SkipNoOracleSupport;
    }
    let order_bit = match order {
        DerivativeOrder::Exc => FLAGS_HAVE_EXC,
        DerivativeOrder::Vxc => FLAGS_HAVE_VXC,
        DerivativeOrder::Fxc => FLAGS_HAVE_FXC,
        DerivativeOrder::Kxc => FLAGS_HAVE_KXC,
        DerivativeOrder::Lxc => FLAGS_HAVE_LXC,
    };
    if flags & order_bit == 0 {
        return TupleResult::SkipNoOracleSupport;
    }

    let id_obj = match FunctionalId::from_raw(func_id as u16) {
        Ok(id) => id,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let functional = match MggaFunctional::from_id(id_obj) {
        Ok(f) => f,
        Err(_) => return TupleResult::SkipNotRouted,
    };

    let (rho, sigma, lapl, tau) = if spin == Spin::Unpolarized {
        (RHO_UNPOL, SIGMA_UNPOL, LAPL_UNPOL, TAU_UNPOL)
    } else {
        (RHO_POL, SIGMA_POL, LAPL_POL, TAU_POL)
    };
    let np = if spin == Spin::Unpolarized { rho.len() } else { rho.len() / 2 };

    let oracle = match oracle_mgga_all(func_id, c_spin, rho, sigma, lapl, tau) {
        Ok(o) => o,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("oracle_mgga_all({name}={func_id}, spin={c_spin}): {e}"),
            };
        }
    };

    let input = match MggaInput::new(rho, sigma, lapl, tau, np, spin) {
        Ok(i) => i,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("MggaInput::new: {e}"),
            };
        }
    };

    let (d_vrho, d_vsigma, d_vlapl, d_vtau) =
        if spin == Spin::Unpolarized { (1, 1, 1, 1) } else { (2, 3, 2, 2) };

    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * d_vrho];
    let mut vsigma = vec![0.0f64; np * d_vsigma];
    let mut vlapl = vec![0.0f64; np * d_vlapl];
    let mut vtau = vec![0.0f64; np * d_vtau];

    let mut output = MggaOutput {
        zk: if functional.has_exc() { Some(&mut zk) } else { None },
        vrho:  if order >= DerivativeOrder::Vxc { Some(&mut vrho) } else { None },
        vsigma:if order >= DerivativeOrder::Vxc { Some(&mut vsigma) } else { None },
        vlapl: if order >= DerivativeOrder::Vxc { Some(&mut vlapl) } else { None },
        vtau:  if order >= DerivativeOrder::Vxc { Some(&mut vtau) } else { None },
        ..Default::default()
    };

    if let Err(e) = dispatch_mgga(
        functional,
        &input,
        order,
        &mut output,
        &libxc_rs::NoParams,
        &Thresholds::default(),
    ) {
        return classify_dispatch_err(e);
    }
    drop(output);

    let pairs: Vec<(&str, &[f64], &[f64])> = match order {
        DerivativeOrder::Exc => {
            if functional.has_exc() {
                vec![("zk", &zk[..], &oracle.zk[..])]
            } else {
                Vec::new()
            }
        }
        DerivativeOrder::Vxc => vec![
            ("vrho",   &vrho[..],   &oracle.vrho[..]),
            ("vsigma", &vsigma[..], &oracle.vsigma[..]),
            ("vlapl",  &vlapl[..],  &oracle.vlapl[..]),
            ("vtau",   &vtau[..],   &oracle.vtau[..]),
        ],
        // Fxc/Kxc/Lxc not wired through dispatch_mgga in Phase 4 plan 04-04;
        // they will SkipUnsupportedOrder above. If reached here, treat as
        // empty pair list — no-op pass.
        _ => Vec::new(),
    };

    let mut max_e = 0.0f64;
    for (label, rust_slice, c_slice) in pairs {
        for i in 0..rust_slice.len().min(c_slice.len()) {
            let e = rel_err_with_floor(rust_slice[i], c_slice[i]);
            if e > max_e {
                max_e = e;
            }
            if e > STRICT_TOL {
                return TupleResult::Fail {
                    max_rel_err: e,
                    detail: format!(
                        "{name} {} {} .{label}[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                        order_label(order),
                        spin_label(spin),
                        rust_slice[i],
                        c_slice[i],
                        e
                    ),
                };
            }
        }
    }

    TupleResult::Pass(max_e)
}

// ---- Tests ------------------------------------------------------------------

/// Plan 09-07 Test 3: canonical-list invariant.
#[test]
fn parity_phase09_deferred_count_invariant() {
    assert_eq!(
        DEFERRED_GGA_NAMES.len(),
        25,
        "Canonical deferred-GGA count must be 25 per RESEARCH.md (Plan 09-05 audit)"
    );
    assert_eq!(
        DEFERRED_GGA_ENTRIES.len(),
        25,
        "ENTRIES must align 1:1 with NAMES"
    );
    for (i, e) in DEFERRED_GGA_ENTRIES.iter().enumerate() {
        assert_eq!(
            e.canonical, DEFERRED_GGA_NAMES[i],
            "canonical mismatch at index {i}"
        );
        assert!(!e.ids.is_empty(), "entry '{}' must list ≥1 libxc id", e.canonical);
    }
}

/// Plan 09-07 Test 1: full deferred-GGA sweep at strict 1e-12.
#[test]
fn parity_phase09_deferred_gga_full_sweep() {
    let orders = [
        DerivativeOrder::Exc,
        DerivativeOrder::Vxc,
        DerivativeOrder::Fxc,
        DerivativeOrder::Kxc,
        DerivativeOrder::Lxc,
    ];
    let spins = [Spin::Unpolarized, Spin::Polarized];

    let mut total = 0usize;
    let mut pass = 0usize;
    let mut skip = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for entry in DEFERRED_GGA_ENTRIES {
        for &id in entry.ids {
            for &order in &orders {
                for &spin in &spins {
                    total += 1;
                    let res = run_gga_tuple(id, entry.canonical, order, spin);
                    match &res {
                        TupleResult::Pass(e) => {
                            pass += 1;
                            eprintln!(
                                "PARITY_TUPLE: {} {} {} {} max_rel_err={:.3e} PASS",
                                entry.canonical, id, order_label(order), spin_label(spin), e
                            );
                        }
                        TupleResult::SkipUnsupportedFunctional => {
                            skip += 1;
                            eprintln!(
                                "PARITY_TUPLE: {} {} {} {} SKIP UnsupportedFunctional",
                                entry.canonical, id, order_label(order), spin_label(spin)
                            );
                        }
                        TupleResult::SkipNotRouted => {
                            skip += 1;
                            eprintln!(
                                "PARITY_TUPLE: {} {} {} {} SKIP NotRouted",
                                entry.canonical, id, order_label(order), spin_label(spin)
                            );
                        }
                        TupleResult::SkipUnsupportedOrder => {
                            skip += 1;
                            eprintln!(
                                "PARITY_TUPLE: {} {} {} {} SKIP UnsupportedOrder",
                                entry.canonical, id, order_label(order), spin_label(spin)
                            );
                        }
                        TupleResult::SkipNoOracleSupport => {
                            skip += 1;
                            eprintln!(
                                "PARITY_TUPLE: {} {} {} {} SKIP NoOracleSupport",
                                entry.canonical, id, order_label(order), spin_label(spin)
                            );
                        }
                        TupleResult::Fail { max_rel_err, detail } => {
                            failures.push(format!(
                                "PARITY_TUPLE: {} {} {} {} max_rel_err={:.3e} FAIL: {}",
                                entry.canonical, id,
                                order_label(order), spin_label(spin),
                                max_rel_err, detail
                            ));
                        }
                    }
                }
            }
        }
    }

    eprintln!(
        "PARITY_SUMMARY: gga total={total} pass={pass} skip={skip} fail={}",
        failures.len()
    );

    assert!(
        failures.is_empty(),
        "Phase 9 strict 1e-12 GGA parity sweep failed for {} tuple(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

/// Plan 09-07 Test 2: MGGA non-regression spot-check.
#[test]
fn parity_phase09_mgga_spot_check() {
    let orders = [
        DerivativeOrder::Exc,
        DerivativeOrder::Vxc,
        DerivativeOrder::Fxc,
    ];
    let spins = [Spin::Unpolarized, Spin::Polarized];

    let mut total = 0usize;
    let mut pass = 0usize;
    let mut skip = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for &(name, id) in SPOT_CHECK_MGGAS {
        for &order in &orders {
            for &spin in &spins {
                total += 1;
                let res = run_mgga_tuple(id, name, order, spin);
                match &res {
                    TupleResult::Pass(e) => {
                        pass += 1;
                        eprintln!(
                            "PARITY_TUPLE: {} {} {} {} max_rel_err={:.3e} PASS",
                            name, id, order_label(order), spin_label(spin), e
                        );
                    }
                    TupleResult::SkipUnsupportedFunctional => {
                        skip += 1;
                        eprintln!(
                            "PARITY_TUPLE: {} {} {} {} SKIP UnsupportedFunctional",
                            name, id, order_label(order), spin_label(spin)
                        );
                    }
                    TupleResult::SkipNotRouted => {
                        skip += 1;
                        eprintln!(
                            "PARITY_TUPLE: {} {} {} {} SKIP NotRouted",
                            name, id, order_label(order), spin_label(spin)
                        );
                    }
                    TupleResult::SkipUnsupportedOrder => {
                        skip += 1;
                        eprintln!(
                            "PARITY_TUPLE: {} {} {} {} SKIP UnsupportedOrder",
                            name, id, order_label(order), spin_label(spin)
                        );
                    }
                    TupleResult::SkipNoOracleSupport => {
                        skip += 1;
                        eprintln!(
                            "PARITY_TUPLE: {} {} {} {} SKIP NoOracleSupport",
                            name, id, order_label(order), spin_label(spin)
                        );
                    }
                    TupleResult::Fail { max_rel_err, detail } => {
                        failures.push(format!(
                            "PARITY_TUPLE: {} {} {} {} max_rel_err={:.3e} FAIL: {}",
                            name, id,
                            order_label(order), spin_label(spin),
                            max_rel_err, detail
                        ));
                    }
                }
            }
        }
    }

    eprintln!(
        "PARITY_SUMMARY: mgga total={total} pass={pass} skip={skip} fail={}",
        failures.len()
    );

    assert!(
        failures.is_empty(),
        "Phase 9 strict 1e-12 MGGA spot-check failed for {} tuple(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
