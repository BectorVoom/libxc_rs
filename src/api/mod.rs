//! Layer-3 ergonomic API.
//!
//! Wraps the Phase-5 `Functional` runtime handle in three primitives:
//! - [`FunctionalBuilder`] — owned-self chained-config builder.
//! - [`BatchEvaluator`] — workspace-only batch driver with auto-dispatch.
//! - [`EvaluateInput`] — sealed dispatch trait for LDA/GGA/MGGA inputs.
//!
//! Layer-3 has zero `unsafe` (BUILD-04 / COMPAT-03 invariant).

pub mod batch;
pub mod builder;
pub mod evaluate;

pub use batch::BatchEvaluator;
pub use builder::FunctionalBuilder;
pub use evaluate::EvaluateInput;
