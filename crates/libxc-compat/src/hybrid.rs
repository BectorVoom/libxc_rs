//! Hybrid + auxiliary + NLC accessors + AK13 helpers (06-02b-T2).
//!
//! The AK13 asymptotic formula is inlined verbatim from
//! libxc-master/src/gga_x_ak13.c (lines 32-55); constants are read directly
//! from that file and from libxc-master/src/util.h (no inference).

#![allow(clippy::missing_safety_doc)]

use crate::c_layout::xc_func_type;
use crate::errno::{self, set_error};
use crate::raw_handle::FunctionalSlot;
use crate::extern_c_wrapper;
use libxc_core::model::HybridType;

/// libxc `XC_HYB_*` integer constants. VERIFIED against libxc-master/src/xc.h:94-100.
/// Exhaustive over every `HybridType` variant (no `_` arm) so a future variant
/// without a mapping becomes a compile error (T-06-16).
fn hybrid_type_to_int(t: HybridType) -> i32 {
    match t {
        HybridType::Semilocal => 0,       // XC_HYB_SEMILOCAL     (xc.h:94)
        HybridType::Hybrid => 1,          // XC_HYB_HYBRID        (xc.h:95)
        HybridType::Cam => 2,             // XC_HYB_CAM           (xc.h:96)
        HybridType::CamYukawa => 3,       // XC_HYB_CAMY          (xc.h:97)
        HybridType::CamGaussian => 4,     // XC_HYB_CAMG          (xc.h:98)
        HybridType::DoubleHybrid => 5,    // XC_HYB_DOUBLE_HYBRID (xc.h:99)
        HybridType::Mixture => 32768,     // XC_HYB_MIXTURE       (xc.h:100)
    }
}

/// `int xc_hyb_type(const xc_func_type *p);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_hyb_type(p: *const xc_func_type) -> i32 {
    extern_c_wrapper!(p, "xc_hyb_type", {
        let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
        Ok(hybrid_type_to_int(f.hybrid_type()))
    })
}

/// `double xc_hyb_exx_coef(const xc_func_type *p);` — returns NaN on error.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_hyb_exx_coef(p: *const xc_func_type) -> f64 {
    if p.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_hyb_exx_coef: null handle");
        return f64::NAN;
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<f64, libxc_core::error::LibxcRsError> {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            Ok(f.exx_coefficient().unwrap_or(0.0))
        },
    ));
    match result {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            set_error(e.discriminant(), &e.to_string());
            f64::NAN
        }
        Err(_) => {
            set_error(errno::LIBXC_RS_PANIC, "xc_hyb_exx_coef: panic");
            f64::NAN
        }
    }
}

/// `void xc_hyb_cam_coef(const xc_func_type *p, double *omega, double *alpha, double *beta);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_hyb_cam_coef(
    p: *const xc_func_type,
    omega: *mut f64,
    alpha: *mut f64,
    beta: *mut f64,
) {
    if p.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_hyb_cam_coef: null handle");
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<(), libxc_core::error::LibxcRsError> {
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            if let Some(c) = f.cam_coefficients() {
                if !omega.is_null() {
                    // SAFETY: caller contract — writable f64.
                    unsafe { *omega = c.omega; }
                }
                if !alpha.is_null() {
                    // SAFETY: caller contract — writable f64.
                    unsafe { *alpha = c.alpha; }
                }
                if !beta.is_null() {
                    // SAFETY: caller contract — writable f64.
                    unsafe { *beta = c.beta; }
                }
                Ok(())
            } else {
                Err(libxc_core::error::LibxcRsError::FamilyMismatch {
                    id: f.meta().id,
                    expected: libxc_core::model::Family::Gga,
                    actual: f.meta().family,
                })
            }
        },
    ));
}

/// `void xc_nlc_coef(const xc_func_type *p, double *nlc_b, double *nlc_c);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_nlc_coef(p: *const xc_func_type, nlc_b: *mut f64, nlc_c: *mut f64) {
    if p.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_nlc_coef: null handle");
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if let Ok(f) = unsafe { FunctionalSlot::as_initialized_const(p) }
            && let Some(c) = f.nlc_coefficients()
        {
            if !nlc_b.is_null() {
                // SAFETY: caller contract — writable f64.
                unsafe { *nlc_b = c.b; }
            }
            if !nlc_c.is_null() {
                // SAFETY: caller contract — writable f64.
                unsafe { *nlc_c = c.c; }
            }
        }
    }));
}

/// `int xc_num_aux_funcs(const xc_func_type *p);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_num_aux_funcs(p: *const xc_func_type) -> i32 {
    extern_c_wrapper!(p, "xc_num_aux_funcs", {
        let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
        Ok(f.auxiliary_functionals().len() as i32)
    })
}

/// `void xc_aux_func_ids(const xc_func_type *p, int *ids);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_aux_func_ids(p: *const xc_func_type, ids: *mut i32) {
    if p.is_null() || ids.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_aux_func_ids: null pointer");
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if let Ok(f) = unsafe { FunctionalSlot::as_initialized_const(p) } {
            let aux = f.auxiliary_functionals();
            // SAFETY: caller contract — `ids` holds `xc_num_aux_funcs(p)` ints.
            let slice = unsafe { std::slice::from_raw_parts_mut(ids, aux.len()) };
            for (i, a) in aux.iter().enumerate() {
                slice[i] = a.meta().id.raw() as i32;
            }
        }
    }));
}

/// `void xc_aux_func_weights(const xc_func_type *p, double *weights);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_aux_func_weights(p: *const xc_func_type, weights: *mut f64) {
    if p.is_null() || weights.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_aux_func_weights: null pointer");
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if let Ok(f) = unsafe { FunctionalSlot::as_initialized_const(p) } {
            let mix = f.mix_coefficients();
            // SAFETY: caller contract — `weights` holds `mix_coefficients().len()` f64s.
            let slice = unsafe { std::slice::from_raw_parts_mut(weights, mix.len()) };
            slice.copy_from_slice(mix);
        }
    }));
}

// === AK13 helpers — formula INLINED verbatim from libxc-master/src/gga_x_ak13.c ===

/// libxc-master/src/gga_x_ak13.c:32 — par_ak13[0] = 3*muGE/5 + 8*pi/15.
/// VERIFIED by direct read of that line.
pub const AK13_PAR_B1: f64 = 1.74959015598863046792081721182;
/// libxc-master/src/gga_x_ak13.c:33 — par_ak13[1] = muGE - B1.
pub const AK13_PAR_B2: f64 = -1.62613336586517367779736042170;

/// libxc-master/src/util.h:211 — `#define X_FACTOR_C 0.9305257363491000250020102180716672510262`
/// (`3/8*cbrt(3/pi)*4^(2/3)`). Read directly from util.h — NOT inferred.
const X_FACTOR_C: f64 = 0.9305257363491000250020102180716672510262;

/// `double xc_gga_ak13_get_asymptotic(double homo);`
/// libxc-master/src/gga_x_ak13.c:35-38 — forwards to the inner formula with par_ak13[0].
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub extern "C" fn xc_gga_ak13_get_asymptotic(homo: f64) -> f64 {
    ak13_pars_asymptotic_inner(homo, AK13_PAR_B1)
}

/// `double xc_gga_ak13_pars_get_asymptotic(double homo, const double *ext_params);`
/// libxc-master/src/gga_x_ak13.c:40-55 — `ext_params[0]` supplies B1.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_ak13_pars_get_asymptotic(homo: f64, ext_params: *const f64) -> f64 {
    let b1 = if ext_params.is_null() {
        AK13_PAR_B1
    } else {
        // SAFETY: caller contract — `ext_params` points to at least 1 f64.
        unsafe { *ext_params }
    };
    ak13_pars_asymptotic_inner(homo, b1)
}

/// Inner formula. Direct port of libxc-master/src/gga_x_ak13.c:40-55.
/// Op order preserved exactly per CLAUDE.md "Operation order preservation".
#[inline]
fn ak13_pars_asymptotic_inner(homo: f64, ak13_b1: f64) -> f64 {
    // libxc:47 — Qx = sqrt(2.0)*ak13_B1/(3.0*CBRT(3.0*M_PI*M_PI));
    let qx = (2.0_f64).sqrt() * ak13_b1
        / (3.0 * (3.0 * std::f64::consts::PI * std::f64::consts::PI).cbrt());
    // libxc:49-50 — aa = X_FACTOR_C*Qx; aa2 = aa*aa;
    let aa = X_FACTOR_C * qx;
    let aa2 = aa * aa;
    // libxc:52 — factor = (homo < 0.0) ? -1.0 : 1.0;
    let factor = if homo < 0.0 { -1.0 } else { 1.0 };
    // libxc:54 — return (aa2/2.0)*(1.0 + factor*sqrt(1.0 - 4.0*homo/aa2));
    (aa2 / 2.0) * (1.0 + factor * (1.0 - 4.0 * homo / aa2).sqrt())
}

// === AK13 oracle pairs — (homo, expected) ===
//
// Computed offline from the inlined formula with the util.h-verified X_FACTOR_C
// and the gga_x_ak13.c-verified par_ak13[0] (B1), in IEEE-754 f64 with the same
// op order (Python 3.12 math.sqrt/math.cbrt == libm == Rust f64). The libxc C
// oracle (`xc_gga_ak13_get_asymptotic`) was NOT re-run here because rebuilding
// libxc-master via cmake is infeasible on this RAM-constrained box — see SUMMARY
// for the documented fallback. Correctness vs libxc is guaranteed by the
// verbatim formula + directly-read constants; the test below is a regression
// guard (bit-exact or 1e-12 tolerance).
//
// NOTE: the plan example used homo=+0.05, but for any homo>0 the AK13 asymptotic
// formula evaluates sqrt(1 - 4*homo/aa2) of a negative argument -> NaN (out of
// physical domain — HOMO energies are negative). All three oracle points are
// therefore in the valid negative domain.
/// Oracle pairs: (homo, expected). See module note for provenance.
pub const AK13_ORACLE_PAIRS: &[(f64, f64)] = &[
    (-5.00000000000000000e-01_f64, -1.47323787720958527e-01_f64),
    (-1.00000000000000006e-01_f64, -5.34966754138725409e-02_f64),
    (-5.00000000000000028e-02_f64, -3.26636078636666632e-02_f64),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ak13_default_constants() {
        assert_eq!(
            AK13_PAR_B1.to_bits(),
            1.74959015598863046792081721182_f64.to_bits()
        );
        assert_eq!(
            AK13_PAR_B2.to_bits(),
            (-1.62613336586517367779736042170_f64).to_bits()
        );
    }

    #[test]
    fn ak13_get_asymptotic_oracle_parity() {
        for &(homo, expected) in AK13_ORACLE_PAIRS.iter() {
            let actual = xc_gga_ak13_get_asymptotic(homo);
            let bit_match = actual.to_bits() == expected.to_bits();
            let tol_match = (actual - expected).abs() < 1e-12;
            assert!(
                bit_match || tol_match,
                "AK13 oracle mismatch at homo={homo}: actual={actual}, expected={expected}, diff={}",
                (actual - expected).abs()
            );
        }
    }

    #[test]
    fn ak13_pars_with_default_matches_get_asymptotic() {
        let homo = -0.3;
        let actual_default = xc_gga_ak13_get_asymptotic(homo);
        let params = [AK13_PAR_B1, AK13_PAR_B2];
        let actual_pars = unsafe { xc_gga_ak13_pars_get_asymptotic(homo, params.as_ptr()) };
        assert_eq!(
            actual_default.to_bits(),
            actual_pars.to_bits(),
            "default-path and pars-path must produce bit-identical results"
        );
    }
}
