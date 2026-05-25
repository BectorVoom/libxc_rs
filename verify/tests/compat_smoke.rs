//! FFI integration tests for the libxc_rs compat layer (06-03-T4).
//!
//! Calls libxc_rs's `extern "C"` functions through their raw FFI signature (not
//! via the typed Rust API), exercising the C-ABI surface end-to-end. Scenarios:
//!   1. lifecycle_round_trip          — alloc → init → end → init → end → free (no leak)
//!   2. evaluate_all_orders           — exc / exc_vxc / vxc / fxc / kxc / lxc on lda_x at np=4
//!   3. null_skips_derivative         — passing NULL for an output pointer skips it
//!   4. discovery_matches_registry    — xc_number_of_functionals == 649; id↔name parity
//!   5. errno_round_trip              — a non-ok return populates xc_rs_last_error_*
//!   6. hybrid_oracle_b3lyp           — xc_hyb_cam_coef returns alpha = 0.20 for B3LYP
//!   7. ffi_vs_typed_api_bit_equivalence — same input via FFI and typed API; bit-equal output
//!
//! NOTE: building this test compiles libxc_rs with verify's default features
//! (oracle-lda/gga/mgga → all per-functional kernels), so it runs in a
//! full-build / CI environment, not on RAM-constrained dev boxes. The evaluate
//! scenarios require the lda_x kernel (oracle-lda feature).

use std::ffi::{c_char, CStr, CString};
use std::ptr;

// Opaque handle types — ABI-compatible with src/compat/c_layout::{xc_func_type,
// xc_func_info_type} (both zero-sized #[repr(C)]; only the pointer matters).
#[repr(C)]
struct xc_func_type {
    _opaque: [u8; 0],
}
#[repr(C)]
struct xc_func_info_type {
    _opaque: [u8; 0],
}

// Re-declare every extern "C" function exercised below.
unsafe extern "C" {
    fn xc_func_alloc() -> *mut xc_func_type;
    fn xc_func_init(p: *mut xc_func_type, functional: i32, nspin: i32) -> i32;
    fn xc_func_end(p: *mut xc_func_type) -> i32;
    fn xc_func_free(p: *mut xc_func_type);
    fn xc_func_get_info(p: *const xc_func_type) -> *const xc_func_info_type;

    fn xc_func_set_dens_threshold(p: *mut xc_func_type, t: f64) -> i32;

    fn xc_number_of_functionals() -> i32;
    fn xc_functional_get_number(name: *const c_char) -> i32;
    fn xc_functional_get_name(number: i32) -> *const c_char;
    fn xc_family_from_id(id: i32, family: *mut i32, number: *mut i32) -> i32;

    fn xc_lda_exc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64) -> i32;
    fn xc_lda_exc_vxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64) -> i32;
    fn xc_lda_vxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64) -> i32;
    fn xc_lda_fxc(p: *const xc_func_type, np: usize, rho: *const f64, v2rho2: *mut f64) -> i32;
    fn xc_lda_kxc(p: *const xc_func_type, np: usize, rho: *const f64, v3rho3: *mut f64) -> i32;
    fn xc_lda_lxc(p: *const xc_func_type, np: usize, rho: *const f64, v4rho4: *mut f64) -> i32;

    fn xc_hyb_cam_coef(p: *const xc_func_type, omega: *mut f64, alpha: *mut f64, beta: *mut f64);

    fn xc_rs_last_error_code() -> i32;
    fn xc_rs_last_error_message() -> *const c_char;
}

const RHO: [f64; 4] = [0.1, 0.2, 0.3, 0.4];

#[test]
fn lifecycle_round_trip() {
    unsafe {
        let p = xc_func_alloc();
        assert!(!p.is_null(), "alloc returned null");
        assert_eq!(
            xc_func_init(p, 1, 1),
            0,
            "init #1 failed: code={} msg={}",
            xc_rs_last_error_code(),
            CStr::from_ptr(xc_rs_last_error_message()).to_string_lossy()
        );
        // Re-init must drop the previous functional (Pitfall 1), not leak.
        assert_eq!(xc_func_end(p), 0);
        assert_eq!(xc_func_init(p, 2, 1), 0, "init #2 failed");
        // A threshold setter through the FFI path must succeed.
        assert_eq!(xc_func_set_dens_threshold(p, 1e-12), 0);
        // get_info must return a non-null pointer for an initialized handle.
        assert!(!xc_func_get_info(p).is_null());
        assert_eq!(xc_func_end(p), 0);
        xc_func_free(p);
    }
}

#[test]
fn evaluate_all_orders() {
    unsafe {
        let p = xc_func_alloc();
        assert_eq!(xc_func_init(p, 1, 1), 0); // lda_x unpolarized
        let mut zk = [0.0f64; 4];
        let mut vrho = [0.0f64; 4];
        let mut v2rho2 = [0.0f64; 4];
        let mut v3rho3 = [0.0f64; 4];
        let mut v4rho4 = [0.0f64; 4];
        assert_eq!(xc_lda_exc(p, 4, RHO.as_ptr(), zk.as_mut_ptr()), 0);
        assert_eq!(xc_lda_exc_vxc(p, 4, RHO.as_ptr(), zk.as_mut_ptr(), vrho.as_mut_ptr()), 0);
        assert_eq!(xc_lda_vxc(p, 4, RHO.as_ptr(), vrho.as_mut_ptr()), 0);
        assert_eq!(xc_lda_fxc(p, 4, RHO.as_ptr(), v2rho2.as_mut_ptr()), 0);
        assert_eq!(xc_lda_kxc(p, 4, RHO.as_ptr(), v3rho3.as_mut_ptr()), 0);
        assert_eq!(xc_lda_lxc(p, 4, RHO.as_ptr(), v4rho4.as_mut_ptr()), 0);
        for v in &zk {
            assert!(*v < 0.0, "lda_x exc must be negative; got {v}");
        }
        for v in &vrho {
            assert!(*v < 0.0);
        }
        xc_func_end(p);
        xc_func_free(p);
    }
}

#[test]
fn null_skips_derivative() {
    unsafe {
        let p = xc_func_alloc();
        assert_eq!(xc_func_init(p, 1, 1), 0);
        let mut zk = [0.0f64; 4];
        // NULL vrho: zk populated, no crash.
        assert_eq!(xc_lda_exc_vxc(p, 4, RHO.as_ptr(), zk.as_mut_ptr(), ptr::null_mut()), 0);
        for v in &zk {
            assert!(*v < 0.0);
        }
        xc_func_end(p);
        xc_func_free(p);
    }
}

#[test]
fn discovery_matches_registry() {
    unsafe {
        assert_eq!(xc_number_of_functionals(), 649);
        let name = CString::new("lda_x").unwrap();
        let id = xc_functional_get_number(name.as_ptr());
        assert!(id > 0, "lda_x lookup must return a positive id; got {id}");
        let name_back = xc_functional_get_name(id);
        assert!(!name_back.is_null());
        let s = CStr::from_ptr(name_back).to_string_lossy();
        assert_eq!(s, "lda_x");
        let mut family = 0i32;
        let mut number = 0i32;
        assert_eq!(xc_family_from_id(id, &mut family, &mut number), 0);
        assert_eq!(family, 1);
    }
}

#[test]
fn errno_round_trip() {
    unsafe {
        let p = xc_func_alloc();
        let rc = xc_func_init(p, 99999, 1);
        assert!(rc < 0, "expected negative error code; got {rc}");
        assert_eq!(xc_rs_last_error_code(), rc);
        let msg = xc_rs_last_error_message();
        assert!(!msg.is_null());
        let s = CStr::from_ptr(msg).to_string_lossy().to_ascii_lowercase();
        assert!(
            s.contains("unknown") || s.contains("functional"),
            "unexpected error message: {s}"
        );
        xc_func_free(p);
    }
}

#[test]
fn hybrid_oracle_b3lyp() {
    unsafe {
        let p = xc_func_alloc();
        let name = CString::new("hyb_gga_xc_b3lyp").unwrap();
        let id = xc_functional_get_number(name.as_ptr());
        assert!(id > 0, "B3LYP lookup failed: code={}", xc_rs_last_error_code());
        assert_eq!(xc_func_init(p, id, 1), 0);
        let mut omega = 0.0f64;
        let mut alpha = 0.0f64;
        let mut beta = 0.0f64;
        xc_hyb_cam_coef(p, &mut omega, &mut alpha, &mut beta);
        // B3LYP carries 20% exact exchange (single Fock term).
        assert!((alpha - 0.20).abs() < 1e-12, "B3LYP alpha: got {alpha}, expected 0.20");
        xc_func_end(p);
        xc_func_free(p);
    }
}

#[test]
fn ffi_vs_typed_api_bit_equivalence() {
    use libxc_rs::{
        BatchEvaluator, DerivativeOrder, Functional, FunctionalId, LdaInput, LdaOutput, Spin,
    };

    let id = FunctionalId::from_name("lda_x").unwrap();

    // Typed-API path.
    let mut zk_typed = [0.0f64; 4];
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    let input = LdaInput::new(&RHO, 4, Spin::Unpolarized).unwrap();
    let mut output =
        LdaOutput::new(Some(&mut zk_typed), None, None, None, None, 4, Spin::Unpolarized).unwrap();
    let mut be = BatchEvaluator::new(Spin::Unpolarized, 4);
    be.evaluate(&f, &input, DerivativeOrder::Exc, &mut output).unwrap();

    // FFI path.
    let mut zk_ffi = [0.0f64; 4];
    unsafe {
        let p = xc_func_alloc();
        assert_eq!(xc_func_init(p, id.raw() as i32, 1), 0);
        assert_eq!(xc_lda_exc(p, 4, RHO.as_ptr(), zk_ffi.as_mut_ptr()), 0);
        xc_func_end(p);
        xc_func_free(p);
    }

    for i in 0..4 {
        assert_eq!(
            zk_typed[i].to_bits(),
            zk_ffi[i].to_bits(),
            "byte-for-byte mismatch at i={i}: typed={} ffi={}",
            zk_typed[i],
            zk_ffi[i]
        );
    }
}
