#![deny(warnings)]
//! `libxc-core` — the pure data layer of libxc_rs.
//!
//! Holds the model, metadata, registry, input/output, dims, error types, and
//! the deferred id-registry. Contains zero compute logic and zero CubeCL
//! imports (SC-2) — orchestration lives in `libxc-eval`, the C ABI shim in
//! `libxc-compat`. Extracted by Phase 10 (workspace-level modular split).
//!
//! NOTE: `layout/` is an undeclared placeholder directory (no consumers); it
//! moved here as dead weight and is intentionally NOT declared as a module,
//! matching its prior undeclared state in the monolithic crate.

pub mod model;
pub mod meta;
pub mod error;
pub mod dims;
pub mod registry;
pub mod input;
pub mod output;
pub mod deferred;
