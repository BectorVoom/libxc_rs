//! Phase 11 Plan 09 — G1 production τ-clamp parity for the `mgga_c_b94` canary.
//!
//! Sibling of `g3_mgga_c_b94_parity.rs`. Where G3 proved the kernel matches the
//! libxc id=397 oracle on the matched grids, **G1 specifically targets the
//! von Weizsäcker regularization** that gap G-1 added to the PRODUCTION MGGA
//! path (`src/eval/mgga_dispatch/prepare.rs::tau_von_weizsacker`).
//!
//! The point: libxc's MGGA work-driver clamps τ to its von Weizsäcker lower
//! bound τ_W = σ/(8ρ) *before* evaluating the functional. Rust production now
//! does the same in `dispatch_mgga`. This test exercises a grid where τ < τ_W
//! (the clamp is ACTIVE) and proves:
//!   1. applying the SAME clamp production uses brings `zk` to parity with the
//!      libxc id=397 oracle at strict 1e-12 (the oracle gets RAW τ and clamps
//!      internally; we get CLAMPED τ — they must agree), and
//!   2. the clamp was actually active (`τ_clamped[i] > τ_raw[i]` on the sub-vW
//!      points), so the test cannot pass vacuously / would diverge against raw τ.
//!
//! Building this crate compiles ONE kernel (`libxc-kernel-mgga_c_b94`) plus
//! `cubecl` and `libxc-sys` — never the 281-kernel umbrella.

#![allow(non_snake_case)]

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::prelude::*;

use libxc_kernel_mgga_c_b94::exc_unpol::mgga_c_b94_exc_unpol;

// ---- libxc default ext_params for mgga_c_b94 (mgga_c_b94.c:25) -------------
const PARAM_GAMMA: f64 = 1.0;
const PARAM_CSS: f64 = 0.88;
const PARAM_CAB: f64 = 0.63;

// Thresholds::default() from src/model/mod.rs — the values dispatch passes.
const DENS_THRESHOLD: f64 = 1e-15;
const ZETA_THRESHOLD: f64 = 1e-10;

// Sub-vW grid. τ_W = σ/(8ρ); each point annotated active (clamp raises τ) or
// no-op (τ already ≥ τ_W). At least 2 active + 1 no-op per plan 11-09 Task 3.
//   i=0: τ_W = 2.0/(8·1.0) = 0.25  > τ=0.01  → ACTIVE  (clamp → 0.25)
//   i=1: τ_W = 1.0/(8·0.5) = 0.25  > τ=0.02  → ACTIVE  (clamp → 0.25)
//   i=2: τ_W = 0.1/(8·1.0) = 0.0125 < τ=1.0  → NO-OP   (clamp → 1.0)
const RHO: &[f64] = &[1.0, 0.5, 1.0];
const SIGMA: &[f64] = &[2.0, 1.0, 0.1];
const LAPL: &[f64] = &[0.05, 0.02, 0.05];
const TAU: &[f64] = &[0.01, 0.02, 1.0];

const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;

// XC_MGGA_C_B94 = 397 (libxc-master/src/xc_funcs.h:336).
const MGGA_C_B94_ID: i32 = 397;
const XC_UNPOLARIZED: i32 = 1;

/// Thin launchable wrapper around the unrouted (plain `#[cube]`) kernel —
/// copied verbatim from `g3_mgga_c_b94_parity.rs` (concrete-f64, no turbofish).
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

/// von Weizsäcker lower bound τ ≥ σ/(8ρ).
///
/// BYTE-IDENTICAL to the PRODUCTION clamp in
/// `src/eval/mgga_dispatch/prepare.rs::tau_von_weizsacker` — same raise-only
/// `tau[i].max(sigma[i] / (8.0 * rho[i]))` and the same `ρ < dens_threshold`
/// guard. This test exercises the exact formula production uses (plan 11-09
/// invariant 3); if the two ever diverge this parity check is meaningless.
fn tau_von_weizsacker(rho: &[f64], sigma: &[f64], tau: &[f64], dens_threshold: f64) -> Vec<f64> {
    (0..tau.len())
        .map(|i| {
            if rho[i] < dens_threshold {
                tau[i]
            } else {
                tau[i].max(sigma[i] / (8.0 * rho[i]))
            }
        })
        .collect()
}

/// Evaluate `mgga_c_b94_exc_unpol` on the CLAMPED grid via direct CubeCL launch.
fn rust_b94_zk(tau_clamped: &[f64]) -> Vec<f64> {
    let np = RHO.len();
    let client = CpuRuntime::client(&CpuDevice);

    let rho_h = client.create_from_slice(bytemuck::cast_slice(RHO));
    let sigma_h = client.create_from_slice(bytemuck::cast_slice(SIGMA));
    let lapl_h = client.create_from_slice(bytemuck::cast_slice(LAPL));
    let tau_h = client.create_from_slice(bytemuck::cast_slice(tau_clamped));
    let zeros = vec![0.0f64; np];
    let zk_h = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let zk_read = zk_h.clone();

    let cube_dim = CubeDim::new_1d(256);
    let cube_count = CubeCount::new_1d((np as u32).div_ceil(256));

    // cubecl 0.10 launch ABI (see g3 sibling: from_raw_parts MOVES the handle,
    // no turbofish/vectorization arg; scalars pass bare).
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

/// Evaluate libxc id=397 exc, unpolarized, on the RAW grid (libxc's work-driver
/// applies its own von Weizsäcker clamp internally).
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
fn g1_tau_clamp_dispatch_parity_1e12() {
    let np = RHO.len();

    // Apply the production clamp, then assert it was ACTIVE on the sub-vW points
    // — guarantees the test is not vacuous (would diverge against raw τ).
    let tau_clamped = tau_von_weizsacker(RHO, SIGMA, TAU, DENS_THRESHOLD);
    assert!(
        tau_clamped[0] > TAU[0] && tau_clamped[1] > TAU[1],
        "clamp not active on sub-vW points: raw={TAU:?} clamped={tau_clamped:?} \
         — test would be vacuous"
    );
    assert!(
        (tau_clamped[2] - TAU[2]).abs() < 1e-300,
        "clamp must be a no-op on the τ ≥ τ_W point: raw={} clamped={}",
        TAU[2],
        tau_clamped[2]
    );
    eprintln!(
        "G1_CLAMP: raw_tau={TAU:?} clamped_tau={tau_clamped:?} (i0,i1 active; i2 no-op)"
    );

    let rust = rust_b94_zk(&tau_clamped);
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
            "G1_POINT: mgga_c_b94 exc unpol zk[{i}] rust={:.15e} c={:.15e} rel_err={:.3e}",
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
        "G1_SUMMARY: mgga_c_b94 exc unpol (sub-vW grid) np={np} max_rel_err={max_e:.3e} \
         tol={STRICT_TOL:.0e} verdict={}",
        if failures.is_empty() { "PASS" } else { "FAIL" }
    );

    assert!(
        failures.is_empty(),
        "G1 production τ-clamp parity FAILED at strict 1e-12 ({} point(s)):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
