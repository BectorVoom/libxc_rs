//! C-ABI threshold setters and ext_params setters/getters.
//!
//! Plan 06-03 EXTENDS this file with the 35 evaluate functions
//! (12 LDA + 12 GGA + 11 MGGA). This task (06-02a-T3) covers the 9
//! setters/getters only.

#![allow(clippy::missing_safety_doc)]

use crate::compat::c_layout::{xc_func_type, LIBXC_EXT_PARAMS_DEFAULT};
use crate::compat::errno::{self, set_error};
use crate::compat::raw_handle::FunctionalSlot;
use crate::dims::Dimensions;
use crate::eval::workspace::EvaluationWorkspace;
use crate::extern_c_wrapper;
use crate::input::LdaInput;
use crate::model::DerivativeOrder;
use crate::output::LdaOutput;
use std::ffi::{c_char, CStr};

// === 4 threshold setters — each forwards to the Phase-5 setter (which now
//     walks auxiliaries per the Pitfall 4 fix in 06-02a-T1 Step 5). ===

/// `int xc_func_set_dens_threshold(xc_func_type *p, double t_dens);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_set_dens_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_dens_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_density_threshold(t);
        Ok(0)
    })
}

/// `int xc_func_set_zeta_threshold(xc_func_type *p, double t_zeta);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_set_zeta_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_zeta_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_zeta_threshold(t);
        Ok(0)
    })
}

/// `int xc_func_set_sigma_threshold(xc_func_type *p, double t_sigma);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_set_sigma_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_sigma_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_sigma_threshold(t);
        Ok(0)
    })
}

/// `int xc_func_set_tau_threshold(xc_func_type *p, double t_tau);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_set_tau_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_tau_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_tau_threshold(t);
        Ok(0)
    })
}

// === 5 ext_params functions ===

/// `int xc_func_set_ext_params(xc_func_type *p, const double *ext_params);`
///
/// Pitfall 10: any value equal to `LIBXC_EXT_PARAMS_DEFAULT` (-999998888.0) is
/// substituted with that parameter's per-spec `default_value` before forwarding.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_set_ext_params(p: *mut xc_func_type, ext_params: *const f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_ext_params", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        let n = f.meta().ext_params.len();
        if n == 0 {
            return Ok(0); // no ext_params on this functional; nothing to do
        }
        if ext_params.is_null() {
            return Err(crate::LibxcRsError::ExtParamCountMismatch {
                id: f.meta().id,
                expected: n,
                actual: 0,
            });
        }
        // SAFETY: caller contract — buffer sized for `meta().ext_params.len()`.
        let raw_slice = unsafe { std::slice::from_raw_parts(ext_params, n) };
        // Pitfall 10 substitution.
        let mut substituted: Vec<f64> = Vec::with_capacity(n);
        for (i, &v) in raw_slice.iter().enumerate() {
            if v == LIBXC_EXT_PARAMS_DEFAULT {
                substituted.push(f.meta().ext_params[i].default_value);
            } else {
                substituted.push(v);
            }
        }
        f.set_ext_params(&substituted)?;
        Ok(0)
    })
}

/// `int xc_func_get_ext_params(const xc_func_type *p, double *ext_params);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_get_ext_params(p: *const xc_func_type, ext_params: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_get_ext_params", {
        let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
        let n = f.meta().ext_params.len();
        if n == 0 {
            return Ok(0);
        }
        if ext_params.is_null() {
            return Err(crate::LibxcRsError::ExtParamCountMismatch {
                id: f.meta().id,
                expected: n,
                actual: 0,
            });
        }
        // SAFETY: caller contract — buffer sized for `meta().ext_params.len()`.
        let slice = unsafe { std::slice::from_raw_parts_mut(ext_params, n) };
        if let Some(vals) = f.ext_params() {
            slice.copy_from_slice(vals);
        } else {
            slice.fill(f64::NAN);
        }
        Ok(0)
    })
}

/// `int xc_func_set_ext_params_name(xc_func_type *p, const char *name, double par);`
///
/// Pitfall 10 substitution applies at the single-name level too.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_set_ext_params_name(
    p: *mut xc_func_type,
    name: *const c_char,
    par: f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_ext_params_name", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        if name.is_null() {
            return Err(crate::LibxcRsError::UnknownExtParamName {
                id: f.meta().id,
                name: "<null>".to_string(),
            });
        }
        // SAFETY: name is non-null; caller contract = valid C string.
        let s = unsafe { CStr::from_ptr(name) }
            .to_str()
            .map_err(|_| crate::LibxcRsError::UnknownExtParamName {
                id: f.meta().id,
                name: "<non-utf8>".to_string(),
            })?;
        // Pitfall 10 substitution at the single-name level.
        let val = if par == LIBXC_EXT_PARAMS_DEFAULT {
            let idx = f
                .meta()
                .ext_params
                .iter()
                .position(|spec| spec.name == s)
                .ok_or_else(|| crate::LibxcRsError::UnknownExtParamName {
                    id: f.meta().id,
                    name: s.to_string(),
                })?;
            f.meta().ext_params[idx].default_value
        } else {
            par
        };
        f.set_ext_param(s, val)?;
        Ok(0)
    })
}

/// `double xc_func_get_ext_params_name(const xc_func_type *p, const char *name);`
///
/// Returns NaN on error (errno set). Double-returning fns cannot use
/// `extern_c_wrapper!` (which returns `i32`), so they hand-roll `catch_unwind`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_get_ext_params_name(
    p: *const xc_func_type,
    name: *const c_char,
) -> f64 {
    if p.is_null() || name.is_null() {
        set_error(
            errno::LIBXC_RS_NULL_HANDLE,
            "xc_func_get_ext_params_name: null pointer",
        );
        return f64::NAN;
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<f64, crate::LibxcRsError> {
            // SAFETY: p and name are non-null (checked above); caller contract.
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            let s = unsafe { CStr::from_ptr(name) }.to_str().map_err(|_| {
                crate::LibxcRsError::UnknownExtParamName {
                    id: f.meta().id,
                    name: "<non-utf8>".to_string(),
                }
            })?;
            f.ext_param(s)
        },
    ));
    match result {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            set_error(e.discriminant(), &e.to_string());
            f64::NAN
        }
        Err(_) => {
            set_error(
                errno::LIBXC_RS_PANIC,
                "xc_func_get_ext_params_name: panic",
            );
            f64::NAN
        }
    }
}

/// `double xc_func_get_ext_params_value(const xc_func_type *p, int number);`
///
/// Returns NaN on error (errno set).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_get_ext_params_value(p: *const xc_func_type, number: i32) -> f64 {
    if p.is_null() {
        set_error(
            errno::LIBXC_RS_NULL_HANDLE,
            "xc_func_get_ext_params_value: null pointer",
        );
        return f64::NAN;
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<f64, crate::LibxcRsError> {
            // SAFETY: p is non-null (checked above); caller contract.
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            f.ext_param_by_index(number as usize)
        },
    ));
    match result {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            set_error(e.discriminant(), &e.to_string());
            f64::NAN
        }
        Err(_) => {
            set_error(
                errno::LIBXC_RS_PANIC,
                "xc_func_get_ext_params_value: panic",
            );
            f64::NAN
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compat::raw_handle::*;

    /// Pitfall 10: passing `LIBXC_EXT_PARAMS_DEFAULT` for every parameter must
    /// substitute the per-spec default values.
    #[test]
    fn ext_params_default_marker_substitution() {
        // Pick the first registered functional that has at least one ext_param.
        let target_id = crate::registry::all_functional_ids()
            .find(|fid| {
                crate::registry::lookup_by_id(fid.raw())
                    .map(|m| !m.ext_params.is_empty())
                    .unwrap_or(false)
            })
            .expect("at least one functional has ext_params");
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, target_id.raw() as i32, 1), 0);
            let meta = crate::registry::lookup_by_id(target_id.raw()).unwrap();
            let n = meta.ext_params.len();
            let vals: Vec<f64> = vec![LIBXC_EXT_PARAMS_DEFAULT; n];
            assert_eq!(xc_func_set_ext_params(p, vals.as_ptr()), 0);
            let mut readback = vec![0.0; n];
            assert_eq!(xc_func_get_ext_params(p, readback.as_mut_ptr()), 0);
            for (i, v) in readback.iter().enumerate() {
                assert_eq!(
                    *v, meta.ext_params[i].default_value,
                    "param {i} default not substituted"
                );
            }
            xc_func_end(p);
            xc_func_free(p);
        }
    }

    /// Threshold setter wired via the compat layer must reach auxiliaries
    /// (covers both the Pitfall 4 fix and the wrapper plumbing in one test).
    #[test]
    fn xc_func_set_dens_threshold_propagates_to_aux_b3lyp() {
        unsafe {
            let p = xc_func_alloc();
            let id = crate::registry::lookup_by_name("hyb_gga_xc_b3lyp")
                .unwrap()
                .raw() as i32;
            assert_eq!(xc_func_init(p, id, 1), 0);
            assert_eq!(xc_func_set_dens_threshold(p, 1e-12), 0);
            let f = FunctionalSlot::as_initialized_const(p).unwrap();
            assert_eq!(f.thresholds().density, 1e-12);
            for aux in f.auxiliary_functionals() {
                assert_eq!(
                    aux.thresholds().density,
                    1e-12,
                    "aux {} did not receive threshold via FFI path",
                    aux.meta().name
                );
            }
            xc_func_end(p);
            xc_func_free(p);
        }
    }
}

// =====================================================================
// 06-03-T1: 12 LDA evaluate functions (+ shared pointer helpers)
// =====================================================================

/// Convert a NULL-able `*mut f64` to `Option<&mut [f64]>` of length `np * stride`.
/// NULL maps to `None` (libxc "skip this derivative" semantics — Pitfall 8).
unsafe fn ptr_to_opt_slice<'a>(ptr: *mut f64, np: usize, stride: usize) -> Option<&'a mut [f64]> {
    if ptr.is_null() {
        None
    } else {
        let len = np.checked_mul(stride).expect("np * stride overflow");
        // SAFETY: caller asserts `ptr` covers `np * stride` f64s with no aliasing.
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
    }
}

/// Build a required (`*const f64`) input slice of length `np * stride`.
unsafe fn input_slice<'a>(ptr: *const f64, np: usize, stride: usize) -> &'a [f64] {
    let len = np.checked_mul(stride).expect("np * stride overflow");
    // SAFETY: caller asserts `ptr` covers `np * stride` f64s.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

/// Build LdaInput + LdaOutput + workspace, run `evaluate_lda`. NULL outputs skipped.
unsafe fn lda_evaluate(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    order: DerivativeOrder,
    zk: *mut f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
    v3rho3: *mut f64,
    v4rho4: *mut f64,
) -> Result<i32, crate::LibxcRsError> {
    // SAFETY: p non-null + initialized (wrapper macro + slot accessor enforce).
    let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
    let spin = f.spin();
    let dims = Dimensions::lda(spin);
    // SAFETY: rho covers np * dims.rho per the C-ABI contract.
    let rho_slice = unsafe { input_slice(rho, np, dims.rho as usize) };
    let input = LdaInput::new(rho_slice, np, spin)?;
    let mut output = LdaOutput::new(
        unsafe { ptr_to_opt_slice(zk, np, dims.zk as usize) },
        unsafe { ptr_to_opt_slice(vrho, np, dims.vrho as usize) },
        unsafe { ptr_to_opt_slice(v2rho2, np, dims.v2rho2 as usize) },
        unsafe { ptr_to_opt_slice(v3rho3, np, dims.v3rho3 as usize) },
        unsafe { ptr_to_opt_slice(v4rho4, np, dims.v4rho4 as usize) },
        np,
        spin,
    )?;
    let mut ws = EvaluationWorkspace::new(np, spin);
    f.evaluate_lda(&input, order, &mut output, &mut ws)?;
    Ok(0)
}

/// `xc_lda_out_params` — struct-of-pointers for the `xc_lda_new` API entry.
#[repr(C)]
pub struct XcLdaOutParams {
    pub zk: *mut f64,
    pub vrho: *mut f64,
    pub v2rho2: *mut f64,
    pub v3rho3: *mut f64,
    pub v4rho4: *mut f64,
}

const NULL_F64: *mut f64 = std::ptr::null_mut();

/// `void xc_lda_exc(p, np, rho, zk);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_exc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Exc, zk, NULL_F64, NULL_F64, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_exc_vxc(p, np, rho, zk, vrho);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_exc_vxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Vxc, zk, vrho, NULL_F64, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_vxc(p, np, rho, vrho);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_vxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_vxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Vxc, NULL_F64, vrho, NULL_F64, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_exc_vxc_fxc(p, np, rho, zk, vrho, v2rho2);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_exc_vxc_fxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64, v2rho2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc_fxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Fxc, zk, vrho, v2rho2, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_vxc_fxc(p, np, rho, vrho, v2rho2);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_vxc_fxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64, v2rho2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_vxc_fxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Fxc, NULL_F64, vrho, v2rho2, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_fxc(p, np, rho, v2rho2);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_fxc(p: *const xc_func_type, np: usize, rho: *const f64, v2rho2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_fxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Fxc, NULL_F64, NULL_F64, v2rho2, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_exc_vxc_fxc_kxc(p, np, rho, zk, vrho, v2rho2, v3rho3);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_exc_vxc_fxc_kxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64, v2rho2: *mut f64, v3rho3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc_fxc_kxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Kxc, zk, vrho, v2rho2, v3rho3, NULL_F64) }
    })
}

/// `void xc_lda_vxc_fxc_kxc(p, np, rho, vrho, v2rho2, v3rho3);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_vxc_fxc_kxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64, v2rho2: *mut f64, v3rho3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_vxc_fxc_kxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Kxc, NULL_F64, vrho, v2rho2, v3rho3, NULL_F64) }
    })
}

/// `void xc_lda_kxc(p, np, rho, v3rho3);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_kxc(p: *const xc_func_type, np: usize, rho: *const f64, v3rho3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_kxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Kxc, NULL_F64, NULL_F64, NULL_F64, v3rho3, NULL_F64) }
    })
}

/// `void xc_lda_lxc(p, np, rho, v4rho4);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_lxc(p: *const xc_func_type, np: usize, rho: *const f64, v4rho4: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_lxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Lxc, NULL_F64, NULL_F64, NULL_F64, NULL_F64, v4rho4) }
    })
}

/// `void xc_lda(p, np, rho, zk, vrho, v2rho2, v3rho3, v4rho4);`
/// Family-summary: infers `order` from the highest non-NULL output (Pitfall 8).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    v2rho2: *mut f64,
    v3rho3: *mut f64,
    v4rho4: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_lda", {
        let order = if !v4rho4.is_null() {
            DerivativeOrder::Lxc
        } else if !v3rho3.is_null() {
            DerivativeOrder::Kxc
        } else if !v2rho2.is_null() {
            DerivativeOrder::Fxc
        } else if !vrho.is_null() {
            DerivativeOrder::Vxc
        } else if !zk.is_null() {
            DerivativeOrder::Exc
        } else {
            return Ok(0); // all NULL: no-op (libxc parity)
        };
        unsafe { lda_evaluate(p, np, rho, order, zk, vrho, v2rho2, v3rho3, v4rho4) }
    })
}

/// `void xc_lda_new(p, order, np, rho, xc_lda_out_params *out);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_new(
    p: *const xc_func_type,
    order: i32,
    np: usize,
    rho: *const f64,
    out: *const XcLdaOutParams,
) -> i32 {
    extern_c_wrapper!(p, "xc_lda_new", {
        if out.is_null() {
            return Err(crate::LibxcRsError::OutputBufferSizeMismatch {
                field: "out",
                expected: 1,
                actual: 0,
            });
        }
        // SAFETY: out is non-null + caller contract (valid XcLdaOutParams).
        let o: &XcLdaOutParams = unsafe { &*out };
        let der_order = unsafe { lda_order_from_int(order, p) }?;
        unsafe { lda_evaluate(p, np, rho, der_order, o.zk, o.vrho, o.v2rho2, o.v3rho3, o.v4rho4) }
    })
}

/// Map the integer `order` argument (0..=4) of `xc_*_new` to `DerivativeOrder`.
unsafe fn lda_order_from_int(order: i32, p: *const xc_func_type) -> Result<DerivativeOrder, crate::LibxcRsError> {
    Ok(match order {
        0 => DerivativeOrder::Exc,
        1 => DerivativeOrder::Vxc,
        2 => DerivativeOrder::Fxc,
        3 => DerivativeOrder::Kxc,
        4 => DerivativeOrder::Lxc,
        _ => {
            // SAFETY: p non-null + initialized (wrapper macro enforced).
            let id = unsafe { FunctionalSlot::as_initialized_const(p)? }.meta().id;
            return Err(crate::LibxcRsError::UnsupportedDerivativeOrder {
                id,
                order: DerivativeOrder::Lxc,
                max: DerivativeOrder::Lxc,
            });
        }
    })
}

#[cfg(test)]
mod lda_evaluate_tests {
    use super::*;
    use crate::compat::raw_handle::*;

    #[test]
    fn lda_exc_smoke() {
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 1), 0); // lda_x unpolarized
            let rho = [0.1f64, 0.2, 0.3, 0.4];
            let mut zk = [0.0f64; 4];
            assert_eq!(xc_lda_exc(p, 4, rho.as_ptr(), zk.as_mut_ptr()), 0);
            for v in &zk {
                assert!(*v < 0.0, "lda_x exc must be negative; got {v}");
            }
            xc_func_end(p);
            xc_func_free(p);
        }
    }

    #[test]
    fn xc_lda_infers_order_from_outputs() {
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 1), 0);
            let rho = [0.1f64, 0.2, 0.3, 0.4];
            let mut zk = [0.0f64; 4];
            let mut vrho = [0.0f64; 4];
            assert_eq!(
                xc_lda(p, 4, rho.as_ptr(), zk.as_mut_ptr(), vrho.as_mut_ptr(), NULL_F64, NULL_F64, NULL_F64),
                0
            );
            for v in &zk {
                assert!(*v < 0.0);
            }
            for v in &vrho {
                assert!(*v < 0.0);
            }
            // All-NULL: no-op, return 0.
            assert_eq!(
                xc_lda(p, 4, rho.as_ptr(), NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64),
                0
            );
            xc_func_end(p);
            xc_func_free(p);
        }
    }

    #[test]
    fn null_skips_derivative_in_exc_vxc() {
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 1), 0);
            let rho = [0.1f64, 0.2, 0.3, 0.4];
            let mut zk = [0.0f64; 4];
            assert_eq!(xc_lda_exc_vxc(p, 4, rho.as_ptr(), zk.as_mut_ptr(), NULL_F64), 0);
            for v in &zk {
                assert!(*v < 0.0);
            }
            xc_func_end(p);
            xc_func_free(p);
        }
    }
}
