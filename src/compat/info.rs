//! `xc_func_info_get_*` and `xc_func_reference_get_*` accessors (06-02b-T1).
//!
//! The opaque `*const xc_func_info_type` secretly points at a
//! `&'static FunctionalMeta`; `*const func_reference_type` at a
//! `&'static Reference`. Both are cast back here.

#![allow(clippy::missing_safety_doc)]

use crate::compat::c_layout::{func_reference_type, xc_func_info_type};
use crate::compat::errno::cache_cstring;
use crate::meta::{FunctionalMeta, Reference};
use std::ffi::c_char;

unsafe fn info_ref<'a>(info: *const xc_func_info_type) -> Option<&'a FunctionalMeta> {
    if info.is_null() {
        None
    } else {
        // SAFETY: non-null info pointers originate from `xc_func_get_info`, which
        // returns `&'static FunctionalMeta` cast to `*const xc_func_info_type`.
        Some(unsafe { &*(info as *const FunctionalMeta) })
    }
}

unsafe fn ref_ref<'a>(r: *const func_reference_type) -> Option<&'a Reference> {
    if r.is_null() {
        None
    } else {
        // SAFETY: non-null reference pointers originate from
        // `xc_func_info_get_references`, which casts `&'static Reference`.
        Some(unsafe { &*(r as *const Reference) })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_number(info: *const xc_func_info_type) -> i32 {
    if let Some(m) = unsafe { info_ref(info) } { m.id.raw() as i32 } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_kind(info: *const xc_func_info_type) -> i32 {
    if let Some(m) = unsafe { info_ref(info) } { m.kind as i32 } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_name(info: *const xc_func_info_type) -> *const c_char {
    if let Some(m) = unsafe { info_ref(info) } { cache_cstring(m.name) } else { std::ptr::null() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_family(info: *const xc_func_info_type) -> i32 {
    if let Some(m) = unsafe { info_ref(info) } { m.family as i32 } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_flags(info: *const xc_func_info_type) -> i32 {
    if let Some(m) = unsafe { info_ref(info) } { m.flags.bits() as i32 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_n_ext_params(info: *const xc_func_info_type) -> i32 {
    if let Some(m) = unsafe { info_ref(info) } { m.ext_params.len() as i32 } else { -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_ext_params_name(
    info: *const xc_func_info_type,
    number: i32,
) -> *const c_char {
    if let Some(m) = unsafe { info_ref(info) }
        && number >= 0
        && (number as usize) < m.ext_params.len()
    {
        return cache_cstring(m.ext_params[number as usize].name);
    }
    std::ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_ext_params_description(
    info: *const xc_func_info_type,
    number: i32,
) -> *const c_char {
    if let Some(m) = unsafe { info_ref(info) }
        && number >= 0
        && (number as usize) < m.ext_params.len()
    {
        return cache_cstring(m.ext_params[number as usize].description);
    }
    std::ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_ext_params_default_value(
    info: *const xc_func_info_type,
    number: i32,
) -> f64 {
    if let Some(m) = unsafe { info_ref(info) }
        && number >= 0
        && (number as usize) < m.ext_params.len()
    {
        return m.ext_params[number as usize].default_value;
    }
    f64::NAN
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_references(
    info: *const xc_func_info_type,
    number: i32,
) -> *const func_reference_type {
    if let Some(m) = unsafe { info_ref(info) }
        && number >= 0
        && (number as usize) < m.references.len()
    {
        return &m.references[number as usize] as *const Reference as *const func_reference_type;
    }
    std::ptr::null()
}

// 4 reference accessors — Reference field names verified against src/meta/mod.rs
// (citation / doi / bibtex / key, all &'static str).

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_reference_get_ref(r: *const func_reference_type) -> *const c_char {
    if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.citation) } else { std::ptr::null() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_reference_get_doi(r: *const func_reference_type) -> *const c_char {
    if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.doi) } else { std::ptr::null() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_reference_get_bibtex(r: *const func_reference_type) -> *const c_char {
    if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.bibtex) } else { std::ptr::null() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_reference_get_key(r: *const func_reference_type) -> *const c_char {
    if let Some(rr) = unsafe { ref_ref(r) } { cache_cstring(rr.key) } else { std::ptr::null() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compat::raw_handle::*;
    use std::ffi::CStr;

    #[test]
    fn info_get_name_returns_cached_cstring() {
        unsafe {
            let p = xc_func_alloc();
            assert_eq!(xc_func_init(p, 1, 1), 0); // lda_x
            let info = xc_func_get_info(p);
            assert!(!info.is_null());
            let name = xc_func_info_get_name(info);
            let s = CStr::from_ptr(name).to_string_lossy();
            assert_eq!(s, "lda_x");
            assert_eq!(xc_func_info_get_number(info), 1);
            assert_eq!(xc_func_info_get_family(info), 1);
            xc_func_end(p);
            xc_func_free(p);
        }
    }
}
