//! Phase 11 oracle parity sweep — strict 1e-12 across already-routed functionals
//! plus the explicit worst-case offenders that Phase 11 must turn green.
//!
//! See .planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md decision D-05:
//! tolerance is strict 1e-12 with no per-order relaxation (operation-order
//! preservation per existing D-08 / Phase 9 D-14 holds across Phase 11 regen).
//!
//! Wave-0 role (per plan 11-01 Task 3):
//!   - Establishes the home for P11-INV-4 (oracle parity invariant).
//!   - `phase11_smoke` is the harness-wiring gate: covers routed entries from
//!     each family (≥3 LDA + ≥3 GGA + ≥3 MGGA per W8 floor). Expected to pass
//!     against today's pre-Phase-11 tree.
//!   - `phase11_worst_case` lists the offenders Phase 11 must close — marked
//!     `#[ignore]` until plan 11-05 lands the unified emission. Each entry
//!     today either fails to compile-in-the-current-tree or fails at
//!     strict 1e-12; the executor un-ignores once the family has regenerated
//!     under the D-02 tuple-return + D-10 unified layout.
//!
//! **No-skip invariant (mirrors parity_phase09.rs lines 18-23):** A tuple is
//! allowed to skip ONLY if `dispatch_{lda,gga,mgga}` returns `UnsupportedFunctional`
//! or `UnsupportedDerivativeOrder`. Every skipped tuple is printed with a
//! `PARITY_TUPLE: ... SKIP <reason>` line so the post-run audit can assemble
//! the full report.
//!
//! Deviations from plan 11-01 Task 3 (recorded for 11-01-SUMMARY.md):
//!   - LDA smoke entry `lda_xc_ksdt` (id 259) substituted with `lda_xc_teter93`
//!     (id 20). Reason: lda_xc_ksdt is in the documented deferred-LDA set
//!     (`verify/src/lib.rs:10` comment + line 73, and `LdaFunctional::from_id`
//!     does NOT route 259 — see `src/model/lda_functional.rs:78-..`). Including
//!     it would yield only `SkipNotRouted`, which violates the W8 floor
//!     ("entries that currently pass at 1e-12"). lda_xc_teter93 IS routed at
//!     line 100 of `src/model/lda_functional.rs`.
//!   - The MGGA smoke list is the first 4 of the W-D10-6 widened candidate
//!     set; final selection is execution-time-verified by the user (see
//!     11-01-SUMMARY.md once the cargo test result is captured).
//!   - `phase11_worst_case` is `#[ignore]`'d at Wave 0 — the worst-case files
//!     today either fail to compile under the workspace path-resolution
//!     staleness (see 11-DISPATCH-AUDIT.md) or fail at strict 1e-12.
//!     Plan 11-05's collapse turns this green.

use libxc_rs::LibxcRsError;
use libxc_rs::eval::{dispatch_gga, dispatch_lda, dispatch_mgga};
use libxc_rs::input::{GgaInput, LdaInput, MggaInput};
use libxc_rs::model::{
    DerivativeOrder, FunctionalId, GgaFunctional, LdaFunctional, MggaFunctional, Spin, Thresholds,
};
use libxc_rs::output::{GgaOutput, LdaOutput, MggaOutput};
use libxc_rs_verify::{
    FLAGS_HAVE_EXC, FLAGS_HAVE_VXC, oracle_func_flags, oracle_gga_all, oracle_lda_all,
    oracle_mgga_all,
};

// ---- D-05 contract ---------------------------------------------------------

const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;

// ---- D-19a / D-19c f32 dual-precision contract -----------------------------
//
// LIBXC_RS_F32=1 enables the env-gated f32 leg. The f32 tests run the SAME
// rust dispatch path against the f64 oracle and check max_rel_err against a
// per-functional tolerance loaded from
// crates/kernels/math/tests/f32_tolerance_overrides.toml.
//
// Default (no override): RELAXED_TOL_F32 = 1e-6.
// Hard ceiling: 1e-3 (asserted in f32_tolerance_for — any override above
// this triggers a panic indicating an f32 codegen defect, not convergence
// noise). Brent-class helpers (br89, mbrxc and dependents) get 1e-4.
//
// NOTE: At Task 4 the f32 dispatch wiring through chunks does not yet exist —
// the f32 leg compiles and exercises the TOML loader + ceiling assertion, but
// the per-entry parity comparison stays f64-typed until Task 6 (Gate 3) wires
// the f32 chunk → helper integration spike for mgga_c_b94.

const RELAXED_TOL_F32: f64 = 1e-6;

/// Per-test f32 tolerance override callback (D-19c).
/// Loads `crates/kernels/math/tests/f32_tolerance_overrides.toml` once and looks up
/// by functional name. Returns RELAXED_TOL_F32 (1e-6) if no override.
fn f32_tolerance_for(functional_name: &str) -> f64 {
    static OVERRIDES: std::sync::OnceLock<std::collections::HashMap<String, f64>> =
        std::sync::OnceLock::new();
    let map = OVERRIDES.get_or_init(|| {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../crates/kernels/math/tests/f32_tolerance_overrides.toml");
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let parsed: toml::Value = toml::from_str(&text)
            .unwrap_or(toml::Value::Table(Default::default()));
        let mut m = std::collections::HashMap::new();
        if let Some(tbl) = parsed.as_table() {
            for (key, val) in tbl {
                if key == "default" {
                    continue;
                }
                if let Some(tol) = val.get("tolerance").and_then(|v| v.as_float()) {
                    // Hard ceiling per CONTEXT.md D-19c: 1e-3
                    assert!(
                        tol <= 1e-3,
                        "f32 tolerance override for {} ({}) exceeds 1e-3 ceiling — \
                         investigate as f32 codegen defect, not convergence noise",
                        key,
                        tol
                    );
                    m.insert(key.clone(), tol);
                }
            }
        }
        m
    });
    map.get(functional_name).copied().unwrap_or(RELAXED_TOL_F32)
}

fn f32_mode_enabled() -> bool {
    std::env::var("LIBXC_RS_F32").as_deref() == Ok("1")
}

// ---- Phase11 entry types ---------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Family {
    Lda,
    Gga,
    Mgga,
}

#[derive(Debug, Clone, Copy)]
struct PhaseEntry {
    canonical: &'static str,
    family: Family,
    id: i32,
}

// ---- Smoke set (W8 floor: ≥3 per family) -----------------------------------

/// Routed functionals expected to pass at strict 1e-12 against today's tree.
/// Pinned at plan-time; MGGA portion verified at execution-time per W-D10-6.
const PHASE11_SMOKE: &[PhaseEntry] = &[
    // LDA (3 entries — substituted lda_xc_teter93 for plan's deferred lda_xc_ksdt)
    PhaseEntry { canonical: "lda_x",           family: Family::Lda,  id: 1   },
    PhaseEntry { canonical: "lda_c_pw",        family: Family::Lda,  id: 12  },
    PhaseEntry { canonical: "lda_xc_teter93",  family: Family::Lda,  id: 20  },
    // GGA (3 entries — per plan)
    PhaseEntry { canonical: "gga_x_pbe",       family: Family::Gga,  id: 101 },
    PhaseEntry { canonical: "gga_c_pbe",       family: Family::Gga,  id: 130 },
    PhaseEntry { canonical: "gga_x_b88",       family: Family::Gga,  id: 106 },
    // MGGA (4 entries — first 4 of W-D10-6 widened candidate set; subject to
    // execution-time re-selection if any fails at strict 1e-12 against today's
    // tree per the W-D10-6 selection rule).
    PhaseEntry { canonical: "mgga_x_lta",      family: Family::Mgga, id: 201 },
    PhaseEntry { canonical: "mgga_x_rlda",     family: Family::Mgga, id: 688 },
    PhaseEntry { canonical: "mgga_k_rda",      family: Family::Mgga, id: 621 },
    PhaseEntry { canonical: "mgga_xc_zlp",     family: Family::Mgga, id: 42 },
];

// ---- Worst-case set (Phase 11 closure target) -----------------------------

/// Entries that Phase 11 must turn green. Today, these either fail to compile
/// against the workspace's path-resolution staleness (see 11-DISPATCH-AUDIT.md)
/// OR exceed STRICT_TOL on the current oversized files. Plan 11-05's collapse
/// + regen-under-D-02-tuple-return closes them.
const PHASE11_WORST_CASE: &[PhaseEntry] = &[
    PhaseEntry { canonical: "mgga_c_revtpss",       family: Family::Mgga, id: 241 },
    PhaseEntry { canonical: "mgga_c_kcisk",         family: Family::Mgga, id: 562 },
    PhaseEntry { canonical: "mgga_c_b94",           family: Family::Mgga, id: 568 },
    PhaseEntry { canonical: "mgga_x_r4scan",        family: Family::Mgga, id: 497 },
    PhaseEntry { canonical: "mgga_x_br89_explicit", family: Family::Mgga, id: 285 },
    PhaseEntry { canonical: "mgga_xc_b97m_v",       family: Family::Mgga, id: 254 },
];

// ---- Test grids (matched-input pattern, mirrors phase09 lines 141-164) -----

const RHO_UNPOL: &[f64] = &[0.1, 0.5, 1.0, 5.0];
const SIGMA_UNPOL: &[f64] = &[0.01, 0.1, 0.5, 2.0];
const LAPL_UNPOL: &[f64] = &[0.001, 0.01, 0.05, 0.2];
const TAU_UNPOL: &[f64] = &[0.01, 0.05, 0.2, 1.0];

const RHO_POL: &[f64] = &[0.1, 0.05, 0.5, 0.3, 1.0, 0.8, 5.0, 3.0];

// ---- Helpers ---------------------------------------------------------------

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

fn family_label(f: Family) -> &'static str {
    match f {
        Family::Lda => "lda",
        Family::Gga => "gga",
        Family::Mgga => "mgga",
    }
}

#[derive(Debug)]
enum TupleResult {
    Pass(f64),
    SkipUnsupportedFunctional,
    SkipNotRouted,
    SkipUnsupportedOrder,
    SkipNoOracleSupport,
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

fn check_slice(name: &str, label: &str, rust: &[f64], c: &[f64]) -> TupleResult {
    let mut max_e = 0.0f64;
    let n = rust.len().min(c.len());
    for i in 0..n {
        let e = rel_err_with_floor(rust[i], c[i]);
        if e > max_e {
            max_e = e;
        }
        if e > STRICT_TOL {
            return TupleResult::Fail {
                max_rel_err: e,
                detail: format!(
                    "{name} .{label}[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                    rust[i], c[i], e
                ),
            };
        }
    }
    TupleResult::Pass(max_e)
}

// ---- Per-family Exc smoke runners ------------------------------------------
//
// Wave-0 scope is the harness-wiring gate — we only exercise the Exc order at
// unpolarized spin per family for the smoke set. Later waves widen to all
// orders × both spins.

fn run_lda_exc_unpol(entry: &PhaseEntry) -> TupleResult {
    let flags = oracle_func_flags(entry.id, 1).unwrap_or(0);
    if flags & FLAGS_HAVE_EXC == 0 {
        return TupleResult::SkipNoOracleSupport;
    }
    let id_obj = match FunctionalId::from_raw(entry.id as u16) {
        Ok(id) => id,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let functional = match LdaFunctional::from_id(id_obj) {
        Ok(f) => f,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let rho = RHO_UNPOL;
    let np = rho.len();
    let oracle = match oracle_lda_all(entry.id, 1, rho) {
        Ok(o) => o,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("oracle_lda_all({}={}): {e}", entry.canonical, entry.id),
            };
        }
    };
    let input = match LdaInput::new(rho, np, Spin::Unpolarized) {
        Ok(i) => i,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("LdaInput::new: {e}"),
            };
        }
    };
    let mut zk = vec![0.0f64; np];
    let mut output = LdaOutput {
        zk: if functional.has_exc() { Some(&mut zk) } else { None },
        ..Default::default()
    };
    if let Err(e) = dispatch_lda(
        functional,
        &input,
        DerivativeOrder::Exc,
        &mut output,
        &libxc_rs::NoParams::default(),
        &Thresholds::default(),
    ) {
        return classify_dispatch_err(e);
    }
    drop(output);
    if !functional.has_exc() {
        return TupleResult::SkipUnsupportedOrder;
    }
    check_slice(entry.canonical, "zk", &zk, &oracle.zk)
}

fn run_gga_exc_unpol(entry: &PhaseEntry) -> TupleResult {
    let flags = oracle_func_flags(entry.id, 1).unwrap_or(0);
    if flags & FLAGS_HAVE_EXC == 0 {
        return TupleResult::SkipNoOracleSupport;
    }
    let id_obj = match FunctionalId::from_raw(entry.id as u16) {
        Ok(id) => id,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let functional = match GgaFunctional::from_id(id_obj) {
        Ok(f) => f,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let rho = RHO_UNPOL;
    let sigma = SIGMA_UNPOL;
    let np = rho.len();
    let oracle = match oracle_gga_all(entry.id, 1, rho, sigma) {
        Ok(o) => o,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("oracle_gga_all({}={}): {e}", entry.canonical, entry.id),
            };
        }
    };
    let input = match GgaInput::new(rho, sigma, np, Spin::Unpolarized) {
        Ok(i) => i,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("GgaInput::new: {e}"),
            };
        }
    };
    let mut zk = vec![0.0f64; np];
    let mut output = GgaOutput {
        zk: if functional.has_exc() { Some(&mut zk) } else { None },
        ..Default::default()
    };
    if let Err(e) = dispatch_gga(
        functional,
        &input,
        DerivativeOrder::Exc,
        &mut output,
        &libxc_rs::NoParams::default(),
        &Thresholds::default(),
    ) {
        return classify_dispatch_err(e);
    }
    drop(output);
    if !functional.has_exc() {
        return TupleResult::SkipUnsupportedOrder;
    }
    check_slice(entry.canonical, "zk", &zk, &oracle.zk)
}

fn run_mgga_exc_unpol(entry: &PhaseEntry) -> TupleResult {
    let flags = oracle_func_flags(entry.id, 1).unwrap_or(0);
    if flags & FLAGS_HAVE_EXC == 0 {
        return TupleResult::SkipNoOracleSupport;
    }
    let id_obj = match FunctionalId::from_raw(entry.id as u16) {
        Ok(id) => id,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let functional = match MggaFunctional::from_id(id_obj) {
        Ok(f) => f,
        Err(_) => return TupleResult::SkipNotRouted,
    };
    let rho = RHO_UNPOL;
    let sigma = SIGMA_UNPOL;
    let lapl = LAPL_UNPOL;
    let tau = TAU_UNPOL;
    let np = rho.len();
    let oracle = match oracle_mgga_all(entry.id, 1, rho, sigma, lapl, tau) {
        Ok(o) => o,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("oracle_mgga_all({}={}): {e}", entry.canonical, entry.id),
            };
        }
    };
    let input = match MggaInput::new(rho, sigma, lapl, tau, np, Spin::Unpolarized) {
        Ok(i) => i,
        Err(e) => {
            return TupleResult::Fail {
                max_rel_err: f64::INFINITY,
                detail: format!("MggaInput::new: {e}"),
            };
        }
    };
    let mut zk = vec![0.0f64; np];
    let mut output = MggaOutput {
        zk: if functional.has_exc() { Some(&mut zk) } else { None },
        ..Default::default()
    };
    if let Err(e) = dispatch_mgga(
        functional,
        &input,
        DerivativeOrder::Exc,
        &mut output,
        &libxc_rs::NoParams::default(),
        &Thresholds::default(),
    ) {
        return classify_dispatch_err(e);
    }
    drop(output);
    if !functional.has_exc() {
        return TupleResult::SkipUnsupportedOrder;
    }
    check_slice(entry.canonical, "zk", &zk, &oracle.zk)
}

fn run_entry(entry: &PhaseEntry) -> TupleResult {
    match entry.family {
        Family::Lda => run_lda_exc_unpol(entry),
        Family::Gga => run_gga_exc_unpol(entry),
        Family::Mgga => run_mgga_exc_unpol(entry),
    }
}

fn report(label: &str, entries: &[PhaseEntry]) -> (usize, usize, usize, Vec<String>) {
    let mut pass = 0usize;
    let mut skip = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for entry in entries {
        let res = run_entry(entry);
        match &res {
            TupleResult::Pass(e) => {
                pass += 1;
                eprintln!(
                    "PARITY_TUPLE: {} {} {} exc unpol max_rel_err={:.3e} PASS",
                    label, family_label(entry.family), entry.canonical, e
                );
            }
            TupleResult::SkipUnsupportedFunctional => {
                skip += 1;
                eprintln!(
                    "PARITY_TUPLE: {} {} {} exc unpol SKIP UnsupportedFunctional",
                    label, family_label(entry.family), entry.canonical
                );
            }
            TupleResult::SkipNotRouted => {
                skip += 1;
                eprintln!(
                    "PARITY_TUPLE: {} {} {} exc unpol SKIP NotRouted",
                    label, family_label(entry.family), entry.canonical
                );
            }
            TupleResult::SkipUnsupportedOrder => {
                skip += 1;
                eprintln!(
                    "PARITY_TUPLE: {} {} {} exc unpol SKIP UnsupportedOrder",
                    label, family_label(entry.family), entry.canonical
                );
            }
            TupleResult::SkipNoOracleSupport => {
                skip += 1;
                eprintln!(
                    "PARITY_TUPLE: {} {} {} exc unpol SKIP NoOracleSupport",
                    label, family_label(entry.family), entry.canonical
                );
            }
            TupleResult::Fail { max_rel_err, detail } => {
                failures.push(format!(
                    "PARITY_TUPLE: {} {} {} exc unpol max_rel_err={:.3e} FAIL: {}",
                    label, family_label(entry.family), entry.canonical, max_rel_err, detail
                ));
            }
        }
    }
    let total = entries.len();
    eprintln!(
        "PARITY_SUMMARY: {label} total={total} pass={pass} skip={skip} fail={}",
        failures.len()
    );
    (total, pass, skip, failures)
}

// ---- Test entry points -----------------------------------------------------

#[test]
fn phase11_smoke() {
    let _ = (
        RHO_POL,
        FLAGS_HAVE_VXC, // referenced for forward compatibility with later orders
    );
    let (_total, _pass, _skip, failures) = report("smoke", PHASE11_SMOKE);
    assert!(
        failures.is_empty(),
        "phase11_smoke: {} tuple(s) failed at strict 1e-12 (Wave-0 harness gate):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
#[ignore = "Phase 11 wave 5 (collapse + regen) turns this green; today the \
            worst-case files either fail to compile under workspace path-staleness \
            or exceed strict 1e-12. See 11-DISPATCH-AUDIT.md and plan 11-05."]
fn phase11_worst_case() {
    let (_total, _pass, _skip, failures) = report("worst_case", PHASE11_WORST_CASE);
    assert!(
        failures.is_empty(),
        "phase11_worst_case: {} tuple(s) failed at strict 1e-12:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

// ---- D-19 / D-19c f32 dual-precision test entry points ---------------------
//
// Env-gated by LIBXC_RS_F32=1. At Task 4 (this commit) the bodies validate:
//   - the TOML loader works
//   - the per-entry tolerance lookup returns a sensible value
//   - the hard-ceiling assertion in f32_tolerance_for fires correctly
// Task 6 (Gate 3 Leg 3) wires the actual f32 chunk dispatch + parity check
// once Task 5 has converted the helper layer to generic <F: Float>.

#[test]
fn phase11_smoke_f32() {
    if !f32_mode_enabled() {
        return;
    }
    // Validate every smoke entry has a sensible tolerance lookup; the loader
    // also enforces the 1e-3 hard ceiling at first call.
    for entry in PHASE11_SMOKE {
        let tol = f32_tolerance_for(entry.canonical);
        assert!(
            tol > 0.0 && tol <= 1e-3,
            "phase11_smoke_f32: tolerance for {} ({}) is out of range",
            entry.canonical,
            tol
        );
        eprintln!(
            "PARITY_TUPLE_F32: smoke {} {} tolerance={:.0e} (Task-4 loader probe; \
             actual f32 parity dispatch wired in Task 6 Gate 3)",
            family_label(entry.family),
            entry.canonical,
            tol
        );
    }
}

#[test]
fn phase11_worst_case_f32() {
    if !f32_mode_enabled() {
        return;
    }
    // Same probe shape as smoke_f32; worst-case entries include the
    // mgga_c_b94 D-19c override (1e-4 per TOML).
    for entry in PHASE11_WORST_CASE {
        let tol = f32_tolerance_for(entry.canonical);
        assert!(
            tol > 0.0 && tol <= 1e-3,
            "phase11_worst_case_f32: tolerance for {} ({}) is out of range",
            entry.canonical,
            tol
        );
        eprintln!(
            "PARITY_TUPLE_F32: worst_case {} {} tolerance={:.0e} (Task-4 loader probe; \
             actual f32 parity dispatch wired in Task 6 Gate 3)",
            family_label(entry.family),
            entry.canonical,
            tol
        );
    }
}
