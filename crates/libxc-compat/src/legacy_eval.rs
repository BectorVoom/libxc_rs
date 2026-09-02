//! C-ABI threshold setters and ext_params setters/getters.
//!
//! Plan 06-03 EXTENDS this file with the 35 evaluate functions
//! (12 LDA + 12 GGA + 11 MGGA). This task (06-02a-T3) covers the 9
//! setters/getters only.

#![allow(clippy::missing_safety_doc)]

use crate::c_layout::{xc_func_type, LIBXC_EXT_PARAMS_DEFAULT};
use crate::errno::{self, set_error};
use crate::raw_handle::FunctionalSlot;
use libxc_core::dims::Dimensions;
use libxc_eval::eval::workspace::EvaluationWorkspace;

// ---------------------------------------------------------------------------
// Reused evaluation workspace
// ---------------------------------------------------------------------------

/// One `EvaluationWorkspace` per thread, reused across calls.
///
/// The C API hands back a `*mut xc_func_type` with a fixed layout, so the
/// workspace cannot live on the handle without changing that layout. A
/// thread-local achieves the same thing for the property that matters:
/// `AGENTS.md` requires that repeated workloads reuse workspaces instead of
/// reallocating on hot paths, and every `xc_lda`/`xc_gga`/`xc_mgga` call used
/// to build a fresh one.
///
/// It starts at `DerivativeOrder::Exc`, which is one or two doubles per grid
/// point. A semilocal functional never grows it, because it never touches the
/// scratch at all; a mixed one grows it once, in `evaluate_mixed_*`, to
/// exactly the order being evaluated. Either way the second and later calls at
/// a given `(np, spin)` allocate nothing.
///
/// A change of `np` or `spin` replaces it. In a DFT code `np` is the grid
/// batch size and does not vary call to call.
fn with_workspace<R>(
    np: usize,
    spin: libxc_core::model::Spin,
    f: impl FnOnce(&mut EvaluationWorkspace) -> R,
) -> R {
    use std::cell::RefCell;
    thread_local! {
        static WS: RefCell<Option<EvaluationWorkspace>> = const { RefCell::new(None) };
    }
    WS.with(|cell| {
        let mut slot = cell.borrow_mut();
        let stale = match slot.as_ref() {
            Some(w) => w.np() != np || w.spin() != spin,
            None => true,
        };
        if stale {
            *slot = Some(EvaluationWorkspace::with_order(
                np,
                spin,
                libxc_core::model::DerivativeOrder::Exc,
            ));
        }
        f(slot.as_mut().expect("just populated"))
    })
}

use crate::extern_c_wrapper;
use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::DerivativeOrder;
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};
use std::ffi::{c_char, CStr};

// === 4 threshold setters — each forwards to the Phase-5 setter (which now
//     walks auxiliaries per the Pitfall 4 fix in 06-02a-T1 Step 5). ===

/// `int xc_func_set_dens_threshold(xc_func_type *p, double t_dens);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_set_dens_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_dens_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_density_threshold(t);
        Ok(0)
    })
}

/// `int xc_func_set_zeta_threshold(xc_func_type *p, double t_zeta);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_set_zeta_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_zeta_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_zeta_threshold(t);
        Ok(0)
    })
}

/// `int xc_func_set_sigma_threshold(xc_func_type *p, double t_sigma);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_set_sigma_threshold(p: *mut xc_func_type, t: f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_sigma_threshold", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        f.set_sigma_threshold(t);
        Ok(0)
    })
}

/// `int xc_func_set_tau_threshold(xc_func_type *p, double t_tau);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_set_ext_params(p: *mut xc_func_type, ext_params: *const f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_ext_params", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        let n = f.meta().ext_params.len();
        if n == 0 {
            return Ok(0); // no ext_params on this functional; nothing to do
        }
        if ext_params.is_null() {
            return Err(libxc_core::error::LibxcRsError::ExtParamCountMismatch {
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_get_ext_params(p: *const xc_func_type, ext_params: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_func_get_ext_params", {
        let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
        let n = f.meta().ext_params.len();
        if n == 0 {
            return Ok(0);
        }
        if ext_params.is_null() {
            return Err(libxc_core::error::LibxcRsError::ExtParamCountMismatch {
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_set_ext_params_name(
    p: *mut xc_func_type,
    name: *const c_char,
    par: f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_func_set_ext_params_name", {
        let f = unsafe { FunctionalSlot::as_initialized_mut(p)? };
        if name.is_null() {
            return Err(libxc_core::error::LibxcRsError::UnknownExtParamName {
                id: f.meta().id,
                name: "<null>".to_string(),
            });
        }
        // SAFETY: name is non-null; caller contract = valid C string.
        let s = unsafe { CStr::from_ptr(name) }
            .to_str()
            .map_err(|_| libxc_core::error::LibxcRsError::UnknownExtParamName {
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
                .ok_or_else(|| libxc_core::error::LibxcRsError::UnknownExtParamName {
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
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
        || -> Result<f64, libxc_core::error::LibxcRsError> {
            // SAFETY: p and name are non-null (checked above); caller contract.
            let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
            let s = unsafe { CStr::from_ptr(name) }.to_str().map_err(|_| {
                libxc_core::error::LibxcRsError::UnknownExtParamName {
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_get_ext_params_value(p: *const xc_func_type, number: i32) -> f64 {
    if p.is_null() {
        set_error(
            errno::LIBXC_RS_NULL_HANDLE,
            "xc_func_get_ext_params_value: null pointer",
        );
        return f64::NAN;
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<f64, libxc_core::error::LibxcRsError> {
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
    use crate::raw_handle::*;

    /// Pitfall 10: passing `LIBXC_EXT_PARAMS_DEFAULT` for every parameter must
    /// substitute the per-spec default values.
    #[test]
    fn ext_params_default_marker_substitution() {
        // Pick the first registered functional that has at least one ext_param.
        let target_id = libxc_core::registry::all_functional_ids()
            .find(|fid| {
                libxc_core::registry::lookup_by_id(fid.raw())
                    .map(|m| !m.ext_params.is_empty())
                    .unwrap_or(false)
            })
            .expect("at least one functional has ext_params");
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, target_id.raw() as i32, 1), 0);
            let meta = libxc_core::registry::lookup_by_id(target_id.raw()).unwrap();
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
            let id = libxc_core::registry::lookup_by_name("hyb_gga_xc_b3lyp")
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
) -> Result<i32, libxc_core::error::LibxcRsError> {
    // SAFETY: p non-null + initialized (wrapper macro + slot accessor enforce).
    let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
    let spin = f.spin();
    let dims = Dimensions::lda(spin);
    // SAFETY: rho covers np * dims.rho per the C-ABI contract.
    let rho_slice = unsafe { input_slice(rho, np, dims.rho as usize) };
    let input = LdaInput::new(rho_slice, np, spin)?;

    let max_provided = if !v4rho4.is_null() {
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
        return Ok(0);
    };
    let effective_order = order.min(max_provided);

    let mut scratch = vec![0.0; dims.total_output_components() * np];
    #[allow(unused_assignments)]
    let mut cursor = scratch.as_mut_slice();

    macro_rules! lda_slot {
        ($ptr:expr, $field:ident, $req_order:ident) => {{
            let len = dims.$field as usize * np;
            if !$ptr.is_null() {
                unsafe { ptr_to_opt_slice($ptr, np, dims.$field as usize) }
            } else if effective_order >= DerivativeOrder::$req_order {
                let (head, rest) = cursor.split_at_mut(len);
                cursor = rest;
                let _ = &cursor;
                Some(head)
            } else {
                None
            }
        }};
    }

    let mut output = LdaOutput::new(
        lda_slot!(zk, zk, Exc),
        lda_slot!(vrho, vrho, Vxc),
        lda_slot!(v2rho2, v2rho2, Fxc),
        lda_slot!(v3rho3, v3rho3, Kxc),
        lda_slot!(v4rho4, v4rho4, Lxc),
        np,
        spin,
    )?;
    with_workspace(np, spin, |ws| {
        f.evaluate_lda(&input, effective_order, &mut output, ws)
    })?;
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_exc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Exc, zk, NULL_F64, NULL_F64, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_exc_vxc(p, np, rho, zk, vrho);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_exc_vxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Vxc, zk, vrho, NULL_F64, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_vxc(p, np, rho, vrho);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_vxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_vxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Vxc, NULL_F64, vrho, NULL_F64, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_exc_vxc_fxc(p, np, rho, zk, vrho, v2rho2);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_exc_vxc_fxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64, v2rho2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc_fxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Fxc, zk, vrho, v2rho2, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_vxc_fxc(p, np, rho, vrho, v2rho2);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_vxc_fxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64, v2rho2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_vxc_fxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Fxc, NULL_F64, vrho, v2rho2, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_fxc(p, np, rho, v2rho2);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_fxc(p: *const xc_func_type, np: usize, rho: *const f64, v2rho2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_fxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Fxc, NULL_F64, NULL_F64, v2rho2, NULL_F64, NULL_F64) }
    })
}

/// `void xc_lda_exc_vxc_fxc_kxc(p, np, rho, zk, vrho, v2rho2, v3rho3);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_exc_vxc_fxc_kxc(p: *const xc_func_type, np: usize, rho: *const f64, zk: *mut f64, vrho: *mut f64, v2rho2: *mut f64, v3rho3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc_fxc_kxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Kxc, zk, vrho, v2rho2, v3rho3, NULL_F64) }
    })
}

/// `void xc_lda_vxc_fxc_kxc(p, np, rho, vrho, v2rho2, v3rho3);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_vxc_fxc_kxc(p: *const xc_func_type, np: usize, rho: *const f64, vrho: *mut f64, v2rho2: *mut f64, v3rho3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_vxc_fxc_kxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Kxc, NULL_F64, vrho, v2rho2, v3rho3, NULL_F64) }
    })
}

/// `void xc_lda_kxc(p, np, rho, v3rho3);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_kxc(p: *const xc_func_type, np: usize, rho: *const f64, v3rho3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_kxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Kxc, NULL_F64, NULL_F64, NULL_F64, v3rho3, NULL_F64) }
    })
}

/// `void xc_lda_lxc(p, np, rho, v4rho4);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_lxc(p: *const xc_func_type, np: usize, rho: *const f64, v4rho4: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_lda_lxc", {
        unsafe { lda_evaluate(p, np, rho, DerivativeOrder::Lxc, NULL_F64, NULL_F64, NULL_F64, NULL_F64, v4rho4) }
    })
}

/// `void xc_lda(p, np, rho, zk, vrho, v2rho2, v3rho3, v4rho4);`
/// Family-summary: infers `order` from the highest non-NULL output (Pitfall 8).
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
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
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_lda_new(
    p: *const xc_func_type,
    order: i32,
    np: usize,
    rho: *const f64,
    out: *const XcLdaOutParams,
) -> i32 {
    extern_c_wrapper!(p, "xc_lda_new", {
        if out.is_null() {
            return Err(libxc_core::error::LibxcRsError::OutputBufferSizeMismatch {
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
unsafe fn lda_order_from_int(order: i32, p: *const xc_func_type) -> Result<DerivativeOrder, libxc_core::error::LibxcRsError> {
    Ok(match order {
        0 => DerivativeOrder::Exc,
        1 => DerivativeOrder::Vxc,
        2 => DerivativeOrder::Fxc,
        3 => DerivativeOrder::Kxc,
        4 => DerivativeOrder::Lxc,
        _ => {
            // SAFETY: p non-null + initialized (wrapper macro enforced).
            let id = unsafe { FunctionalSlot::as_initialized_const(p)? }.meta().id;
            return Err(libxc_core::error::LibxcRsError::UnsupportedDerivativeOrder {
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
    use crate::raw_handle::*;

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

// =====================================================================
// 06-03-T2a: 12 GGA evaluate functions
// =====================================================================

/// Build GgaInput + GgaOutput + workspace, run `evaluate_gga`. NULL outputs skipped.
/// Output-pointer order matches `GgaOutput::new`.
unsafe fn gga_evaluate(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    order: DerivativeOrder,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rhosigma2: *mut f64,
    v3sigma3: *mut f64,
    v4rho4: *mut f64,
    v4rho3sigma: *mut f64,
    v4rho2sigma2: *mut f64,
    v4rhosigma3: *mut f64,
    v4sigma4: *mut f64,
) -> Result<i32, libxc_core::error::LibxcRsError> {
    // SAFETY: p non-null + initialized (wrapper macro + slot accessor enforce).
    let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
    let spin = f.spin();
    let dims = Dimensions::gga(spin);
    // SAFETY: rho/sigma cover np * dims.{rho,sigma} per the C-ABI contract.
    let rho_slice = unsafe { input_slice(rho, np, dims.rho as usize) };
    let sigma_slice = unsafe { input_slice(sigma, np, dims.sigma as usize) };
    let input = GgaInput::new(rho_slice, sigma_slice, np, spin)?;

    let max_provided = if !v4rho4.is_null()
        || !v4rho3sigma.is_null()
        || !v4rho2sigma2.is_null()
        || !v4rhosigma3.is_null()
        || !v4sigma4.is_null()
    {
        DerivativeOrder::Lxc
    } else if !v3rho3.is_null()
        || !v3rho2sigma.is_null()
        || !v3rhosigma2.is_null()
        || !v3sigma3.is_null()
    {
        DerivativeOrder::Kxc
    } else if !v2rho2.is_null() || !v2rhosigma.is_null() || !v2sigma2.is_null() {
        DerivativeOrder::Fxc
    } else if !vrho.is_null() || !vsigma.is_null() {
        DerivativeOrder::Vxc
    } else if !zk.is_null() {
        DerivativeOrder::Exc
    } else {
        return Ok(0);
    };
    let effective_order = order.min(max_provided);

    let mut scratch = vec![0.0; dims.total_output_components() * np];
    #[allow(unused_assignments)]
    let mut cursor = scratch.as_mut_slice();

    macro_rules! gga_slot {
        ($ptr:expr, $field:ident, $req_order:ident) => {{
            let len = dims.$field as usize * np;
            if !$ptr.is_null() {
                unsafe { ptr_to_opt_slice($ptr, np, dims.$field as usize) }
            } else if effective_order >= DerivativeOrder::$req_order {
                let (head, rest) = cursor.split_at_mut(len);
                cursor = rest;
                let _ = &cursor;
                Some(head)
            } else {
                None
            }
        }};
    }

    let mut output = GgaOutput::new(
        gga_slot!(zk, zk, Exc),
        gga_slot!(vrho, vrho, Vxc),
        gga_slot!(vsigma, vsigma, Vxc),
        gga_slot!(v2rho2, v2rho2, Fxc),
        gga_slot!(v2rhosigma, v2rhosigma, Fxc),
        gga_slot!(v2sigma2, v2sigma2, Fxc),
        gga_slot!(v3rho3, v3rho3, Kxc),
        gga_slot!(v3rho2sigma, v3rho2sigma, Kxc),
        gga_slot!(v3rhosigma2, v3rhosigma2, Kxc),
        gga_slot!(v3sigma3, v3sigma3, Kxc),
        gga_slot!(v4rho4, v4rho4, Lxc),
        gga_slot!(v4rho3sigma, v4rho3sigma, Lxc),
        gga_slot!(v4rho2sigma2, v4rho2sigma2, Lxc),
        gga_slot!(v4rhosigma3, v4rhosigma3, Lxc),
        gga_slot!(v4sigma4, v4sigma4, Lxc),
        np,
        spin,
    )?;
    with_workspace(np, spin, |ws| {
        f.evaluate_gga(&input, effective_order, &mut output, ws)
    })?;
    Ok(0)
}

/// `xc_gga_out_params` — struct-of-pointers for the `xc_gga_new` API entry.
#[repr(C)]
pub struct XcGgaOutParams {
    pub zk: *mut f64,
    pub vrho: *mut f64,
    pub vsigma: *mut f64,
    pub v2rho2: *mut f64,
    pub v2rhosigma: *mut f64,
    pub v2sigma2: *mut f64,
    pub v3rho3: *mut f64,
    pub v3rho2sigma: *mut f64,
    pub v3rhosigma2: *mut f64,
    pub v3sigma3: *mut f64,
    pub v4rho4: *mut f64,
    pub v4rho3sigma: *mut f64,
    pub v4rho2sigma2: *mut f64,
    pub v4rhosigma3: *mut f64,
    pub v4sigma4: *mut f64,
}

/// `void xc_gga_exc(p, np, rho, sigma, zk);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_exc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, zk: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_exc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Exc, zk,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_exc_vxc(p, np, rho, sigma, zk, vrho, vsigma);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_exc_vxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, zk: *mut f64, vrho: *mut f64, vsigma: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_exc_vxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Vxc, zk, vrho, vsigma,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_vxc(p, np, rho, sigma, vrho, vsigma);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_vxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, vrho: *mut f64, vsigma: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_vxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Vxc, NULL_F64, vrho, vsigma,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_exc_vxc_fxc(p, np, rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_exc_vxc_fxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, zk: *mut f64, vrho: *mut f64, vsigma: *mut f64, v2rho2: *mut f64, v2rhosigma: *mut f64, v2sigma2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_exc_vxc_fxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Fxc, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_vxc_fxc(p, np, rho, sigma, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_vxc_fxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, vrho: *mut f64, vsigma: *mut f64, v2rho2: *mut f64, v2rhosigma: *mut f64, v2sigma2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_vxc_fxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Fxc, NULL_F64, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_fxc(p, np, rho, sigma, v2rho2, v2rhosigma, v2sigma2);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_fxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, v2rho2: *mut f64, v2rhosigma: *mut f64, v2sigma2: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_fxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Fxc, NULL_F64, NULL_F64, NULL_F64, v2rho2, v2rhosigma, v2sigma2,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_exc_vxc_fxc_kxc(p, np, rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_exc_vxc_fxc_kxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, zk: *mut f64, vrho: *mut f64, vsigma: *mut f64, v2rho2: *mut f64, v2rhosigma: *mut f64, v2sigma2: *mut f64, v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rhosigma2: *mut f64, v3sigma3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_exc_vxc_fxc_kxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Kxc, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
                v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_vxc_fxc_kxc(p, np, rho, sigma, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_vxc_fxc_kxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, vrho: *mut f64, vsigma: *mut f64, v2rho2: *mut f64, v2rhosigma: *mut f64, v2sigma2: *mut f64, v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rhosigma2: *mut f64, v3sigma3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_vxc_fxc_kxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Kxc, NULL_F64, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
                v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_kxc(p, np, rho, sigma, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_kxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rhosigma2: *mut f64, v3sigma3: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_kxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Kxc, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64,
                v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64)
        }
    })
}

/// `void xc_gga_lxc(p, np, rho, sigma, v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_lxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, v4rho4: *mut f64, v4rho3sigma: *mut f64, v4rho2sigma2: *mut f64, v4rhosigma3: *mut f64, v4sigma4: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_gga_lxc", {
        unsafe {
            gga_evaluate(p, np, rho, sigma, DerivativeOrder::Lxc, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64, NULL_F64,
                NULL_F64, NULL_F64, NULL_F64, NULL_F64, v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4)
        }
    })
}

/// `void xc_gga(p, np, rho, sigma, zk … v4sigma4);` — family-summary, order inferred (Pitfall 8).
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn xc_gga(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
    vsigma: *mut f64,
    v2rho2: *mut f64,
    v2rhosigma: *mut f64,
    v2sigma2: *mut f64,
    v3rho3: *mut f64,
    v3rho2sigma: *mut f64,
    v3rhosigma2: *mut f64,
    v3sigma3: *mut f64,
    v4rho4: *mut f64,
    v4rho3sigma: *mut f64,
    v4rho2sigma2: *mut f64,
    v4rhosigma3: *mut f64,
    v4sigma4: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_gga", {
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
            return Ok(0);
        };
        unsafe {
            gga_evaluate(p, np, rho, sigma, order, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2,
                v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4)
        }
    })
}

/// `void xc_gga_new(p, order, np, rho, sigma, xc_gga_out_params *out);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_gga_new(
    p: *const xc_func_type,
    order: i32,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    out: *const XcGgaOutParams,
) -> i32 {
    extern_c_wrapper!(p, "xc_gga_new", {
        if out.is_null() {
            return Err(libxc_core::error::LibxcRsError::OutputBufferSizeMismatch {
                field: "out",
                expected: 1,
                actual: 0,
            });
        }
        // SAFETY: out non-null + caller contract (valid XcGgaOutParams).
        let o: &XcGgaOutParams = unsafe { &*out };
        let der_order = unsafe { lda_order_from_int(order, p) }?;
        unsafe {
            gga_evaluate(p, np, rho, sigma, der_order, o.zk, o.vrho, o.vsigma, o.v2rho2, o.v2rhosigma, o.v2sigma2,
                o.v3rho3, o.v3rho2sigma, o.v3rhosigma2, o.v3sigma3, o.v4rho4, o.v4rho3sigma, o.v4rho2sigma2, o.v4rhosigma3, o.v4sigma4)
        }
    })
}

#[cfg(test)]
mod gga_evaluate_tests {
    use super::*;
    use crate::raw_handle::*;

    #[test]
    fn gga_exc_smoke() {
        unsafe {
            let p = xc_func_alloc();
            let pbe_x_id = libxc_core::registry::lookup_by_name("gga_x_pbe").unwrap().raw() as i32;
            assert_eq!(xc_func_init(p, pbe_x_id, 1), 0);
            let rho = [0.1f64, 0.2, 0.3, 0.4];
            let sigma = [0.01f64, 0.02, 0.03, 0.04];
            let mut zk = [0.0f64; 4];
            assert_eq!(xc_gga_exc(p, 4, rho.as_ptr(), sigma.as_ptr(), zk.as_mut_ptr()), 0);
            for v in &zk {
                assert!(*v < 0.0, "gga_x_pbe exc must be negative; got {v}");
            }
            xc_func_end(p);
            xc_func_free(p);
        }
    }
}

// =====================================================================
// 06-03-T2b: 11 MGGA evaluate functions
// =====================================================================
//
// MggaOutput has ~70 Option<&mut [f64]> derivative fields. Rather than a 70-arg
// constructor, each entry point builds the struct via `mgga_out!` (struct-literal
// + ..Default::default()), listing only the fields it exposes. The macro relies
// on `param name == MggaOutput field name == Dimensions field name` (the libxc
// canonical naming used throughout this crate); `cargo check` validates every
// name. NULL pointers map to None (skip-derivative). No `xc_mgga_new` exists in
// libxc's xc.h, so none is provided here.

/// Build an `MggaOutput` from a set of NULL-able output pointers. Each `$field`
/// names a fn parameter, an `MggaOutput` field, and a `Dimensions::mgga` stride.
macro_rules! mgga_out {
    ($np:expr, $dims:expr; $($field:ident),* $(,)?) => {
        MggaOutput {
            $( $field: unsafe { ptr_to_opt_slice($field, $np, $dims.$field as usize) }, )*
            ..Default::default()
        }
    };
}

/// Read the functional once to derive the MGGA `Dimensions` (strides) for output sizing.
unsafe fn mgga_dims(p: *const xc_func_type) -> Result<Dimensions, libxc_core::error::LibxcRsError> {
    // SAFETY: p non-null + initialized (wrapper macro enforced).
    let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
    Ok(Dimensions::mgga(f.spin()))
}

/// Build MggaInput + workspace, validate the (pre-built) output, run `evaluate_mgga`.
unsafe fn mgga_run(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    sigma: *const f64,
    lapl: *const f64,
    tau: *const f64,
    order: DerivativeOrder,
    mut output: MggaOutput,
) -> Result<i32, libxc_core::error::LibxcRsError> {
    // SAFETY: p non-null + initialized (wrapper macro + slot accessor enforce).
    let f = unsafe { FunctionalSlot::as_initialized_const(p)? };
    let spin = f.spin();
    let dims = Dimensions::mgga(spin);
    // SAFETY: rho/sigma/lapl/tau cover np * dims.{field} per the C-ABI contract.
    let rho_slice = unsafe { input_slice(rho, np, dims.rho as usize) };
    let sigma_slice = unsafe { input_slice(sigma, np, dims.sigma as usize) };
    let lapl_slice = unsafe { input_slice(lapl, np, dims.lapl as usize) };
    let tau_slice = unsafe { input_slice(tau, np, dims.tau as usize) };
    let input = MggaInput::new(rho_slice, sigma_slice, lapl_slice, tau_slice, np, spin)?;
    output.validate(np, spin)?;
    with_workspace(np, spin, |ws| {
        f.evaluate_mgga(&input, order, &mut output, ws)
    })?;
    Ok(0)
}

/// `void xc_mgga_exc(p, np, rho, sigma, lapl, tau, zk);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_exc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64, zk: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_exc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims; zk);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Exc, output) }
    })
}

/// `void xc_mgga_exc_vxc(p, np, rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_exc_vxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64, zk: *mut f64, vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_exc_vxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims; zk, vrho, vsigma, vlapl, vtau);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Vxc, output) }
    })
}

/// `void xc_mgga_vxc(p, np, rho, sigma, lapl, tau, vrho, vsigma, vlapl, vtau);`
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_vxc(p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64, vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_vxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims; vrho, vsigma, vlapl, vtau);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Vxc, output) }
    })
}

/// `void xc_mgga_exc_vxc_fxc(...)` — orders 0,1,2.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_exc_vxc_fxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    zk: *mut f64, vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64,
    v2rho2: *mut f64, v2rhosigma: *mut f64, v2rholapl: *mut f64, v2rhotau: *mut f64,
    v2sigma2: *mut f64, v2sigmalapl: *mut f64, v2sigmatau: *mut f64, v2lapl2: *mut f64,
    v2lapltau: *mut f64, v2tau2: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_exc_vxc_fxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            zk, vrho, vsigma, vlapl, vtau,
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Fxc, output) }
    })
}

/// `void xc_mgga_vxc_fxc(...)` — orders 1,2.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_vxc_fxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64,
    v2rho2: *mut f64, v2rhosigma: *mut f64, v2rholapl: *mut f64, v2rhotau: *mut f64,
    v2sigma2: *mut f64, v2sigmalapl: *mut f64, v2sigmatau: *mut f64, v2lapl2: *mut f64,
    v2lapltau: *mut f64, v2tau2: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_vxc_fxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            vrho, vsigma, vlapl, vtau,
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Fxc, output) }
    })
}

/// `void xc_mgga_fxc(...)` — order 2 only.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_fxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    v2rho2: *mut f64, v2rhosigma: *mut f64, v2rholapl: *mut f64, v2rhotau: *mut f64,
    v2sigma2: *mut f64, v2sigmalapl: *mut f64, v2sigmatau: *mut f64, v2lapl2: *mut f64,
    v2lapltau: *mut f64, v2tau2: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_fxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Fxc, output) }
    })
}

/// `void xc_mgga_exc_vxc_fxc_kxc(...)` — orders 0,1,2,3.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_exc_vxc_fxc_kxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    zk: *mut f64, vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64,
    v2rho2: *mut f64, v2rhosigma: *mut f64, v2rholapl: *mut f64, v2rhotau: *mut f64,
    v2sigma2: *mut f64, v2sigmalapl: *mut f64, v2sigmatau: *mut f64, v2lapl2: *mut f64,
    v2lapltau: *mut f64, v2tau2: *mut f64,
    v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rho2lapl: *mut f64, v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64, v3rhosigmalapl: *mut f64, v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64, v3rholapltau: *mut f64, v3rhotau2: *mut f64, v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64, v3sigma2tau: *mut f64, v3sigmalapl2: *mut f64, v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64, v3lapl3: *mut f64, v3lapl2tau: *mut f64, v3lapltau2: *mut f64,
    v3tau3: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_exc_vxc_fxc_kxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            zk, vrho, vsigma, vlapl, vtau,
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2,
            v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau,
            v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2,
            v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Kxc, output) }
    })
}

/// `void xc_mgga_vxc_fxc_kxc(...)` — orders 1,2,3.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_vxc_fxc_kxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64,
    v2rho2: *mut f64, v2rhosigma: *mut f64, v2rholapl: *mut f64, v2rhotau: *mut f64,
    v2sigma2: *mut f64, v2sigmalapl: *mut f64, v2sigmatau: *mut f64, v2lapl2: *mut f64,
    v2lapltau: *mut f64, v2tau2: *mut f64,
    v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rho2lapl: *mut f64, v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64, v3rhosigmalapl: *mut f64, v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64, v3rholapltau: *mut f64, v3rhotau2: *mut f64, v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64, v3sigma2tau: *mut f64, v3sigmalapl2: *mut f64, v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64, v3lapl3: *mut f64, v3lapl2tau: *mut f64, v3lapltau2: *mut f64,
    v3tau3: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_vxc_fxc_kxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            vrho, vsigma, vlapl, vtau,
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2,
            v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau,
            v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2,
            v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Kxc, output) }
    })
}

/// `void xc_mgga_kxc(...)` — order 3 only.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_kxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rho2lapl: *mut f64, v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64, v3rhosigmalapl: *mut f64, v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64, v3rholapltau: *mut f64, v3rhotau2: *mut f64, v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64, v3sigma2tau: *mut f64, v3sigmalapl2: *mut f64, v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64, v3lapl3: *mut f64, v3lapl2tau: *mut f64, v3lapltau2: *mut f64,
    v3tau3: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_kxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau,
            v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2,
            v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Kxc, output) }
    })
}

/// `void xc_mgga_lxc(...)` — order 4 only (35 fields).
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_mgga_lxc(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    v4rho4: *mut f64, v4rho3sigma: *mut f64, v4rho3lapl: *mut f64, v4rho3tau: *mut f64, v4rho2sigma2: *mut f64,
    v4rho2sigmalapl: *mut f64, v4rho2sigmatau: *mut f64, v4rho2lapl2: *mut f64, v4rho2lapltau: *mut f64,
    v4rho2tau2: *mut f64, v4rhosigma3: *mut f64, v4rhosigma2lapl: *mut f64, v4rhosigma2tau: *mut f64,
    v4rhosigmalapl2: *mut f64, v4rhosigmalapltau: *mut f64, v4rhosigmatau2: *mut f64,
    v4rholapl3: *mut f64, v4rholapl2tau: *mut f64, v4rholapltau2: *mut f64, v4rhotau3: *mut f64,
    v4sigma4: *mut f64, v4sigma3lapl: *mut f64, v4sigma3tau: *mut f64, v4sigma2lapl2: *mut f64,
    v4sigma2lapltau: *mut f64, v4sigma2tau2: *mut f64, v4sigmalapl3: *mut f64, v4sigmalapl2tau: *mut f64,
    v4sigmalapltau2: *mut f64, v4sigmatau3: *mut f64, v4lapl4: *mut f64, v4lapl3tau: *mut f64,
    v4lapl2tau2: *mut f64, v4lapltau3: *mut f64, v4tau4: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga_lxc", {
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            v4rho4, v4rho3sigma, v4rho3lapl, v4rho3tau, v4rho2sigma2, v4rho2sigmalapl, v4rho2sigmatau,
            v4rho2lapl2, v4rho2lapltau, v4rho2tau2, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau,
            v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2,
            v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2,
            v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2,
            v4lapltau3, v4tau4);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, DerivativeOrder::Lxc, output) }
    })
}

/// `void xc_mgga(p, np, rho, sigma, lapl, tau, zk … v4tau4);` — family-summary,
/// order inferred from highest non-NULL output (Pitfall 8). All 70 output pointers.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
// This entry point specifies every MggaOutput field, so the macro's trailing
// `..Default::default()` is a no-op here (harmless; the macro is shared with the
// partial-field entry points above).
#[allow(clippy::needless_update)]
pub unsafe extern "C" fn xc_mgga(
    p: *const xc_func_type, np: usize, rho: *const f64, sigma: *const f64, lapl: *const f64, tau: *const f64,
    zk: *mut f64, vrho: *mut f64, vsigma: *mut f64, vlapl: *mut f64, vtau: *mut f64,
    v2rho2: *mut f64, v2rhosigma: *mut f64, v2rholapl: *mut f64, v2rhotau: *mut f64,
    v2sigma2: *mut f64, v2sigmalapl: *mut f64, v2sigmatau: *mut f64, v2lapl2: *mut f64,
    v2lapltau: *mut f64, v2tau2: *mut f64,
    v3rho3: *mut f64, v3rho2sigma: *mut f64, v3rho2lapl: *mut f64, v3rho2tau: *mut f64,
    v3rhosigma2: *mut f64, v3rhosigmalapl: *mut f64, v3rhosigmatau: *mut f64,
    v3rholapl2: *mut f64, v3rholapltau: *mut f64, v3rhotau2: *mut f64, v3sigma3: *mut f64,
    v3sigma2lapl: *mut f64, v3sigma2tau: *mut f64, v3sigmalapl2: *mut f64, v3sigmalapltau: *mut f64,
    v3sigmatau2: *mut f64, v3lapl3: *mut f64, v3lapl2tau: *mut f64, v3lapltau2: *mut f64,
    v3tau3: *mut f64,
    v4rho4: *mut f64, v4rho3sigma: *mut f64, v4rho3lapl: *mut f64, v4rho3tau: *mut f64, v4rho2sigma2: *mut f64,
    v4rho2sigmalapl: *mut f64, v4rho2sigmatau: *mut f64, v4rho2lapl2: *mut f64, v4rho2lapltau: *mut f64,
    v4rho2tau2: *mut f64, v4rhosigma3: *mut f64, v4rhosigma2lapl: *mut f64, v4rhosigma2tau: *mut f64,
    v4rhosigmalapl2: *mut f64, v4rhosigmalapltau: *mut f64, v4rhosigmatau2: *mut f64,
    v4rholapl3: *mut f64, v4rholapl2tau: *mut f64, v4rholapltau2: *mut f64, v4rhotau3: *mut f64,
    v4sigma4: *mut f64, v4sigma3lapl: *mut f64, v4sigma3tau: *mut f64, v4sigma2lapl2: *mut f64,
    v4sigma2lapltau: *mut f64, v4sigma2tau2: *mut f64, v4sigmalapl3: *mut f64, v4sigmalapl2tau: *mut f64,
    v4sigmalapltau2: *mut f64, v4sigmatau3: *mut f64, v4lapl4: *mut f64, v4lapl3tau: *mut f64,
    v4lapl2tau2: *mut f64, v4lapltau3: *mut f64, v4tau4: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_mgga", {
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
            return Ok(0);
        };
        let dims = unsafe { mgga_dims(p) }?;
        let output = mgga_out!(np, dims;
            zk, vrho, vsigma, vlapl, vtau,
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2,
            v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau,
            v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2,
            v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3,
            v4rho4, v4rho3sigma, v4rho3lapl, v4rho3tau, v4rho2sigma2, v4rho2sigmalapl, v4rho2sigmatau,
            v4rho2lapl2, v4rho2lapltau, v4rho2tau2, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau,
            v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2,
            v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2,
            v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2,
            v4lapltau3, v4tau4);
        unsafe { mgga_run(p, np, rho, sigma, lapl, tau, order, output) }
    })
}

#[cfg(test)]
mod mgga_evaluate_tests {
    use super::*;
    use crate::raw_handle::*;

    #[test]
    fn mgga_exc_smoke() {
        unsafe {
            let p = xc_func_alloc();
            let mgga_id = libxc_core::registry::lookup_by_name("mgga_x_tpss")
                .expect("mgga_x_tpss in registry")
                .raw() as i32;
            assert_eq!(xc_func_init(p, mgga_id, 1), 0);
            let rho = [0.1f64, 0.2, 0.3, 0.4];
            let sigma = [0.01f64, 0.02, 0.03, 0.04];
            let lapl = [0.0f64; 4];
            let tau = [0.05f64, 0.06, 0.07, 0.08];
            let mut zk = [0.0f64; 4];
            assert_eq!(
                xc_mgga_exc(p, 4, rho.as_ptr(), sigma.as_ptr(), lapl.as_ptr(), tau.as_ptr(), zk.as_mut_ptr()),
                0
            );
            for v in &zk {
                assert!(*v < 0.0, "MGGA exc must be negative; got {v}");
            }
            xc_func_end(p);
            xc_func_free(p);
        }
    }
}
