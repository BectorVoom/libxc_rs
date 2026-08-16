#![deny(warnings)]
//! `libxc-eval` — the compute-orchestration layer of libxc_rs.
//!
//! Owns eval (dispatch), functional (lifecycle/params), the kernel launch glue,
//! and all 306 per-functional kernel dependencies behind the
//! `oracle-{lda,gga,mgga}` features. Depends one-way on `libxc-core` and is NOT
//! depended on by `libxc-compat`'s consumers (SC-3). Extracted by Phase 10.
//!
//! `workspace/` is an undeclared top-level placeholder (0 consumers) moved here
//! as dead weight — NOT declared as a module (matching its prior state); the
//! LIVE workspace is `eval::workspace` (EvaluationWorkspace).
// CubeCL #[cube] macro expansion generates code that triggers these lints.
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

pub mod eval;
pub mod functional;
#[cfg(feature = "cubecl-backend")]
pub mod kernel;
#[cfg(feature = "cubecl-backend")]
pub mod math; // re-export shim of libxc_kernel_math — preserves libxc_rs::math via the facade
