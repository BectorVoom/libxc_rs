//! Thread-local errno + extern "C" accessors + cache_cstring helper for the libxc_rs compat layer.
//!
//! Every fallible extern "C" function on the C ABI returns an `int` (negative
//! for error). Caller can then call `xc_rs_last_error_code()` /
//! `xc_rs_last_error_message()` to retrieve the typed discriminant + the
//! Display-formatted error message for the most recent error on this thread.

use crate::error::LibxcRsError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::pin::Pin;

// === 25 errno constants — mirror LibxcRsError::discriminant() table ===

pub const LIBXC_RS_OK:                              i32 =   0;
pub const LIBXC_RS_PANIC:                           i32 =  -1;
pub const LIBXC_RS_NULL_HANDLE:                     i32 =  -2;
pub const LIBXC_RS_UNINITIALIZED_HANDLE:            i32 =  -3;
pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_ID:           i32 =  -4;
pub const LIBXC_RS_UNKNOWN_FUNCTIONAL_NAME:         i32 =  -5;
pub const LIBXC_RS_REMOVED_FUNCTIONAL_ID:           i32 =  -6;
pub const LIBXC_RS_UNKNOWN_EXT_PARAM_NAME:          i32 =  -7;
pub const LIBXC_RS_EXT_PARAM_INDEX_OUT_OF_RANGE:    i32 =  -8;
pub const LIBXC_RS_EXT_PARAM_COUNT_MISMATCH:        i32 =  -9;
pub const LIBXC_RS_FAMILY_MISMATCH:                 i32 = -10;
pub const LIBXC_RS_SPIN_MISMATCH:                   i32 = -11;
pub const LIBXC_RS_INPUT_BUFFER_SIZE_MISMATCH:      i32 = -12;
pub const LIBXC_RS_OUTPUT_BUFFER_SIZE_MISMATCH:     i32 = -13;
pub const LIBXC_RS_BATCH_OVERFLOW:                  i32 = -14;
pub const LIBXC_RS_UNSUPPORTED_DERIVATIVE_ORDER:    i32 = -15;
pub const LIBXC_RS_UNSUPPORTED_FUNCTIONAL:          i32 = -16;
pub const LIBXC_RS_EXT_PARAM_NOT_FOUND:             i32 = -17;
pub const LIBXC_RS_GPU_NOT_AVAILABLE:               i32 = -18;
pub const LIBXC_RS_DEVICE_CAPABILITY_MISMATCH:      i32 = -19;
pub const LIBXC_RS_ALL_BELOW_THRESHOLD:             i32 = -20;
pub const LIBXC_RS_WORKSPACE_MISMATCH:              i32 = -21;
pub const LIBXC_RS_KERNEL_LAUNCH_FAILED:            i32 = -22;
pub const LIBXC_RS_AUXILIARY_INIT_FAILED:           i32 = -23;
pub const LIBXC_RS_PROPAGATION_CONFLICT:            i32 = -24;
pub const LIBXC_RS_INVALID_SPIN:                    i32 = -25;

thread_local! {
    static LAST_ERROR: RefCell<Option<(i32, CString)>> = const { RefCell::new(None) };
    // HashMap-keyed cache: per-thread, indexed by `&'static str` (i.e. the
    // FunctionalMeta name slice's pointer + length identity is stable, so
    // hashing/eq on the &str works as expected). Storing `Pin<Box<CString>>`
    // ensures the heap address backing `as_ptr()` does NOT move when the
    // HashMap rehashes (Box's heap allocation is itself stable; Pin just
    // documents the invariant — alternative: store `Box<CString>` and use
    // `as_ptr()`, equivalent semantics).
    static CSTRING_CACHE: RefCell<HashMap<&'static str, Pin<Box<CString>>>> =
        RefCell::new(HashMap::new());
}

static EMPTY_CSTRING: &std::ffi::CStr = c"";

/// Set the thread-local errno code + message. Called by `extern_c_wrapper!`
/// on every Err / panic path.
pub fn set_error(code: i32, msg: &str) {
    let cstring = CString::new(msg).unwrap_or_else(|_| {
        // Truncate at first NUL byte (CString::new rejects interior NULs).
        let bytes: Vec<u8> = msg.bytes().take_while(|&b| b != 0).collect();
        CString::new(bytes).unwrap_or_default()
    });
    LAST_ERROR.with(|cell| *cell.borrow_mut() = Some((code, cstring)));
}

/// Map a `LibxcRsError` to its C-ABI integer discriminant. Wraps
/// [`LibxcRsError::discriminant`] for use by the wrapper macro.
pub fn discriminant(e: &LibxcRsError) -> i32 { e.discriminant() }

/// Get-or-insert a thread-local CString for a `&'static str` name.
/// The returned pointer is stable across HashMap rehashes (Box's heap
/// allocation does not move; only the HashMap's internal pointer-table moves).
/// Lifetime: valid until the thread exits OR the HashMap is explicitly cleared
/// (which we never do in production — only in tests).
///
/// Used by `compat::ids::xc_functional_get_name`, `compat::info::xc_func_info_get_*`,
/// `compat::ids::xc_available_functional_names`, etc.
pub fn cache_cstring(s: &'static str) -> *const c_char {
    CSTRING_CACHE.with(|cell| {
        let mut map = cell.borrow_mut();
        // Use `entry` to insert if missing.
        let pinned = map
            .entry(s)
            .or_insert_with(|| Pin::new(Box::new(CString::new(s).unwrap_or_default())));
        pinned.as_ref().get_ref().as_ptr()
    })
}

/// Retrieve the most recent error code on this thread, or `LIBXC_RS_OK`.
#[unsafe(no_mangle)]
pub extern "C" fn xc_rs_last_error_code() -> i32 {
    LAST_ERROR.with(|cell| {
        cell.borrow().as_ref().map(|(code, _)| *code).unwrap_or(LIBXC_RS_OK)
    })
}

/// Retrieve the most recent error message on this thread.
/// Returns a pointer to a thread-local `CString`; valid until the next
/// error-setting call on this thread. Never returns NULL — when no error
/// has been recorded, returns a static empty C string.
#[unsafe(no_mangle)]
pub extern "C" fn xc_rs_last_error_message() -> *const c_char {
    LAST_ERROR.with(|cell| match cell.borrow().as_ref() {
        Some((_, cstr)) => cstr.as_ptr(),
        None => EMPTY_CSTRING.as_ptr(),
    })
}

#[cfg(test)] mod tests {
    use super::*;
    use crate::error::LibxcRsError;

    #[test]
    fn errno_round_trip() {
        set_error(-7, "unknown ext param 'alpha'");
        assert_eq!(xc_rs_last_error_code(), -7);
        unsafe {
            let p = xc_rs_last_error_message();
            let s = std::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
            assert!(s.contains("alpha"));
        }
    }

    #[test]
    fn discriminant_uses_libxc_rs_error_method() {
        assert_eq!(discriminant(&LibxcRsError::UnknownFunctionalId(42)),  -4);
        assert_eq!(discriminant(&LibxcRsError::Panicked { message: String::new() }), -1);
        assert_eq!(discriminant(&LibxcRsError::InvalidSpin(7)), -25);
        assert_eq!(discriminant(&LibxcRsError::UninitializedHandle), -3);
    }

    /// Verify the HashMap-keyed cache is stable across ≥ 649 distinct insertions.
    /// Pre-commits the cache shape: single-slot would corrupt under load
    /// (every call to xc_available_functional_names overwrites the prior name).
    #[test]
    fn cache_cstring_holds_649_pointers_stable() {
        std::thread::spawn(|| {
            // Collect 649 distinct &'static str via the registry.
            let names: Vec<&'static str> = crate::registry::all_functional_ids()
                .filter_map(|fid| crate::registry::lookup_by_id(fid.raw()).ok().map(|m| m.name))
                .collect();
            assert!(names.len() >= 649, "registry must have ≥ 649 names; got {}", names.len());
            // Insert all 649 (or more) and snapshot pointers.
            let mut ptrs: Vec<*const c_char> = names.iter().map(|n| cache_cstring(n)).collect();
            // Insert one more (forces rehash if HashMap is near load factor).
            let extra: &'static str = "this_is_a_distinct_test_name_for_rehash_safety";
            ptrs.push(cache_cstring(extra));
            // Re-fetch every pointer; must equal the snapshot (Box heap allocation stable).
            for (i, n) in names.iter().enumerate() {
                let p_now = cache_cstring(n);
                assert_eq!(p_now, ptrs[i],
                    "pointer for name {n:?} (index {i}) moved across rehash: {:p} -> {:p}",
                    ptrs[i], p_now);
                // Bonus: pointer still resolves to the right C string.
                let s = unsafe { std::ffi::CStr::from_ptr(p_now).to_string_lossy() };
                assert_eq!(s, *n, "C string at cached pointer differs from key");
            }
        }).join().unwrap();
    }
}
