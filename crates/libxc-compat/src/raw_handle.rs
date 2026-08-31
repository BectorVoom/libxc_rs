//! Lifecycle (alloc/init/end/free/get_info) and the `FunctionalSlot` state machine.
//!
//! `xc_func_type*` is `Box<FunctionalSlot>::into_raw()` cast to opaque.
//! All `Box::into_raw` / `Box::from_raw` / `std::ptr::replace` live here.
//!
//! Per CONTEXT D-A1-1 / D-A1-2 / D-A1-3.

#![allow(clippy::missing_safety_doc)]

use libxc_core::error::LibxcRsError;
use crate::c_layout::{xc_func_info_type, xc_func_type};
use crate::errno::{self, set_error};
use crate::extern_c_wrapper;
use libxc_eval::functional::Functional;
use libxc_core::model::{FunctionalId, Spin};

/// Two-state slot: `Empty` (allocated but not initialized) or `Initialized(Functional)`.
///
/// Re-init replaces the inner `Functional`, dropping the old one (Pitfall 1).
#[repr(C)]
#[allow(clippy::large_enum_variant)]
pub enum FunctionalSlot {
    Empty,
    Initialized(Functional),
}

impl FunctionalSlot {
    /// Read-only access.
    ///
    /// # Safety
    /// Caller asserts `p` is non-null and points to a valid `Box<FunctionalSlot>`
    /// returned by [`xc_func_alloc`] (the wrapper macro NULL-checks before this is called).
    pub(crate) unsafe fn as_initialized_const<'a>(
        p: *const xc_func_type,
    ) -> Result<&'a Functional, LibxcRsError> {
        // SAFETY: caller's contract.
        let slot: &FunctionalSlot = unsafe { &*(p as *const FunctionalSlot) };
        match slot {
            FunctionalSlot::Initialized(f) => Ok(f),
            FunctionalSlot::Empty => Err(LibxcRsError::UninitializedHandle),
        }
    }

    /// Mutable access.
    ///
    /// # Safety
    /// Same contract as [`as_initialized_const`]. The caller must guarantee
    /// no aliasing references to the slot exist for the duration of the
    /// returned borrow (CONTEXT D-A1-3 — single-threaded per handle).
    //
    // Intended C-ABI compat API: mutable counterpart to `as_initialized_const`,
    // not yet wired to a C entry point (no mutable C op exists yet). Retained so
    // the path is ready when one lands; allow under crate `#![deny(warnings)]`.
    #[allow(dead_code)]
    pub(crate) unsafe fn as_initialized_mut<'a>(
        p: *mut xc_func_type,
    ) -> Result<&'a mut Functional, LibxcRsError> {
        // SAFETY: caller's contract.
        let slot: &mut FunctionalSlot = unsafe { &mut *(p as *mut FunctionalSlot) };
        match slot {
            FunctionalSlot::Initialized(f) => Ok(f),
            FunctionalSlot::Empty => Err(LibxcRsError::UninitializedHandle),
        }
    }
}

// === Lifecycle ===

/// `xc_func_type *xc_func_alloc();` — allocates an empty slot.
///
/// Caller must release with [`xc_func_free`].
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub extern "C" fn xc_func_alloc() -> *mut xc_func_type {
    Box::into_raw(Box::new(FunctionalSlot::Empty)) as *mut xc_func_type
}

/// `int xc_func_init(xc_func_type *p, int functional, int nspin);`
///
/// Initializes the slot. Re-init replaces the prior `Functional` via
/// `std::ptr::replace`, dropping the old value (Pitfall 1).
///
/// # Safety
/// Caller must pass a pointer obtained from [`xc_func_alloc`].
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_init(
    p: *mut xc_func_type,
    functional: i32,
    nspin: i32,
) -> i32 {
    extern_c_wrapper!(p, "xc_func_init", {
        if functional < 0 || functional > u16::MAX as i32 {
            return Err(LibxcRsError::UnknownFunctionalId(0));
        }
        let id = FunctionalId::from_raw(functional as u16)?;
        let spin = match nspin {
            1 => Spin::Unpolarized,
            2 => Spin::Polarized,
            other => return Err(LibxcRsError::InvalidSpin(other)),
        };
        let f = Functional::new(id, spin)?;
        // SAFETY: p is non-null (wrapper macro NULL-checked). std::ptr::replace
        // drops the previous slot value, preventing leaks on re-init (Pitfall 1).
        unsafe {
            let _ = std::ptr::replace(p as *mut FunctionalSlot, FunctionalSlot::Initialized(f));
        }
        Ok(0)
    })
}

/// `int xc_func_end(xc_func_type *p);` — resets to `Empty`, dropping the inner `Functional`.
///
/// libxc's `xc_func_end` is `void`; we widen to `int` per CONTEXT D-A4-1 so
/// callers can observe NULL-handle / panic errors uniformly.
///
/// # Safety
/// Caller must pass a pointer obtained from [`xc_func_alloc`].
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_end(p: *mut xc_func_type) -> i32 {
    extern_c_wrapper!(p, "xc_func_end", {
        // SAFETY: p is non-null (wrapper macro NULL-checked). std::ptr::replace
        // drops the previous slot value (Pitfall 1 — symmetric with init).
        unsafe {
            let _ = std::ptr::replace(p as *mut FunctionalSlot, FunctionalSlot::Empty);
        }
        Ok(0)
    })
}

/// `void xc_func_free(xc_func_type *p);` — frees the `Box`.
///
/// # Safety
/// Caller must pass a pointer obtained from [`xc_func_alloc`]. After this call
/// the pointer is dangling; do not use it again.
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_free(p: *mut xc_func_type) {
    if p.is_null() {
        return;
    }
    // SAFETY: p obtained from xc_func_alloc (caller contract). Reconstructing
    // the Box drops the inner FunctionalSlot (which itself drops the contained
    // Functional, if Initialized).
    unsafe {
        drop(Box::from_raw(p as *mut FunctionalSlot));
    }
}

/// `const xc_func_info_type *xc_func_get_info(const xc_func_type *p);`
///
/// Returns `&'static FunctionalMeta` cast to `*const xc_func_info_type`,
/// or NULL on Empty / panic / NULL handle.
///
/// # Safety
/// Caller must pass a pointer obtained from [`xc_func_alloc`].
#[cfg_attr(feature = "c-abi", unsafe(no_mangle))]
pub unsafe extern "C" fn xc_func_get_info(p: *const xc_func_type) -> *const xc_func_info_type {
    if p.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_func_get_info: null handle");
        return std::ptr::null();
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: p is non-null and per caller contract points to a valid
        // Box<FunctionalSlot>.
        unsafe { FunctionalSlot::as_initialized_const(p) }
            .map(|f| f.meta() as *const libxc_core::meta::FunctionalMeta as *const xc_func_info_type)
    }));
    match result {
        Ok(Ok(info)) => info,
        Ok(Err(_)) => {
            set_error(
                errno::LIBXC_RS_UNINITIALIZED_HANDLE,
                "xc_func_get_info: handle uninitialized",
            );
            std::ptr::null()
        }
        Err(_) => {
            set_error(errno::LIBXC_RS_PANIC, "xc_func_get_info: panic");
            std::ptr::null()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errno::xc_rs_last_error_code;

    #[test]
    fn lifecycle_round_trip() {
        unsafe {
            let p = xc_func_alloc();
            assert!(!p.is_null());
            // id=1 (lda_x) is guaranteed to exist; nspin=1 unpolarized.
            let rc = xc_func_init(p, 1, 1);
            assert_eq!(rc, 0, "init failed: code={}", xc_rs_last_error_code());
            assert_eq!(xc_func_end(p), 0);
            xc_func_free(p);
        }
    }

    #[test]
    fn reinit_drops_previous() {
        // Pitfall 1 — re-initializing the same slot must drop the previous
        // Functional rather than leak it.
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 1), 0);
            // Re-init with id=2 (lda_c_wigner) — previous lda_x must be dropped
            // by std::ptr::replace.
            assert_eq!(xc_func_init(p, 2, 1), 0);
            let info = xc_func_get_info(p);
            assert!(!info.is_null());
            xc_func_end(p);
            xc_func_free(p);
        }
    }

    #[test]
    fn null_handle_returns_null_handle_errno() {
        unsafe {
            let rc = xc_func_init(std::ptr::null_mut(), 1, 1);
            assert_eq!(rc, errno::LIBXC_RS_NULL_HANDLE);
            assert_eq!(xc_rs_last_error_code(), errno::LIBXC_RS_NULL_HANDLE);
        }
    }

    #[test]
    fn invalid_spin_returns_invalid_spin_errno() {
        unsafe {
            let p = xc_func_alloc();
            // nspin=7 is invalid (only 1 and 2 are accepted).
            let rc = xc_func_init(p, 1, 7);
            assert_eq!(
                rc,
                errno::LIBXC_RS_INVALID_SPIN,
                "expected LIBXC_RS_INVALID_SPIN ({}); got {}",
                errno::LIBXC_RS_INVALID_SPIN,
                rc
            );
            assert_eq!(xc_rs_last_error_code(), errno::LIBXC_RS_INVALID_SPIN);
            let msg = errno::xc_rs_last_error_message();
            let s = std::ffi::CStr::from_ptr(msg).to_string_lossy();
            assert!(
                s.contains('7'),
                "InvalidSpin message must mention the bad value: {s}"
            );
            xc_func_free(p);
        }
    }

    #[test]
    fn invalid_spin_zero_returns_invalid_spin_errno() {
        // nspin=0 is also invalid.
        unsafe {
            let p = xc_func_alloc();
            let rc = xc_func_init(p, 1, 0);
            assert_eq!(rc, errno::LIBXC_RS_INVALID_SPIN);
            xc_func_free(p);
        }
    }

    #[test]
    fn uninitialized_handle_get_info_returns_null() {
        unsafe {
            let p = xc_func_alloc();
            let info = xc_func_get_info(p);
            assert!(info.is_null());
            assert_eq!(
                xc_rs_last_error_code(),
                errno::LIBXC_RS_UNINITIALIZED_HANDLE
            );
            xc_func_free(p);
        }
    }

    #[test]
    fn null_handle_get_info_returns_null() {
        unsafe {
            let info = xc_func_get_info(std::ptr::null());
            assert!(info.is_null());
            assert_eq!(xc_rs_last_error_code(), errno::LIBXC_RS_NULL_HANDLE);
        }
    }

    #[test]
    fn free_null_is_noop() {
        // libxc's contract: free(NULL) is a no-op (matches free(3) semantics).
        unsafe {
            xc_func_free(std::ptr::null_mut());
        }
    }

    #[test]
    fn unknown_functional_id_returns_errno() {
        unsafe {
            let p = xc_func_alloc();
            // id=9999 is well above the 649 valid functionals.
            let rc = xc_func_init(p, 9999, 1);
            assert!(rc < 0, "expected error; got {rc}");
            xc_func_free(p);
        }
    }

    #[test]
    fn negative_functional_id_returns_errno() {
        unsafe {
            let p = xc_func_alloc();
            let rc = xc_func_init(p, -1, 1);
            assert!(rc < 0, "expected error; got {rc}");
            xc_func_free(p);
        }
    }

    #[test]
    fn end_after_init_succeeds_then_init_again() {
        // init → end → init round-trip. Symmetric with reinit_drops_previous,
        // but exercises the explicit Empty intermediate step.
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 1), 0);
            assert_eq!(xc_func_end(p), 0);
            // After end the slot is Empty; get_info must return NULL.
            assert!(xc_func_get_info(p).is_null());
            // Re-init must succeed.
            assert_eq!(xc_func_init(p, 1, 2), 0);
            xc_func_end(p);
            xc_func_free(p);
        }
    }

    #[test]
    fn polarized_init_succeeds() {
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 2), 0);
            xc_func_end(p);
            xc_func_free(p);
        }
    }
}
