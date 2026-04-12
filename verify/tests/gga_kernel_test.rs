//! Direct CubeCL kernel vs oracle comparison test for GGA functionals.
//!
//! Tests translated GGA kernels by launching them directly via CubeCL
//! and comparing output against the C libxc oracle.
//!
//! IMPORTANT: libxc's work_gga_inc.c clamps rho and sigma inputs before
//! calling the maple2c functions. Our kernels receive raw values.
//! Tests must use input values well above thresholds to avoid clamping effects.

#![allow(clippy::needless_range_loop)]

use cubecl::cpu::CpuRuntime;
use cubecl::prelude::*;
use libxc_rs::kernel::gga::{gga_c_cs1, gga_x_pbe};
use libxc_rs::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer,
    create_zero_output_buffer, read_output_buffer,
};
use libxc_rs_verify::{oracle_gga_all, oracle_gga_all_with_opts, OracleOptions};

const EXC_TOL: f64 = 1e-12;
const VXC_TOL: f64 = 1e-10;
// Use per-functional thresholds matching libxc defaults.
// PBE: dens_threshold = 1e-15, CS1: dens_threshold = 1e-20
// zeta_threshold defaults to DBL_EPSILON in libxc.
// For unpolarized, zeta doesn't affect results.
// For polarized, we set the oracle to match our zeta.

// Use input values well above thresholds to avoid clamping effects.

fn relative_error(actual: f64, expected: f64) -> f64 {
    if expected == 0.0 {
        actual.abs()
    } else {
        ((actual - expected) / expected).abs()
    }
}

// ============================================================================
// gga_x_pbe (ID 101) - exchange with params (kappa, mu)
// PBE default dens_threshold = 1e-15
// ============================================================================

#[test]
fn test_gga_x_pbe_exc_unpol() {
    let func_id = 101;
    let kappa = 0.8040;
    let mu = 0.2195149727645171;
    let dens_threshold = 1e-15;
    let zeta_threshold = 1e-10;

    // All rho >> dens_threshold, so no clamping
    let rho = vec![0.1, 0.2, 0.5, 1.0];
    let sigma = vec![0.01, 0.04, 0.25, 1.0];
    let np = rho.len();

    let oracle = oracle_gga_all(func_id, 1, &rho, &sigma).expect("oracle failed");
    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        gga_x_pbe::exc_unpol::gga_x_pbe_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(kappa), ScalarArg::new(mu),
            ScalarArg::new(dens_threshold), ScalarArg::new(zeta_threshold),
        ).expect("kernel launch failed");
    }

    let zk = read_output_buffer(&client, zk_h, np);
    for i in 0..np {
        let err = relative_error(zk[i], oracle.zk[i]);
        assert!(err < EXC_TOL,
            "pbe exc_unpol zk[{i}]: got {}, expected {}, rel_err = {err:.2e}", zk[i], oracle.zk[i]);
    }
}

#[test]
fn test_gga_x_pbe_vxc_unpol() {
    let func_id = 101;
    let kappa = 0.8040;
    let mu = 0.2195149727645171;
    let dens_threshold = 1e-15;
    let zeta_threshold = 1e-10;

    let rho = vec![0.1, 0.5, 1.0];
    let sigma = vec![0.01, 0.25, 1.0];
    let np = rho.len();

    // Set oracle thresholds to match kernel (zeta_threshold matters for vxc)
    let oracle = oracle_gga_all_with_opts(func_id, 1, &rho, &sigma, &OracleOptions {
        ext_params: None,
        dens_threshold: Some(dens_threshold),
        zeta_threshold: Some(zeta_threshold),
    }).expect("oracle failed");
    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let zk_h = create_zero_output_buffer(&client, np);
    let vrho_h = create_zero_output_buffer(&client, np);
    let vsigma_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        gga_x_pbe::vxc_unpol::gga_x_pbe_vxc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vsigma_h, np, 1),
            ScalarArg::new(kappa), ScalarArg::new(mu),
            ScalarArg::new(dens_threshold), ScalarArg::new(zeta_threshold),
        ).expect("kernel launch failed");
    }

    let zk = read_output_buffer(&client, zk_h, np);
    let vrho = read_output_buffer(&client, vrho_h, np);
    let vsigma = read_output_buffer(&client, vsigma_h, np);

    for i in 0..np {
        let err = relative_error(zk[i], oracle.zk[i]);
        assert!(err < EXC_TOL, "pbe vxc zk[{i}]: rel_err = {err:.2e}");
    }
    for i in 0..np {
        let err = relative_error(vrho[i], oracle.vrho[i]);
        assert!(err < VXC_TOL, "pbe vxc vrho[{i}]: got {}, expected {}, rel_err = {err:.2e}", vrho[i], oracle.vrho[i]);
    }
    for i in 0..np {
        let err = relative_error(vsigma[i], oracle.vsigma[i]);
        assert!(err < VXC_TOL, "pbe vxc vsigma[{i}]: got {}, expected {}, rel_err = {err:.2e}", vsigma[i], oracle.vsigma[i]);
    }
}

// ============================================================================
// gga_c_cs1 (ID 565) - correlation, no params
// CS1 default dens_threshold = 1e-20
// ============================================================================

#[test]
fn test_gga_c_cs1_exc_unpol() {
    let func_id = 565;
    let dens_threshold = 1e-20;
    let zeta_threshold = 1e-10;

    let rho = vec![0.1, 0.5, 1.0];
    let sigma = vec![0.01, 0.25, 1.0];
    let np = rho.len();

    let oracle = oracle_gga_all_with_opts(func_id, 1, &rho, &sigma, &OracleOptions {
        ext_params: None,
        dens_threshold: Some(dens_threshold),
        zeta_threshold: Some(zeta_threshold),
    }).expect("oracle failed");
    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        gga_c_cs1::exc_unpol::gga_c_cs1_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(dens_threshold), ScalarArg::new(zeta_threshold),
        ).expect("kernel launch failed");
    }

    let zk = read_output_buffer(&client, zk_h, np);
    for i in 0..np {
        let err = relative_error(zk[i], oracle.zk[i]);
        assert!(err < EXC_TOL,
            "cs1 exc_unpol zk[{i}]: got {}, expected {}, rel_err = {err:.2e}", zk[i], oracle.zk[i]);
    }
}

#[test]
fn test_gga_c_cs1_exc_pol() {
    let func_id = 565;
    let dens_threshold = 1e-20;
    let zeta_threshold = 1e-10;
    let np = 2;
    let rho = vec![0.3, 0.2, 0.6, 0.4];
    let sigma = vec![0.05, 0.02, 0.03, 0.20, 0.08, 0.12];

    // Set oracle thresholds to match kernel
    let oracle = oracle_gga_all_with_opts(func_id, 2, &rho, &sigma, &OracleOptions {
        ext_params: None,
        dens_threshold: Some(dens_threshold),
        zeta_threshold: Some(zeta_threshold),
    }).expect("oracle failed");
    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let zk_h = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        gga_c_cs1::exc_pol::gga_c_cs1_exc_pol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, rho.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, sigma.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ScalarArg::new(dens_threshold), ScalarArg::new(zeta_threshold),
        ).expect("kernel launch failed");
    }

    let zk = read_output_buffer(&client, zk_h, np);
    for i in 0..np {
        let err = relative_error(zk[i], oracle.zk[i]);
        assert!(err < EXC_TOL,
            "cs1 exc_pol zk[{i}]: got {}, expected {}, rel_err = {err:.2e}", zk[i], oracle.zk[i]);
    }
}

#[test]
fn test_gga_c_cs1_vxc_pol() {
    let func_id = 565;
    let dens_threshold = 1e-20;
    let zeta_threshold = 1e-10;
    let np = 2;
    let rho = vec![0.3, 0.2, 0.6, 0.4];
    let sigma = vec![0.05, 0.02, 0.03, 0.20, 0.08, 0.12];

    let oracle = oracle_gga_all_with_opts(func_id, 2, &rho, &sigma, &OracleOptions {
        ext_params: None,
        dens_threshold: Some(dens_threshold),
        zeta_threshold: Some(zeta_threshold),
    }).expect("oracle failed");
    let client = cpu_client();
    let rho_h = create_input_buffer(&client, &rho);
    let sigma_h = create_input_buffer(&client, &sigma);
    let zk_h = create_zero_output_buffer(&client, np);
    let vrho_h = create_zero_output_buffer(&client, np * 2);
    let vsigma_h = create_zero_output_buffer(&client, np * 3);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        gga_c_cs1::vxc_pol::gga_c_cs1_vxc_pol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_h, rho.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&sigma_h, sigma.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_h, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_h, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&vsigma_h, np * 3, 1),
            ScalarArg::new(dens_threshold), ScalarArg::new(zeta_threshold),
        ).expect("kernel launch failed");
    }

    let zk = read_output_buffer(&client, zk_h, np);
    let vrho = read_output_buffer(&client, vrho_h, np * 2);
    let vsigma = read_output_buffer(&client, vsigma_h, np * 3);

    for i in 0..np {
        let err = relative_error(zk[i], oracle.zk[i]);
        assert!(err < EXC_TOL, "cs1 vxc_pol zk[{i}]: rel_err = {err:.2e}");
    }
    for i in 0..np * 2 {
        let err = relative_error(vrho[i], oracle.vrho[i]);
        assert!(err < VXC_TOL,
            "cs1 vxc_pol vrho[{i}]: got {}, expected {}, rel_err = {err:.2e}", vrho[i], oracle.vrho[i]);
    }
    for i in 0..np * 3 {
        let err = relative_error(vsigma[i], oracle.vsigma[i]);
        assert!(err < VXC_TOL,
            "cs1 vxc_pol vsigma[{i}]: got {}, expected {}, rel_err = {err:.2e}", vsigma[i], oracle.vsigma[i]);
    }
}
