//! Library version and reference functions (06-02b-T2).

#![allow(clippy::missing_safety_doc)]

use crate::registry;
use std::ffi::{c_char, CStr};

static VERSION_STRING: &CStr = c"7.0.0";
static REFERENCE: &CStr = c"libxc_rs: Rust reimplementation of libxc 7.0.0";
static REFERENCE_DOI: &CStr = c"10.1016/j.softx.2017.11.002"; // Lehtola 2018 SoftwareX
static REFERENCE_KEY: &CStr = c"Lehtola2018";

/// `void xc_version(int *major, int *minor, int *micro);`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_version(major: *mut i32, minor: *mut i32, micro: *mut i32) {
    let (ma, mi, mc) = registry::version();
    if !major.is_null() {
        // SAFETY: caller contract — writable int.
        unsafe { *major = ma as i32; }
    }
    if !minor.is_null() {
        // SAFETY: caller contract — writable int.
        unsafe { *minor = mi as i32; }
    }
    if !micro.is_null() {
        // SAFETY: caller contract — writable int.
        unsafe { *micro = mc as i32; }
    }
}

/// `const char *xc_version_string(void);`
#[unsafe(no_mangle)]
pub extern "C" fn xc_version_string() -> *const c_char {
    VERSION_STRING.as_ptr()
}

/// `const char *xc_reference(void);`
#[unsafe(no_mangle)]
pub extern "C" fn xc_reference() -> *const c_char {
    REFERENCE.as_ptr()
}

/// `const char *xc_reference_doi(void);`
#[unsafe(no_mangle)]
pub extern "C" fn xc_reference_doi() -> *const c_char {
    REFERENCE_DOI.as_ptr()
}

/// `const char *xc_reference_key(void);`
#[unsafe(no_mangle)]
pub extern "C" fn xc_reference_key() -> *const c_char {
    REFERENCE_KEY.as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_writes_components() {
        let mut ma: i32 = 0;
        let mut mi: i32 = 0;
        let mut mc: i32 = 0;
        unsafe {
            xc_version(&mut ma, &mut mi, &mut mc);
        }
        assert_eq!((ma, mi, mc), (7, 0, 0));
    }

    #[test]
    fn version_string_matches() {
        unsafe {
            let s = CStr::from_ptr(xc_version_string()).to_string_lossy();
            assert_eq!(s, "7.0.0");
        }
    }

    #[test]
    fn reference_strings_present() {
        unsafe {
            assert!(!CStr::from_ptr(xc_reference()).to_bytes().is_empty());
            let key = CStr::from_ptr(xc_reference_key()).to_string_lossy();
            assert_eq!(key, "Lehtola2018");
        }
    }
}
