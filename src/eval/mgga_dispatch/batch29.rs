//! Dispatch helpers for kernel-mgga-29 functionals.
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

/// Dispatch `hyb_mgga_x_m05` (libxc id 438).
/// Kernel takes 13 per-functional scalar(s): param_a_0,param_a_1,param_a_2,param_a_3,param_a_4,param_a_5,param_a_6,param_a_7,param_a_8,param_a_9,param_a_10,param_a_11,param_csi_HF
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_hyb_mgga_x_m05(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(438u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_xc_lp90` (libxc id 564).
pub(crate) fn dispatch_mgga_xc_lp90(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch29::mgga_xc_lp90::exc_unpol::mgga_xc_lp90_exc_unpol],
        [crate::kernel::mgga::batch29::mgga_xc_lp90::vxc_unpol::mgga_xc_lp90_vxc_unpol],
        "mgga_xc_lp90"
    );
    Ok(())
}

/// Dispatch `mgga_x_gx` (libxc id 575).
/// Kernel takes 3 per-functional scalar(s): param_alphainf,param_c0,param_c1
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_gx(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(575u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_x_pbe_gx` (libxc id 576).
pub(crate) fn dispatch_mgga_x_pbe_gx(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch29::mgga_x_pbe_gx::exc_unpol::mgga_x_pbe_gx_exc_unpol],
        [crate::kernel::mgga::batch29::mgga_x_pbe_gx::vxc_unpol::mgga_x_pbe_gx_vxc_unpol],
        "mgga_x_pbe_gx"
    );
    Ok(())
}

/// Dispatch `mgga_k_rda` (libxc id 621).
/// Kernel takes 10 per-functional scalar(s): param_A0,param_A1,param_A2,param_A3,param_a,param_b,param_beta1,param_beta2,param_beta3,param_c
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_k_rda(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(621u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

