//! Rayon evaluation layer.
//!
//! Calls the plain-Rust kernels in `crates/kernels-rayon` directly on the
//! caller's slices, parallelised over grid points. No device buffers, no
//! upload/launch/read-back cycle.

pub mod funcs;
pub mod gga;
pub mod lda;
pub mod mgga;
pub mod routing;
pub mod sweep_gga;
pub mod sweep_lda;
pub mod sweep_mgga;
