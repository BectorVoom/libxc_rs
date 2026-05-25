//! Phase 12 Plan 04 — non-NEEDS_TAU regression guard for `mgga_k_gea2` (libxc id 627).
//!
//! This canary exists because the 12-04 family-oracle gate caught a D-01 regression:
//! the original `regularize_inputs` applied the sigma-DOWN Fermi-hole clamp
//! UNCONDITIONALLY, but libxc gates it on `XC_FLAGS_NEEDS_TAU` (work_mgga_inc.c:62).
//! `mgga_k_gea2` is a kinetic functional carrying `XC_FLAGS_NEEDS_LAPLACIAN` but NOT
//! `XC_FLAGS_NEEDS_TAU` — it reads sigma + laplacian, never tau. Clamping its sigma down
//! fed it a value libxc never produces, breaking parity (rel_err ~4.4e-3).
//!
//! The fix gates the tau-floor + sigma-DOWN clamp on `needs_tau`. This canary mirrors the
//! production `regularize_inputs` with `needs_tau = false`: the clamp must be INACTIVE
//! (sigma unchanged) on a point where it WOULD fire if wrongly applied — and the kernel
//! must then match the libxc oracle at 1e-12. Builds ONE kernel — never the umbrella.

#![allow(non_snake_case)]

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::prelude::*;

use libxc_kernel_mgga_k_gea2::exc_unpol::mgga_k_gea2_exc_unpol;

const DENS_THRESHOLD: f64 = 1e-15;
const ZETA_THRESHOLD: f64 = 1e-10;

// Same 5-point grid as the NEEDS_TAU canaries. i=4 is a sub-Fermi-hole point
// (8*rho*tau = 0.08 < sigma = 2.0): a WRONGLY-applied sigma-down clamp would
// lower sigma there. With needs_tau=false the clamp must be a no-op (sigma kept).
const RHO: &[f64] = &[0.1, 0.5, 1.0, 5.0, 1.0];
const SIGMA: &[f64] = &[0.01, 0.1, 0.5, 2.0, 2.0];
const LAPL: &[f64] = &[0.001, 0.01, 0.05, 0.2, 0.05];
const TAU: &[f64] = &[0.01, 0.05, 0.2, 1.0, 0.01];

const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;

// XC_MGGA_K_GEA2 = 627; mgga_k_gea2 has NEEDS_LAPLACIAN, NOT NEEDS_TAU.
const FUNC_ID: i32 = 627;
const XC_UNPOLARIZED: i32 = 1;
const NEEDS_TAU: bool = false;

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

/// Mirror the production D-01 regularization
/// (`src/eval/mgga_dispatch/prepare.rs::regularize_inputs`) INCLUDING the `needs_tau`
/// gate: the tau-floor + sigma-DOWN clamp apply ONLY when `needs_tau` is true. For a
/// non-NEEDS_TAU functional, sigma keeps only its (no-op) floor and tau is raw.
fn regularize_inputs(rho: &[f64], sigma: &[f64], tau: &[f64], needs_tau: bool) -> (Vec<f64>, Vec<f64>) {
    let dens_threshold = DENS_THRESHOLD;
    let tau_threshold = 1e-20_f64;
    let sigma_threshold = dens_threshold.powf(4.0 / 3.0);
    let sigma_floor = sigma_threshold * sigma_threshold;
    let mut sigma_out = Vec::with_capacity(sigma.len());
    let mut tau_out = Vec::with_capacity(tau.len());
    for i in 0..rho.len() {
        if rho[i] < dens_threshold {
            sigma_out.push(sigma[i]);
            tau_out.push(tau[i]);
            continue;
        }
        let my_sigma = sigma_floor.max(sigma[i]);
        let (my_sigma, my_tau) = if needs_tau {
            let my_rho = dens_threshold.max(rho[i]);
            let my_tau = tau_threshold.max(tau[i]);
            (my_sigma.min(8.0 * my_rho * my_tau), my_tau)
        } else {
            (my_sigma, tau[i])
        };
        sigma_out.push(my_sigma);
        tau_out.push(my_tau);
    }
    (sigma_out, tau_out)
}

/// Evaluate `mgga_k_gea2_exc_unpol` on the (non-clamped) regularized grid via direct launch.
fn rust_zk() -> Vec<f64> {
    let np = RHO.len();
    let client = CpuRuntime::client(&CpuDevice);

    let (sigma_reg, tau_reg) = regularize_inputs(RHO, SIGMA, TAU, NEEDS_TAU);
    let rho_h = client.create_from_slice(bytemuck::cast_slice(RHO));
    let sigma_h = client.create_from_slice(bytemuck::cast_slice(&sigma_reg));
    let lapl_h = client.create_from_slice(bytemuck::cast_slice(LAPL));
    let tau_h = client.create_from_slice(bytemuck::cast_slice(&tau_reg));
    let zeros = vec![0.0f64; np];
    let zk_h = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let zk_read = zk_h.clone();

    let cube_dim = CubeDim::new_1d(256);
    let cube_count = CubeCount::new_1d((np as u32).div_ceil(256));

    unsafe {
        mgga_k_gea2_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts(rho_h, np),
            ArrayArg::from_raw_parts(sigma_h, np),
            ArrayArg::from_raw_parts(lapl_h, np),
            ArrayArg::from_raw_parts(tau_h, np),
            ArrayArg::from_raw_parts(zk_h, np),
            DENS_THRESHOLD,
            ZETA_THRESHOLD,
        );
    }

    let bytes = client
        .read_one(zk_read)
        .expect("read_one(zk) failed post-launch");
    bytemuck::cast_slice(&bytes).to_vec()
}

/// Evaluate libxc id=627 exc, unpolarized, on the RAW grid.
fn oracle_zk() -> Vec<f64> {
    let np = RHO.len();
    let mut zk = vec![0.0f64; np];
    unsafe {
        let func = libxc_sys::xc_func_alloc();
        assert!(!func.is_null(), "xc_func_alloc returned null");
        let ret = libxc_sys::xc_func_init(func, FUNC_ID, XC_UNPOLARIZED);
        assert_eq!(ret, 0, "xc_func_init failed (code {ret}) for id={FUNC_ID}");
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
fn mgga_k_gea2_f64_parity_1e12() {
    let np = RHO.len();

    // Regression guard: with needs_tau=false the sigma-DOWN clamp must be a NO-OP even on
    // the sub-Fermi-hole point i=4 (where 8*rho*tau=0.08 < sigma=2.0 would fire it). If the
    // clamp were (wrongly) applied unconditionally, sigma_reg[4] would drop to 0.08 and the
    // kernel would diverge from the oracle — this asserts the gate keeps it raw.
    let (sigma_reg, _tau_reg) = regularize_inputs(RHO, SIGMA, TAU, NEEDS_TAU);
    assert!(
        (sigma_reg[4] - SIGMA[4]).abs() < 1e-300,
        "non-NEEDS_TAU functional must NOT have sigma clamped: raw={} reg={} \
         (a sigma-down clamp here is the D-01 regression)",
        SIGMA[4],
        sigma_reg[4]
    );

    let rust = rust_zk();
    let oracle = oracle_zk();

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
            "PARITY_POINT: mgga_k_gea2 exc unpol zk[{i}] rust={:.15e} c={:.15e} rel_err={:.3e}",
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
        "PARITY_SUMMARY: mgga_k_gea2 (non-NEEDS_TAU) exc unpol np={np} max_rel_err={max_e:.3e} \
         tol={STRICT_TOL:.0e} verdict={}",
        if failures.is_empty() { "PASS" } else { "FAIL" }
    );

    assert!(
        failures.is_empty(),
        "mgga_k_gea2 f64 parity FAILED at strict 1e-12 ({} point(s)):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
