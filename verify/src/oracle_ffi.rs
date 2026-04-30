//! Re-export of raw libxc FFI bindings from the `libxc-sys` workspace crate.
//! Kept as a separate module so existing verify callers (oracle_lda_all,
//! oracle_func_flags, ...) import paths do not change.
pub use libxc_sys::*;
