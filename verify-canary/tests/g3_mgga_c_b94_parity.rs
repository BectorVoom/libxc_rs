//! Phase 11.1 Plan 03 — G3 f64 smoke parity for the `mgga_c_b94` canary.
//!
//! This is the REWRITTEN G3 gate (user-approved 2026-05-21). The original gate
//! (`verify/tests/parity_phase11.rs::phase11_worst_case`) routed mgga_c_b94
//! through `libxc_rs::eval::dispatch_mgga`, which:
//!   1. forced ALL 281 kernels to compile (umbrella crate dep), and
//!   2. silently SKIPPED mgga_c_b94 anyway — the functional is deferred and has
//!      NO dispatch arm, and the entry hard-coded id 568 (real libxc id is 397).
//!
//! This test instead launches the kernel DIRECTLY via a thin
//! `#[cube(launch_unchecked)]` wrapper (the kernel itself is emitted as plain
//! `#[cube]` because it is unrouted — translator is routing-aware per quick task
//! 260512-q01), and compares the energy density `zk` to the C libxc oracle for
//! id=397, unpolarized, at strict 1e-12 relative error.
//!
//! Building this crate compiles ONE kernel (`libxc-kernel-mgga_c_b94`) plus
//! `cubecl` and `libxc-sys` — seconds, not the multi-hour 281-kernel build.

#![allow(non_snake_case)]

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::prelude::*;

use libxc_kernel_mgga_c_b94::exc_unpol::mgga_c_b94_exc_unpol;

// ---- libxc default ext_params for mgga_c_b94 -------------------------------
// From libxc-master/src/mgga_c_b94.c:25 — par_b94[N_PAR] = {1.0, 0.88, 0.63}
// for names {_gamma, _css, _cab}. xc_func_init applies these defaults, so the
// oracle and the Rust kernel must use the SAME values for bit-faithful parity.
const PARAM_GAMMA: f64 = 1.0;
const PARAM_CSS: f64 = 0.88;
const PARAM_CAB: f64 = 0.63;

// Thresholds::default() from src/model/mod.rs:162 (the values dispatch passes).
const DENS_THRESHOLD: f64 = 1e-15;
const ZETA_THRESHOLD: f64 = 1e-10;

// Matched-input grids — identical to verify/tests/parity_phase11.rs:172-175.
const RHO: &[f64] = &[0.1, 0.5, 1.0, 5.0];
const SIGMA: &[f64] = &[0.01, 0.1, 0.5, 2.0];
const LAPL: &[f64] = &[0.001, 0.01, 0.05, 0.2];
const TAU: &[f64] = &[0.01, 0.05, 0.2, 1.0];

const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;

// libxc id for mgga_c_b94 — XC_MGGA_C_B94 = 397 (libxc-master/src/xc_funcs.h:336).
const MGGA_C_B94_ID: i32 = 397;
const XC_UNPOLARIZED: i32 = 1;

/// Thin launchable wrapper around the unrouted (plain `#[cube]`) kernel.
/// Delegates verbatim — `ABSOLUTE_POS` and the bounds check live inside
/// `mgga_c_b94_exc_unpol`. The kernel is concrete-f64 (not generic), so no
/// turbofish is required at the call site (cf. 11-PATTERN.md Rule 9/10).
#[cube(launch_unchecked)]
fn b94_exc_unpol_kernel(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_b94_exc_unpol(
        rho, sigma, lapl, tau, zk, param_cab, param_css, param_gamma, dens_threshold,
        zeta_threshold,
    );
}

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

/// libxc work-driver input preprocessing: enforce the von Weizsäcker lower bound
/// τ ≥ τ_W = σ/(8ρ) before evaluating the functional.
///
/// libxc's MGGA work driver regularizes the kinetic-energy density to its von
/// Weizsäcker minimum; the maple2c expression itself uses raw τ. The Rust
/// per-functional dispatch (src/eval/mgga_dispatch) does NOT yet replicate this
/// driver-side regularization — a SYSTEMIC gap affecting every MGGA functional
/// for inputs with τ < τ_W (see memory project_translator_missing_workmgga_tau_clamp;
/// hands back to re-opened Phase 11). Applied here so the G3 canary mirrors libxc
/// faithfully. Confirmed against the libxc id=397 oracle to ~5e-13 on 4 distinct
/// τ < τ_W points; a no-op on points already satisfying the bound.
fn tau_von_weizsacker(rho: &[f64], sigma: &[f64], tau: &[f64]) -> Vec<f64> {
    (0..tau.len())
        .map(|i| tau[i].max(sigma[i] / (8.0 * rho[i])))
        .collect()
}

/// Evaluate `mgga_c_b94_exc_unpol` on the matched grids via direct CubeCL launch.
fn rust_b94_zk() -> Vec<f64> {
    let np = RHO.len();
    let client = CpuRuntime::client(&CpuDevice);

    let tau_vw = tau_von_weizsacker(RHO, SIGMA, TAU);
    let rho_h = client.create_from_slice(bytemuck::cast_slice(RHO));
    let sigma_h = client.create_from_slice(bytemuck::cast_slice(SIGMA));
    let lapl_h = client.create_from_slice(bytemuck::cast_slice(LAPL));
    let tau_h = client.create_from_slice(bytemuck::cast_slice(&tau_vw));
    let zeros = vec![0.0f64; np];
    let zk_h = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let zk_read = zk_h.clone();

    let cube_dim = CubeDim::new_1d(256);
    let cube_count = CubeCount::new_1d((np as u32).div_ceil(256));

    // cubecl 0.10 launch ABI (per the passing math-crate spike, the current
    // correct pattern; src/kernel/launch.rs's 3-arg `::<f64>` form is stale):
    //   - ArrayArg::from_raw_parts(handle, len) MOVES the handle — no turbofish,
    //     no vectorization arg.
    //   - scalar params pass as bare values (`impl<T: ScalarArgSettings>
    //     LaunchArg for T` → RuntimeArg = T), NOT wrapped in ScalarArg.
    //   - launch_unchecked returns (); read_one returns Result<Bytes, _>.
    unsafe {
        b94_exc_unpol_kernel::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts(rho_h, np),
            ArrayArg::from_raw_parts(sigma_h, np),
            ArrayArg::from_raw_parts(lapl_h, np),
            ArrayArg::from_raw_parts(tau_h, np),
            ArrayArg::from_raw_parts(zk_h, np),
            PARAM_CAB,
            PARAM_CSS,
            PARAM_GAMMA,
            DENS_THRESHOLD,
            ZETA_THRESHOLD,
        );
    }

    let bytes = client
        .read_one(zk_read)
        .expect("read_one(zk) failed post-launch");
    bytemuck::cast_slice(&bytes).to_vec()
}

/// Evaluate libxc id=397 (mgga_c_b94) exc, unpolarized, via the C oracle.
fn oracle_b94_zk() -> Vec<f64> {
    let np = RHO.len();
    let mut zk = vec![0.0f64; np];
    unsafe {
        let func = libxc_sys::xc_func_alloc();
        assert!(!func.is_null(), "xc_func_alloc returned null");
        let ret = libxc_sys::xc_func_init(func, MGGA_C_B94_ID, XC_UNPOLARIZED);
        assert_eq!(ret, 0, "xc_func_init failed (code {ret}) for id={MGGA_C_B94_ID}");
        libxc_sys::xc_mgga_exc(
            func,
            np,
            RHO.as_ptr(),
            SIGMA.as_ptr(),
            LAPL.as_ptr(),
            TAU.as_ptr(),
            zk.as_mut_ptr(),
        );
        libxc_sys::xc_func_end(func);
        libxc_sys::xc_func_free(func);
    }
    zk
}

#[test]
fn g3_mgga_c_b94_f64_parity_1e12() {
    let np = RHO.len();
    let rust = rust_b94_zk();
    let oracle = oracle_b94_zk();

    assert_eq!(rust.len(), np, "rust zk length mismatch");
    assert_eq!(oracle.len(), np, "oracle zk length mismatch");

    let mut max_e = 0.0f64;
    let mut failures: Vec<String> = Vec::new();
    for i in 0..np {
        let e = rel_err_with_floor(rust[i], oracle[i]);
        if e > max_e {
            max_e = e;
        }
        eprintln!(
            "G3_TUPLE: mgga_c_b94 exc unpol zk[{i}] rust={:.15e} c={:.15e} rel_err={:.3e}",
            rust[i], oracle[i], e
        );
        if e > STRICT_TOL {
            failures.push(format!(
                "zk[{i}]: rust={:.15e} c={:.15e} rel_err={:.3e}",
                rust[i], oracle[i], e
            ));
        }
    }
    eprintln!(
        "G3_SUMMARY: mgga_c_b94 exc unpol np={np} max_rel_err={max_e:.3e} \
         tol={STRICT_TOL:.0e} verdict={}",
        if failures.is_empty() { "PASS" } else { "FAIL" }
    );

    assert!(
        failures.is_empty(),
        "G3 mgga_c_b94 f64 parity FAILED at strict 1e-12 ({} point(s)):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
