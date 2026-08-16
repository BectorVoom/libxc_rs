//! Benchmark support library: native transliterations + shared timing harness.

pub mod harness;
pub mod native;

#[cfg(feature = "mgga")]
pub mod mgga_glue;
#[cfg(feature = "mgga")]
pub mod mgga_native;
