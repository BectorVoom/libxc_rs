#![deny(warnings)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]
//! C-ABI compatibility layer for libxc_rs (Phase 6; extracted to crates/libxc-compat by Phase 10).
//!
//! Produces the `libxc_rs` cdylib/staticlib (extern "C" ABI). Depends on
//! libxc-core + libxc-eval; nothing in the workspace depends on it (SC-4).

pub mod c_layout;
pub mod errno;
pub mod hybrid;
pub mod ids;
pub mod info;
pub mod legacy_eval;
pub mod library;
pub mod macros;
pub mod raw_handle;
pub mod removed;
