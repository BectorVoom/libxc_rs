//! Gated behind `cubecl-backend` (ADR 0001): this test drives the archived
//! CubeCL backend through `libxc_rs::kernel` / `libxc_rs::math`, which are only
//! re-exported when that feature is on. Run with:
//!   cargo test --features cubecl-backend
#![cfg(feature = "cubecl-backend")]

//! Oracle comparison tests for MGGA kernels.
//!
//! Compares translated Rust #[cube] kernels against libxc C oracle values
//! using the verify crate's oracle_mgga_all() function.
//!
//! IMPORTANT: These tests MUST run with --test-threads=1 because the CubeCL
//! CPU runtime has shared global state that causes buffer corruption when
//! multiple CubeCL kernels launch concurrently. Use:
//!   cargo test --test oracle_mgga -- --test-threads=1
//!
//! Functional IDs (from libxc xc_funcs.h):
//!   XC_MGGA_XC_LP90 = 564
//!   XC_MGGA_K_GEA2  = 627
//!   XC_MGGA_X_LTA   = 201
//!   XC_MGGA_C_B88   = 571
//!
//! Sub-crate locations (after rebatch):
//!   mgga_xc_lp90 -> kernel-mgga-29
//!   mgga_k_gea2  -> kernel-mgga-17
//!   mgga_x_lta   -> kernel-mgga-34
//!   mgga_c_b88   -> kernel-mgga-26

use std::sync::Mutex;

use cubecl::cpu::CpuRuntime;
use cubecl::prelude::*;

use libxc_rs::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer, create_zero_output_buffer,
    read_output_buffer,
};

/// Global lock to serialize CubeCL kernel launches. The CubeCL CPU runtime
/// shares internal state (ComputeClient, device memory) that is NOT safe
/// for concurrent kernel launches from different test threads.
static CUBECL_LOCK: Mutex<()> = Mutex::new(());

// Functional IDs
const XC_MGGA_XC_LP90: i32 = 564;
const XC_MGGA_K_GEA2: i32 = 627;
const XC_MGGA_X_LTA: i32 = 201;
const XC_MGGA_C_B88: i32 = 571;

// Thresholds matching libxc defaults
const DENS_THRESHOLD: f64 = 1e-20;
const ZETA_THRESHOLD: f64 = 1e-10;

/// Relative error helper with absolute floor for near-zero values.
fn rel_err(rust_val: f64, oracle_val: f64) -> f64 {
    if oracle_val.abs() < 1e-300 {
        rust_val.abs()
    } else {
        ((rust_val - oracle_val) / oracle_val).abs()
    }
}

/// Compare two f64 slices element-by-element at 1e-12 relative tolerance.
/// Skips elements where both values are zero (avoids 0/0 relative error).
fn assert_vxc_match(label: &str, rust_vals: &[f64], oracle_vals: &[f64], tol: f64) {
    assert_eq!(
        rust_vals.len(),
        oracle_vals.len(),
        "{}: length mismatch rust={} oracle={}",
        label,
        rust_vals.len(),
        oracle_vals.len()
    );
    for i in 0..rust_vals.len() {
        // Skip comparison when both are essentially zero
        if oracle_vals[i].abs() < 1e-300 && rust_vals[i].abs() < 1e-300 {
            continue;
        }
        let err = rel_err(rust_vals[i], oracle_vals[i]);
        eprintln!(
            "  {label}[{i}]: rust={:.15e} oracle={:.15e} rel_err={:.2e}",
            rust_vals[i], oracle_vals[i], err
        );
        assert!(
            err < tol,
            "{} point {}: rust={}, oracle={}, rel_err={}",
            label,
            i,
            rust_vals[i],
            oracle_vals[i],
            err
        );
    }
}

// =============================================================================
// mgga_xc_lp90 EXC tests (ID 564, no ext_params)
// =============================================================================

#[test]
fn test_mgga_xc_lp90_exc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    // Get oracle values from libxc
    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_XC_LP90, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_xc_lp90 unpol");

    // Launch Rust kernel via CubeCL
    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_29::mgga_xc_lp90::exc_unpol::mgga_xc_lp90_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_h, np);

    eprintln!("mgga_xc_lp90 exc unpol:");
    for i in 0..np {
        let err = rel_err(rust_zk[i], oracle.zk[i]);
        eprintln!(
            "  rho={:.1} sigma={:.2} lapl={:.3} tau={:.1}: rust={:.15e} oracle={:.15e} rel_err={:.2e}",
            rho[i], sigma[i], lapl[i], tau[i], rust_zk[i], oracle.zk[i], err
        );
        assert!(
            err < 1e-12,
            "mgga_xc_lp90 exc unpol point {}: rust={}, oracle={}, rel_err={}",
            i, rust_zk[i], oracle.zk[i], err
        );
    }
}

#[test]
fn test_mgga_xc_lp90_exc_pol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 3;
    // Polarized: rho has 2*np entries [rho_up, rho_dn, ...]
    let rho = vec![0.05, 0.05, 0.3, 0.2, 1.0, 1.0];
    // sigma has 3*np entries [sigma_uu, sigma_ud, sigma_dd, ...]
    let sigma = vec![0.01, 0.005, 0.01, 0.1, 0.05, 0.1, 0.5, 0.2, 0.5];
    // lapl has 2*np entries
    let lapl = vec![0.001, 0.001, 0.01, 0.01, 0.05, 0.05];
    // tau has 2*np entries
    let tau = vec![0.05, 0.05, 0.2, 0.1, 0.5, 0.5];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_XC_LP90, 2, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_xc_lp90 pol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_29::mgga_xc_lp90::exc_pol::mgga_xc_lp90_exc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np * 3, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_h, np);

    eprintln!("mgga_xc_lp90 exc pol:");
    for i in 0..np {
        let err = rel_err(rust_zk[i], oracle.zk[i]);
        eprintln!(
            "  point {}: rust={:.15e} oracle={:.15e} rel_err={:.2e}",
            i, rust_zk[i], oracle.zk[i], err
        );
        assert!(
            err < 1e-12,
            "mgga_xc_lp90 exc pol point {}: rust={}, oracle={}, rel_err={}",
            i, rust_zk[i], oracle.zk[i], err
        );
    }
}

// =============================================================================
// mgga_k_gea2 EXC tests (ID 627, kinetic functional, no ext_params)
// =============================================================================

#[test]
fn test_mgga_k_gea2_exc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_K_GEA2, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_k_gea2 unpol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_17::mgga_k_gea2::exc_unpol::mgga_k_gea2_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_h, np);

    eprintln!("mgga_k_gea2 exc unpol:");
    for i in 0..np {
        let err = rel_err(rust_zk[i], oracle.zk[i]);
        eprintln!(
            "  rho={:.1}: rust={:.15e} oracle={:.15e} rel_err={:.2e}",
            rho[i], rust_zk[i], oracle.zk[i], err
        );
        assert!(
            err < 1e-12,
            "mgga_k_gea2 exc unpol point {}: rust={}, oracle={}, rel_err={}",
            i, rust_zk[i], oracle.zk[i], err
        );
    }
}

// =============================================================================
// mgga_x_lta EXC tests (ID 201, exchange, has ext_param ltafrac=1.0)
// =============================================================================

#[test]
fn test_mgga_x_lta_exc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_X_LTA, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_x_lta unpol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    // mgga_x_lta has ext_param ltafrac with default value 1.0
    let param_ltafrac: f64 = 1.0;

    unsafe {
        libxc_kernel_mgga_34::mgga_x_lta::exc_unpol::mgga_x_lta_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(param_ltafrac),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_h, np);

    eprintln!("mgga_x_lta exc unpol:");
    for i in 0..np {
        let err = rel_err(rust_zk[i], oracle.zk[i]);
        eprintln!(
            "  rho={:.1}: rust={:.15e} oracle={:.15e} rel_err={:.2e}",
            rho[i], rust_zk[i], oracle.zk[i], err
        );
        assert!(
            err < 1e-12,
            "mgga_x_lta exc unpol point {}: rust={}, oracle={}, rel_err={}",
            i, rust_zk[i], oracle.zk[i], err
        );
    }
}

// =============================================================================
// mgga_c_b88 EXC tests (ID 571, correlation, no ext_params)
// =============================================================================

#[test]
fn test_mgga_c_b88_exc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_C_B88, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_c_b88 unpol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_26::mgga_c_b88::exc_unpol::mgga_c_b88_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_h, np);

    eprintln!("mgga_c_b88 exc unpol:");
    for i in 0..np {
        let err = rel_err(rust_zk[i], oracle.zk[i]);
        eprintln!(
            "  rho={:.1}: rust={:.15e} oracle={:.15e} rel_err={:.2e}",
            rho[i], rust_zk[i], oracle.zk[i], err
        );
        assert!(
            err < 1e-12,
            "mgga_c_b88 exc unpol point {}: rust={}, oracle={}, rel_err={}",
            i, rust_zk[i], oracle.zk[i], err
        );
    }
}

// =============================================================================
// VXC tests: verify first-order derivatives (vrho, vsigma, vlapl, vtau)
// VERIFY-04: vrho relative error <= 1e-12
// =============================================================================

// =============================================================================
// mgga_xc_lp90 VXC tests (ID 564)
// =============================================================================

#[test]
fn test_mgga_xc_lp90_vxc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_XC_LP90, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_xc_lp90 vxc unpol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let vrho_h = create_zero_output_buffer(&client, np);
    let vsigma_h = create_zero_output_buffer(&client, np);
    let vlapl_h = create_zero_output_buffer(&client, np);
    let vtau_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_29::mgga_xc_lp90::vxc_unpol::mgga_xc_lp90_vxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vsigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vlapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vtau_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_vrho = read_output_buffer(&client, vrho_h, np);
    let rust_vsigma = read_output_buffer(&client, vsigma_h, np);
    let rust_vlapl = read_output_buffer(&client, vlapl_h, np);
    let rust_vtau = read_output_buffer(&client, vtau_h, np);

    eprintln!("mgga_xc_lp90 vxc unpol:");
    assert_vxc_match("vrho", &rust_vrho, &oracle.vrho, 1e-12);
    assert_vxc_match("vsigma", &rust_vsigma, &oracle.vsigma, 1e-12);
    assert_vxc_match("vlapl", &rust_vlapl, &oracle.vlapl, 1e-12);
    assert_vxc_match("vtau", &rust_vtau, &oracle.vtau, 1e-12);
}

#[test]
fn test_mgga_xc_lp90_vxc_pol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    // Polarized inputs: rho has 2*np entries (interleaved up/dn)
    let rho = vec![
        0.05, 0.05, 0.25, 0.25, 0.5, 0.5, 1.0, 1.0, 2.5, 2.5,
    ];
    // sigma has 3*np entries (sigma_uu, sigma_ud, sigma_dd per point)
    let sigma = vec![
        0.005, 0.002, 0.005, 0.05, 0.01, 0.05, 0.1, 0.05, 0.1, 0.5, 0.2, 0.5, 1.0, 0.5, 1.0,
    ];
    // lapl has 2*np entries
    let lapl = vec![
        0.0, 0.0, 0.001, 0.001, 0.005, 0.005, 0.01, 0.01, 0.05, 0.05,
    ];
    // tau has 2*np entries
    let tau = vec![
        0.05, 0.05, 0.15, 0.15, 0.3, 0.3, 0.6, 0.6, 1.5, 1.5,
    ];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_XC_LP90, 2, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_xc_lp90 vxc pol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let vrho_h = create_zero_output_buffer(&client, np * 2);
    let vsigma_h = create_zero_output_buffer(&client, np * 3);
    let vlapl_h = create_zero_output_buffer(&client, np * 2);
    let vtau_h = create_zero_output_buffer(&client, np * 2);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_29::mgga_xc_lp90::vxc_pol::mgga_xc_lp90_vxc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np * 3, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&vsigma_h, np * 3, 1),
            ArrayArg::from_raw_parts::<f64>(&vlapl_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&vtau_h, np * 2, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_vrho = read_output_buffer(&client, vrho_h, np * 2);
    let rust_vsigma = read_output_buffer(&client, vsigma_h, np * 3);
    let rust_vlapl = read_output_buffer(&client, vlapl_h, np * 2);
    let rust_vtau = read_output_buffer(&client, vtau_h, np * 2);

    eprintln!("mgga_xc_lp90 vxc pol:");
    assert_vxc_match("vrho", &rust_vrho, &oracle.vrho, 1e-12);
    assert_vxc_match("vsigma", &rust_vsigma, &oracle.vsigma, 1e-12);
    assert_vxc_match("vlapl", &rust_vlapl, &oracle.vlapl, 1e-12);
    assert_vxc_match("vtau", &rust_vtau, &oracle.vtau, 1e-12);
}

// =============================================================================
// mgga_k_gea2 VXC tests (ID 627, kinetic functional)
// =============================================================================

#[test]
fn test_mgga_k_gea2_vxc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_K_GEA2, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_k_gea2 vxc unpol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let vrho_h = create_zero_output_buffer(&client, np);
    let vsigma_h = create_zero_output_buffer(&client, np);
    let vlapl_h = create_zero_output_buffer(&client, np);
    let vtau_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_17::mgga_k_gea2::vxc_unpol::mgga_k_gea2_vxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vsigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vlapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vtau_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_vrho = read_output_buffer(&client, vrho_h, np);
    let rust_vsigma = read_output_buffer(&client, vsigma_h, np);
    let rust_vlapl = read_output_buffer(&client, vlapl_h, np);
    let rust_vtau = read_output_buffer(&client, vtau_h, np);

    eprintln!("mgga_k_gea2 vxc unpol:");
    assert_vxc_match("vrho", &rust_vrho, &oracle.vrho, 1e-12);
    assert_vxc_match("vsigma", &rust_vsigma, &oracle.vsigma, 1e-12);
    assert_vxc_match("vlapl", &rust_vlapl, &oracle.vlapl, 1e-12);
    assert_vxc_match("vtau", &rust_vtau, &oracle.vtau, 1e-12);
}

// =============================================================================
// mgga_c_b88 VXC tests (ID 571, correlation with laplacian)
// =============================================================================

#[test]
fn test_mgga_c_b88_vxc_unpol() {
    let _lock = CUBECL_LOCK.lock().unwrap();
    let np = 5;
    let rho = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    let sigma = vec![0.01, 0.1, 0.5, 1.0, 2.0];
    let lapl = vec![0.001, 0.01, 0.05, 0.1, 0.2];
    let tau = vec![0.1, 0.3, 0.6, 1.0, 2.0];

    let oracle = libxc_rs_verify::oracle_mgga_all(XC_MGGA_C_B88, 1, &rho, &sigma, &lapl, &tau)
        .expect("oracle_mgga_all failed for mgga_c_b88 vxc unpol");

    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let lapl_h = create_input_buffer(&client, &lapl);
    let tau_h = create_input_buffer(&client, &tau);
    let zk_h = create_zero_output_buffer(&client, np);
    let vrho_h = create_zero_output_buffer(&client, np);
    let vsigma_h = create_zero_output_buffer(&client, np);
    let vlapl_h = create_zero_output_buffer(&client, np);
    let vtau_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        libxc_kernel_mgga_26::mgga_c_b88::vxc_unpol::mgga_c_b88_vxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&lapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&tau_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vsigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vlapl_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vtau_h, np, 1),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_vrho = read_output_buffer(&client, vrho_h, np);
    let rust_vsigma = read_output_buffer(&client, vsigma_h, np);
    let rust_vlapl = read_output_buffer(&client, vlapl_h, np);
    let rust_vtau = read_output_buffer(&client, vtau_h, np);

    eprintln!("mgga_c_b88 vxc unpol:");
    assert_vxc_match("vrho", &rust_vrho, &oracle.vrho, 1e-12);
    assert_vxc_match("vsigma", &rust_vsigma, &oracle.vsigma, 1e-12);
    // mgga_c_b88 uses laplacian -- verify vlapl specifically
    assert_vxc_match("vlapl", &rust_vlapl, &oracle.vlapl, 1e-12);
    assert_vxc_match("vtau", &rust_vtau, &oracle.vtau, 1e-12);
}
