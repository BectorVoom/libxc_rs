//! Test xc_integrate_func0 and xc_integrate_func1 against reference values.

use cubecl::cpu::{CpuDevice, CpuRuntime};
use cubecl::client::ComputeClient;
use cubecl::prelude::*;
use gga_test::math::integrate::{xc_integrate_func0, xc_integrate_func1};

fn cpu_client() -> ComputeClient<CpuRuntime> {
    CpuRuntime::client(&CpuDevice)
}

#[cube(launch_unchecked)]
fn func0_kernel(out: &mut Array<f64>, b: f64, beta: f64) {
    let ip = ABSOLUTE_POS;
    if ip < out.len() {
        out[ip] = xc_integrate_func0(b, beta);
    }
}

#[cube(launch_unchecked)]
fn func1_kernel(out: &mut Array<f64>, b: f64, beta: f64) {
    let ip = ABSOLUTE_POS;
    if ip < out.len() {
        out[ip] = xc_integrate_func1(b, beta);
    }
}

fn run_func0(b: f64, beta: f64) -> f64 {
    let client = cpu_client();
    let out_handle = client.create_from_slice(bytemuck::cast_slice(&[0.0f64]));
    let cube_dim = CubeDim::new_1d(1);
    let cube_count = CubeCount::new_1d(1);

    unsafe {
        func0_kernel::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&out_handle, 1, 1),
            ScalarArg::new(b),
            ScalarArg::new(beta),
        ).unwrap();
    }

    let bytes = client.read_one(out_handle);
    let out: Vec<f64> = bytemuck::cast_slice(&bytes).to_vec();
    out[0]
}

fn run_func1(b: f64, beta: f64) -> f64 {
    let client = cpu_client();
    let out_handle = client.create_from_slice(bytemuck::cast_slice(&[0.0f64]));
    let cube_dim = CubeDim::new_1d(1);
    let cube_count = CubeCount::new_1d(1);

    unsafe {
        func1_kernel::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&out_handle, 1, 1),
            ScalarArg::new(b),
            ScalarArg::new(beta),
        ).unwrap();
    }

    let bytes = client.read_one(out_handle);
    let out: Vec<f64> = bytemuck::cast_slice(&bytes).to_vec();
    out[0]
}

fn relative_error(got: f64, expected: f64) -> f64 {
    if expected.abs() < 1e-30 {
        got.abs()
    } else {
        ((got - expected) / expected).abs()
    }
}

// Reference values: func0 via 100M-point Simpson, func1 via subtraction + 100M-point Simpson
const REF_BETA_005: &[(f64, f64, f64)] = &[
    // (upper_bound, func0_integral, func1_integral)
    (0.01,  -4.72466643823814031e-04,  2.64825844574383476e-03),
    (0.10,  -4.72096518803047024e-03,  1.55938802571691619e-02),
    (0.50,  -2.31861402492676803e-02,  3.95451591731543484e-02),
    (1.00,  -4.42678408444776642e-02,  4.61666825470977149e-02),
    (2.00,  -7.82629692122618764e-02,  3.36266636301658373e-02),
    (5.00,  -1.38309648985081440e-01, -3.62993068771999411e-02),
    (10.00, -1.85833444693337763e-01, -1.29133996561913600e-01),
    (20.00, -2.29681191256620648e-01, -2.45019257925421119e-01),
];

// Reference values: func0 via 100M-point Simpson, func1 via subtraction + 100M-point Simpson
const REF_BETA_0004: &[(f64, f64, f64)] = &[
    (0.01,  -3.77976074973376561e-05,  2.11862038644964614e-04),
    (0.10,  -3.77952355605454184e-04,  1.24823578298660295e-03),
    (0.50,  -1.88699558515659227e-03,  3.19687075711687233e-03),
    (1.00,  -3.75861576985947422e-03,  3.77238651827826706e-03),
    (2.00,  -7.42131688939819123e-03,  2.36438787576311168e-03),
    (5.00,  -1.75815846106949156e-02, -9.93804140650038532e-03),
    (10.00, -3.17887214563654236e-02, -3.81115940544755546e-02),
    (20.00, -5.27198098394433545e-02, -9.39320774811523429e-02),
];

#[test]
fn test_xc_integrate_func0_beta005() {
    let beta = 0.05;
    let tol = 5e-12;
    let mut max_err = 0.0f64;

    for &(upper, expected, _) in REF_BETA_005 {
        let got = run_func0(upper, beta);
        let err = relative_error(got, expected);
        if err > max_err { max_err = err; }
        assert!(
            err < tol,
            "func0(b={upper}, beta={beta}): got={got:.15e}, expected={expected:.15e}, rel_err={err:.3e}"
        );
    }
    eprintln!("func0 beta=0.05 max_rel_err = {max_err:.3e}");
}

#[test]
fn test_xc_integrate_func1_beta005() {
    let beta = 0.05;
    let tol = 5e-12;
    let mut max_err = 0.0f64;

    for &(upper, _, expected) in REF_BETA_005 {
        let got = run_func1(upper, beta);
        let err = relative_error(got, expected);
        if err > max_err { max_err = err; }
        assert!(
            err < tol,
            "func1(b={upper}, beta={beta}): got={got:.15e}, expected={expected:.15e}, rel_err={err:.3e}"
        );
    }
    eprintln!("func1 beta=0.05 max_rel_err = {max_err:.3e}");
}

#[test]
fn test_xc_integrate_func0_beta0004() {
    let beta = 0.004;
    let tol = 5e-12;
    let mut max_err = 0.0f64;

    for &(upper, expected, _) in REF_BETA_0004 {
        let got = run_func0(upper, beta);
        let err = relative_error(got, expected);
        if err > max_err { max_err = err; }
        assert!(
            err < tol,
            "func0(b={upper}, beta={beta}): got={got:.15e}, expected={expected:.15e}, rel_err={err:.3e}"
        );
    }
    eprintln!("func0 beta=0.004 max_rel_err = {max_err:.3e}");
}

#[test]
fn test_xc_integrate_func1_beta0004() {
    let beta = 0.004;
    let tol = 5e-12;
    let mut max_err = 0.0f64;

    for &(upper, _, expected) in REF_BETA_0004 {
        let got = run_func1(upper, beta);
        let err = relative_error(got, expected);
        if err > max_err { max_err = err; }
        assert!(
            err < tol,
            "func1(b={upper}, beta={beta}): got={got:.15e}, expected={expected:.15e}, rel_err={err:.3e}"
        );
    }
    eprintln!("func1 beta=0.004 max_rel_err = {max_err:.3e}");
}
