//! Discovery functions: id ↔ name lookup, family classification, listing.
//! Wraps `src/registry/mod.rs` 1:1 (06-02b-T1).

#![allow(clippy::missing_safety_doc)]

use crate::errno::{self, cache_cstring, set_error};
use crate::extern_c_wrapper;
use libxc_core::registry;
use std::ffi::{c_char, CStr};

/// `int xc_functional_get_number(const char *name);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_functional_get_number(name: *const c_char) -> i32 {
    if name.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_functional_get_number: null name");
        return errno::LIBXC_RS_NULL_HANDLE;
    }
    extern_c_wrapper!(_, "xc_functional_get_number", {
        // SAFETY: name non-null; caller contract = valid C string.
        let s = unsafe { CStr::from_ptr(name) }
            .to_str()
            .map_err(|_| libxc_core::error::LibxcRsError::UnknownFunctionalName("non-utf8".into()))?;
        let id = registry::lookup_by_name(s)?;
        Ok(id.raw() as i32)
    })
}

/// `const char *xc_functional_get_name(int number);` — pointer into thread-local CString cache.
/// Lifetime: pointer remains valid across subsequent cache_cstring calls (HashMap stability).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_functional_get_name(number: i32) -> *const c_char {
    let result = std::panic::catch_unwind(|| {
        if number < 0 || number > u16::MAX as i32 {
            return None;
        }
        registry::lookup_by_id(number as u16).ok().map(|m| m.name)
    });
    match result {
        Ok(Some(name)) => cache_cstring(name),
        Ok(None) => {
            set_error(
                errno::LIBXC_RS_UNKNOWN_FUNCTIONAL_ID,
                &format!("xc_functional_get_name: unknown id {number}"),
            );
            std::ptr::null()
        }
        Err(_) => {
            set_error(errno::LIBXC_RS_PANIC, "xc_functional_get_name: panic");
            std::ptr::null()
        }
    }
}

/// `int xc_family_from_id(int id, int *family, int *number);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_family_from_id(id: i32, family: *mut i32, number: *mut i32) -> i32 {
    extern_c_wrapper!(_, "xc_family_from_id", {
        if id < 0 || id > u16::MAX as i32 {
            return Err(libxc_core::error::LibxcRsError::UnknownFunctionalId(0));
        }
        let meta = registry::lookup_by_id(id as u16)?;
        if !family.is_null() {
            // SAFETY: caller contract — `family` points to a writable int.
            unsafe { *family = meta.family as i32; }
        }
        if !number.is_null() {
            // SAFETY: caller contract — `number` points to a writable int.
            unsafe { *number = id; }
        }
        Ok(0)
    })
}

/// `int xc_number_of_functionals(void);`
#[unsafe(no_mangle)]
pub extern "C" fn xc_number_of_functionals() -> i32 {
    registry::functional_count() as i32
}

/// `int xc_maximum_name_length(void);`
#[unsafe(no_mangle)]
pub extern "C" fn xc_maximum_name_length() -> i32 {
    registry::max_name_length() as i32
}

/// `void xc_available_functional_numbers(int *list);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_available_functional_numbers(list: *mut i32) {
    if list.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_available_functional_numbers: null list");
        return;
    }
    let _ = std::panic::catch_unwind(|| {
        let count = registry::functional_count();
        // SAFETY: caller contract — `list` holds `xc_number_of_functionals()` ints.
        let slice = unsafe { std::slice::from_raw_parts_mut(list, count) };
        for (i, fid) in registry::all_functional_ids().enumerate() {
            if i < count {
                slice[i] = fid.raw() as i32;
            }
        }
    });
}

/// `void xc_available_functional_numbers_by_name(int *list);` — sorted alphabetically by name.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_available_functional_numbers_by_name(list: *mut i32) {
    if list.is_null() {
        set_error(
            errno::LIBXC_RS_NULL_HANDLE,
            "xc_available_functional_numbers_by_name: null list",
        );
        return;
    }
    let _ = std::panic::catch_unwind(|| {
        let count = registry::functional_count();
        // SAFETY: caller contract — `list` holds `xc_number_of_functionals()` ints.
        let slice = unsafe { std::slice::from_raw_parts_mut(list, count) };
        let mut pairs: Vec<(&'static str, u16)> = registry::all_functional_ids()
            .map(|fid| {
                let m = registry::lookup_by_id(fid.raw()).expect("registered id must lookup");
                (m.name, fid.raw())
            })
            .collect();
        pairs.sort_by_key(|&(n, _)| n);
        for (i, &(_, raw)) in pairs.iter().enumerate() {
            if i < count {
                slice[i] = raw as i32;
            }
        }
    });
}

/// `void xc_available_functional_names(char **list);` — fills thread-local cached pointers.
/// Pointers stable across rehash (HashMap-keyed cache from 06-02a).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_available_functional_names(list: *mut *mut c_char) {
    if list.is_null() {
        set_error(errno::LIBXC_RS_NULL_HANDLE, "xc_available_functional_names: null list");
        return;
    }
    let _ = std::panic::catch_unwind(|| {
        let count = registry::functional_count();
        // SAFETY: caller contract — `list` holds `xc_number_of_functionals()` char* slots.
        let slice = unsafe { std::slice::from_raw_parts_mut(list, count) };
        for (i, fid) in registry::all_functional_ids().enumerate() {
            if i >= count {
                break;
            }
            let m = registry::lookup_by_id(fid.raw()).expect("registered id must lookup");
            slice[i] = cache_cstring(m.name) as *mut c_char;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_matches_registry() {
        assert_eq!(xc_number_of_functionals(), 649);
        let name = std::ffi::CString::new("lda_x").unwrap();
        unsafe {
            let id = xc_functional_get_number(name.as_ptr());
            assert!(id > 0, "lda_x lookup must return positive id; got {id}");
            let p = xc_functional_get_name(id);
            assert!(!p.is_null());
            let s = CStr::from_ptr(p).to_string_lossy();
            assert_eq!(s, "lda_x");
            let mut family = 0i32;
            let mut number = 0i32;
            assert_eq!(xc_family_from_id(id, &mut family, &mut number), 0);
            assert_eq!(family, 1);
        }
    }

    #[test]
    fn available_names_fills_649() {
        let count = xc_number_of_functionals() as usize;
        let mut buf: Vec<*mut c_char> = vec![std::ptr::null_mut(); count];
        unsafe {
            xc_available_functional_names(buf.as_mut_ptr());
        }
        for (i, p) in buf.iter().enumerate() {
            assert!(!p.is_null(), "slot {i} null");
            let s = unsafe { CStr::from_ptr(*p).to_string_lossy() };
            assert!(!s.is_empty(), "slot {i} empty");
        }
    }
}
