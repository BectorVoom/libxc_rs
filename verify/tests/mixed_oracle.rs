//! FUNC-04 / HYB-04 — mixed-evaluation oracle tests for 5 representative
//! hybrid functionals.
//!
//! Calls `Functional::evaluate_gga` / `evaluate_mgga` on B3LYP,
//! CAM-B3LYP (default + perturbed `_omega`), HSE03, mgga_c_b94_hyb, and
//! wB97X, comparing against `xc_gga_vxc` / `xc_mgga_vxc` from the live
//! libxc 7.0.0 oracle. Tolerance: zk rel_err <= 1e-12, vrho/vsigma rel_err
//! <= 1e-10.
//!
//! All five tests are gated `#[ignore]` until Plan 05-01's deferred
//! `cargo xtask generate-metadata` populates `meta.auxiliaries` for each
//! hybrid (currently `&[]` for all 649 ids). Without populated
//! `meta.auxiliaries`, `Functional::new` builds an empty `auxiliaries`
//! Vec for B3LYP et al., and `Functional::evaluate_gga` falls through
//! to direct `dispatch_gga` on the libxc id — which currently returns
//! `UnsupportedFunctional` because hybrid GGA ids are not present in
//! the GGA dispatch enum. This is the expected lifecycle: metadata
//! population unlocks the mixed path that this test exercises.

use approx::assert_relative_eq;
use libxc_rs::eval::EvaluationWorkspace;
use libxc_rs::functional::Functional;
use libxc_rs::input::{GgaInput, MggaInput};
use libxc_rs::model::{DerivativeOrder, Spin};
use libxc_rs::output::{GgaOutput, MggaOutput};
use libxc_rs::registry::lookup_by_name;
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_set_ext_params_name, xc_func_type, xc_gga_vxc, xc_mgga_vxc,
    XC_UNPOLARIZED,
};

const NP: usize = 16;

fn deterministic_gga_grid() -> (Vec<f64>, Vec<f64>) {
    let rho: Vec<f64> = (0..NP).map(|i| 0.1_f64 * (i + 1) as f64).collect();
    let sigma: Vec<f64> = (0..NP).map(|i| 0.01_f64 * (i + 1) as f64).collect();
    (rho, sigma)
}

fn deterministic_mgga_grid() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let rho: Vec<f64> = (0..NP).map(|i| 0.1_f64 * (i + 1) as f64).collect();
    let sigma: Vec<f64> = (0..NP).map(|i| 0.01_f64 * (i + 1) as f64).collect();
    let lapl: Vec<f64> = (0..NP).map(|i| 0.001_f64 * (i + 1) as f64).collect();
    let tau: Vec<f64> = (0..NP).map(|i| 0.05_f64 * (i + 1) as f64).collect();
    (rho, sigma, lapl, tau)
}

fn run_gga_oracle_compare(name: &str, omega: Option<f64>) {
    let id = lookup_by_name(name).expect("registry must contain functional");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    let (rho, sigma) = deterministic_gga_grid();

    let input = GgaInput::new(&rho, &sigma, NP, Spin::Unpolarized).unwrap();

    let mut zk_r = vec![0.0_f64; NP];
    let mut vrho_r = vec![0.0_f64; NP];
    let mut vsigma_r = vec![0.0_f64; NP];
    let mut workspace = EvaluationWorkspace::new(NP, Spin::Unpolarized);
    {
        let mut output = GgaOutput::new(
            Some(&mut zk_r),
            Some(&mut vrho_r),
            Some(&mut vsigma_r),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            NP,
            Spin::Unpolarized,
        )
        .unwrap();
        let mut f_mut = f;
        if let Some(o) = omega {
            f_mut
                .set_ext_param("_omega", o)
                .expect("set _omega should succeed for CAM functionals");
        }
        f_mut
            .evaluate_gga(&input, DerivativeOrder::Vxc, &mut output, &mut workspace)
            .expect("evaluate_gga should succeed for hybrid GGA");
    }

    // libxc oracle
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
    assert_eq!(rc, 0, "xc_func_init must succeed for {name}");
    if let Some(o) = omega {
        // Set _omega ext param via FFI for like-for-like comparison.
        let cstr = std::ffi::CString::new("_omega").unwrap();
        unsafe { xc_func_set_ext_params_name(&mut t, cstr.as_ptr(), o) };
    }
    let mut zk_c = vec![0.0_f64; NP];
    let mut vrho_c = vec![0.0_f64; NP];
    let mut vsigma_c = vec![0.0_f64; NP];
    unsafe {
        xc_gga_vxc(
            &t,
            NP as i32,
            rho.as_ptr(),
            sigma.as_ptr(),
            zk_c.as_mut_ptr(),
            vrho_c.as_mut_ptr(),
            vsigma_c.as_mut_ptr(),
        );
        xc_func_end(&mut t);
    }

    for i in 0..NP {
        assert_relative_eq!(zk_r[i], zk_c[i], max_relative = 1e-12);
        assert_relative_eq!(vrho_r[i], vrho_c[i], max_relative = 1e-10);
        assert_relative_eq!(vsigma_r[i], vsigma_c[i], max_relative = 1e-10);
    }
}

fn run_mgga_oracle_compare(name: &str) {
    let id = lookup_by_name(name).expect("registry must contain functional");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    let (rho, sigma, lapl, tau) = deterministic_mgga_grid();
    let input = MggaInput::new(&rho, &sigma, &lapl, &tau, NP, Spin::Unpolarized).unwrap();

    let mut zk_r = vec![0.0_f64; NP];
    let mut vrho_r = vec![0.0_f64; NP];
    let mut vsigma_r = vec![0.0_f64; NP];
    let mut vlapl_r = vec![0.0_f64; NP];
    let mut vtau_r = vec![0.0_f64; NP];
    let mut workspace = EvaluationWorkspace::new(NP, Spin::Unpolarized);
    {
        let mut output = MggaOutput::default();
        output.zk = Some(&mut zk_r);
        output.vrho = Some(&mut vrho_r);
        output.vsigma = Some(&mut vsigma_r);
        output.vlapl = Some(&mut vlapl_r);
        output.vtau = Some(&mut vtau_r);
        f.evaluate_mgga(&input, DerivativeOrder::Vxc, &mut output, &mut workspace)
            .expect("evaluate_mgga should succeed for hybrid MGGA");
    }

    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
    assert_eq!(rc, 0, "xc_func_init must succeed for {name}");
    let mut zk_c = vec![0.0_f64; NP];
    let mut vrho_c = vec![0.0_f64; NP];
    let mut vsigma_c = vec![0.0_f64; NP];
    let mut vlapl_c = vec![0.0_f64; NP];
    let mut vtau_c = vec![0.0_f64; NP];
    unsafe {
        xc_mgga_vxc(
            &t,
            NP as i32,
            rho.as_ptr(),
            sigma.as_ptr(),
            lapl.as_ptr(),
            tau.as_ptr(),
            zk_c.as_mut_ptr(),
            vrho_c.as_mut_ptr(),
            vsigma_c.as_mut_ptr(),
            vlapl_c.as_mut_ptr(),
            vtau_c.as_mut_ptr(),
        );
        xc_func_end(&mut t);
    }
    for i in 0..NP {
        assert_relative_eq!(zk_r[i], zk_c[i], max_relative = 1e-12);
        assert_relative_eq!(vrho_r[i], vrho_c[i], max_relative = 1e-10);
        assert_relative_eq!(vsigma_r[i], vsigma_c[i], max_relative = 1e-10);
        assert_relative_eq!(vlapl_r[i], vlapl_c[i], max_relative = 1e-10);
        assert_relative_eq!(vtau_r[i], vtau_c[i], max_relative = 1e-10);
    }
}

#[test]
fn b3lyp_gga_vxc_matches_libxc() {
    run_gga_oracle_compare("hyb_gga_xc_b3lyp", None);
}

#[test]
fn cam_b3lyp_gga_vxc_matches_libxc_default() {
    run_gga_oracle_compare("hyb_gga_xc_cam_b3lyp", None);
}

#[test]
fn cam_b3lyp_gga_vxc_matches_libxc_omega_0_5() {
    run_gga_oracle_compare("hyb_gga_xc_cam_b3lyp", Some(0.5));
}

#[test]
fn hse03_gga_vxc_matches_libxc() {
    run_gga_oracle_compare("hyb_gga_xc_hse03", None);
}

#[test]
fn wb97x_gga_vxc_matches_libxc() {
    run_gga_oracle_compare("hyb_gga_xc_wb97x", None);
}

#[test]
fn b94_hyb_mgga_vxc_matches_libxc() {
    run_mgga_oracle_compare("mgga_c_b94_hyb");
}
