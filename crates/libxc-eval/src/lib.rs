#![deny(warnings)]
//! `libxc-eval` — the compute-orchestration layer of libxc_rs.
//!
//! Owns the functional lifecycle/params types and `EvaluationWorkspace`, which
//! the facade and the C-ABI still take. The CubeCL kernel launch glue and the
//! 306 per-functional kernel dependencies it used to carry were deleted with
//! the archive; numerical dispatch is `libxc-reval`.
//!
//! `workspace/` is an undeclared top-level placeholder (0 consumers) moved here
//! as dead weight — NOT declared as a module (matching its prior state); the
//! LIVE workspace is `eval::workspace` (EvaluationWorkspace).
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

pub mod eval;
pub mod functional;
