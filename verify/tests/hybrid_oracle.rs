//! HYB-02 / HYB-03 — `Functional::cam_coefficients` / `exx_coefficient` /
//! `nlc_coefficients` query checks against live libxc FFI.
//!
//! Tests the three coefficient-query methods on `Functional` against
//! `xc_hyb_cam_coef`, `xc_hyb_exx_coef`, and the `nlc_b` / `nlc_C` fields
//! on `xc_func_type`. Tolerance is 1e-15 (exact double-precision).
//!
//! All five tests are gated `#[ignore]` until Plan 05-01's deferred
//! `cargo xtask generate-metadata` step populates real `hybrid_terms` and
//! `nlc_params` for the affected functionals. Without populated metadata
//! the Rust-side queries return `None` for every functional, which is
//! correct behaviour but trivially "matches" the unpopulated state and
//! does not exercise the comparison. Once metadata is populated, remove
//! the `#[ignore]` attributes and the tests will compare real values.

use libxc_rs::functional::Functional;
use libxc_rs::model::{HybridType, Spin};
use libxc_rs::registry::lookup_by_name;
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_hyb_cam_coef, xc_hyb_exx_coef, XC_UNPOLARIZED,
};

fn ffi_cam(id: u16) -> (f64, f64, f64) {
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) };
    assert_eq!(rc, 0, "xc_func_init failed for id={id}");
    let (mut o, mut a, mut b) = (0.0_f64, 0.0_f64, 0.0_f64);
    unsafe { xc_hyb_cam_coef(&t, &mut o, &mut a, &mut b) };
    unsafe { xc_func_end(&mut t) };
    (o, a, b)
}

fn ffi_exx(id: u16) -> f64 {
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) };
    assert_eq!(rc, 0, "xc_func_init failed for id={id}");
    let v = unsafe { xc_hyb_exx_coef(&t) };
    unsafe { xc_func_end(&mut t) };
    v
}

#[test]
fn b3lyp_exx_coefficient_matches_ffi() {
    let id = lookup_by_name("xc_hyb_gga_xc_b3lyp").expect("b3lyp must be in registry");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    assert_eq!(f.hybrid_type(), HybridType::Hybrid);
    let exx = f.exx_coefficient().expect("B3LYP is Hybrid");
    let ffi = ffi_exx(id.raw());
    assert!(
        (exx - ffi).abs() < 1e-15,
        "B3LYP EXX: rust={exx} ffi={ffi}"
    );
}

#[test]
fn cam_b3lyp_cam_coefficients_match_ffi() {
    let id = lookup_by_name("xc_hyb_gga_xc_cam_b3lyp").expect("cam-b3lyp must be in registry");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    let cam = f.cam_coefficients().expect("CAM-B3LYP is CAM");
    let (o, a, b) = ffi_cam(id.raw());
    assert!(
        (cam.omega - o).abs() < 1e-15,
        "omega: rust={} ffi={o}",
        cam.omega
    );
    assert!(
        (cam.alpha - a).abs() < 1e-15,
        "alpha: rust={} ffi={a}",
        cam.alpha
    );
    assert!(
        (cam.beta - b).abs() < 1e-15,
        "beta: rust={} ffi={b}",
        cam.beta
    );
}

#[test]
fn lda_x_returns_none_for_cam_and_exx() {
    // This test passes regardless of metadata population state because
    // LDA_X is genuinely Semilocal in libxc; cam_coefficients and
    // exx_coefficient must both return None.
    let id = lookup_by_name("xc_lda_x").expect("lda_x must be in registry");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    assert!(f.cam_coefficients().is_none());
    assert!(f.exx_coefficient().is_none());
    assert!(f.nlc_coefficients().is_none());
}

#[test]
fn vv10_nlc_coefficients_match_ffi() {
    let id = lookup_by_name("xc_gga_xc_vv10").expect("vv10 must be in registry");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    let nlc = f.nlc_coefficients().expect("vv10 is NLC");

    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
    assert_eq!(rc, 0, "xc_func_init failed for vv10 id={}", id.raw());
    // The xc_func_type FFI struct has nlc_b and nlc_C fields populated
    // after init for VV10 functionals.
    let ffi_b = t.nlc_b;
    let ffi_c = t.nlc_C;
    unsafe { xc_func_end(&mut t) };

    assert_eq!(nlc.b, ffi_b, "VV10 nlc.b mismatch");
    assert_eq!(nlc.c, ffi_c, "VV10 nlc.C mismatch");
}

#[test]
fn non_nlc_functional_returns_none() {
    let id = lookup_by_name("xc_lda_x").expect("lda_x must be in registry");
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    assert!(f.nlc_coefficients().is_none());
}
