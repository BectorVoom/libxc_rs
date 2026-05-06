//! C-ABI types and layout assertions for the libxc_rs compat layer.
//!
//! Per CONTEXT D-A1-1 / D-A1-4: `xc_func_type` and `xc_func_info_type` are
//! **opaque** at the C boundary. The C header forward-declares
//! `typedef struct xc_func_type xc_func_type;` and never exposes any field;
//! the Rust pointer secretly references a `Box<FunctionalSlot>` (for
//! `xc_func_type`) or `&'static FunctionalMeta` (for `xc_func_info_type`).
//!
//! Compile-time assertions guarantee the opaque structs are zero-sized
//! and that the Rust enum discriminants match libxc's `XC_*` integer constants.

#![allow(non_camel_case_types)]

use crate::model::{Family, Kind, Spin};

/// Opaque forward-declared functional handle. C callers see `*mut xc_func_type`;
/// Rust treats it as `*mut FunctionalSlot` after pointer cast.
#[repr(C)]
pub struct xc_func_type {
    _opaque: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

/// Opaque forward-declared info handle. C callers see `*const xc_func_info_type`;
/// Rust treats it as `*const FunctionalMeta` after pointer cast.
#[repr(C)]
pub struct xc_func_info_type {
    _opaque: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

/// Opaque reference-struct handle. C callers see `*const func_reference_type`;
/// Rust treats it as `*const Reference` after pointer cast.
#[repr(C)]
pub struct func_reference_type {
    _opaque: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

// --- Compile-time layout assertions ---
const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0);
const _: () = assert!(std::mem::size_of::<xc_func_info_type>() == 0);
const _: () = assert!(std::mem::size_of::<func_reference_type>() == 0);

// libxc XC_FAMILY_* must match Rust enum repr-u8 values.
const _: () = assert!(Family::Lda  as u8 == 1);
const _: () = assert!(Family::Gga  as u8 == 2);
const _: () = assert!(Family::Mgga as u8 == 4);

// libxc XC_UNPOLARIZED / XC_POLARIZED.
const _: () = assert!(Spin::Unpolarized as u8 == 1);
const _: () = assert!(Spin::Polarized   as u8 == 2);

// libxc XC_EXCHANGE / XC_CORRELATION / XC_EXCHANGE_CORRELATION / XC_KINETIC.
const _: () = assert!(Kind::Exchange            as u8 == 0);
const _: () = assert!(Kind::Correlation         as u8 == 1);
const _: () = assert!(Kind::ExchangeCorrelation as u8 == 2);
const _: () = assert!(Kind::Kinetic             as u8 == 3);

/// libxc magic constant per Pitfall 10 (substituted with per-spec default
/// in `compat::legacy_eval::xc_func_set_ext_params`).
pub const LIBXC_EXT_PARAMS_DEFAULT: f64 = -999998888.0;

/// XC_MAX_REFERENCES — libxc-master/src/xc.h
pub const XC_MAX_REFERENCES: usize = 5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_size_zero() {
        assert_eq!(std::mem::size_of::<xc_func_type>(), 0);
        assert_eq!(std::mem::size_of::<xc_func_info_type>(), 0);
        assert_eq!(std::mem::size_of::<func_reference_type>(), 0);
    }

    #[test]
    fn repr_constants_match_libxc() {
        assert_eq!(Family::Lda  as i32, 1);
        assert_eq!(Family::Gga  as i32, 2);
        assert_eq!(Family::Mgga as i32, 4);
        assert_eq!(Spin::Unpolarized as i32, 1);
        assert_eq!(Spin::Polarized   as i32, 2);
        assert_eq!(Kind::Exchange            as i32, 0);
        assert_eq!(Kind::Correlation         as i32, 1);
        assert_eq!(Kind::ExchangeCorrelation as i32, 2);
        assert_eq!(Kind::Kinetic             as i32, 3);
    }

    #[test]
    fn ext_params_default_constant() {
        assert_eq!(LIBXC_EXT_PARAMS_DEFAULT, -999998888.0);
    }
}
