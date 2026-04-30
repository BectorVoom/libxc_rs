//! Dispatch helpers for kernel-mgga-33 functionals.
//!
//! Auto-generated for plan 04-04 from `mgga_roster.tsv`. Zero-scalar
//! kernels launch via `mgga_zero_scalar_unpol_dispatch!`; scalar-bearing
//! kernels return `UnsupportedFunctional` pending Phase 4 follow-up
//! libxc ext_params wiring (B3 invariant — no shared params struct).

#![allow(unused_imports, unused_variables, clippy::too_many_arguments)]

use super::{MggaLaunchCtx, mgga_zero_scalar_unpol_dispatch};
use crate::error::LibxcRsError;
use crate::model::{DerivativeOrder, Spin};
use cubecl::cpu::CpuRuntime;
use cubecl::frontend::ScalarArg;
use cubecl::prelude::ArrayArg;

/// Dispatch `mgga_x_tpss` (libxc id 202).
/// Kernel takes 7 per-functional scalar(s): param_BLOC_a,param_BLOC_b,param_b,param_c,param_e,param_kappa,param_mu
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_tpss(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(202u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_x_task` (libxc id 707).
/// Kernel takes 11 per-functional scalar(s): param_task_anu_0,param_task_anu_1,param_task_anu_2,param_task_bnu_0,param_task_bnu_1,param_task_bnu_2,param_task_bnu_3,param_task_bnu_4,param_task_c,param_task_d,param_task_h0x
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_task(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(707u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

