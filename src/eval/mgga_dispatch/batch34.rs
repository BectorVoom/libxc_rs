//! Dispatch helpers for kernel-mgga-34 functionals.
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

/// Dispatch `mgga_c_cs` (libxc id 72).
pub(crate) fn dispatch_mgga_c_cs(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch34::mgga_c_cs::exc_unpol::mgga_c_cs_exc_unpol],
        [crate::kernel::mgga::batch34::mgga_c_cs::vxc_unpol::mgga_c_cs_vxc_unpol],
        "mgga_c_cs"
    );
    Ok(())
}

/// Dispatch `mgga_x_lta` (libxc id 201).
/// Kernel takes 1 per-functional scalar(s): param_ltafrac
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_lta(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(201u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_x_tau_hcth` (libxc id 205).
/// Kernel takes 8 per-functional scalar(s): param_cx_local_0,param_cx_local_1,param_cx_local_2,param_cx_local_3,param_cx_nlocal_0,param_cx_nlocal_1,param_cx_nlocal_2,param_cx_nlocal_3
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_tau_hcth(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(205u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_x_th` (libxc id 225).
pub(crate) fn dispatch_mgga_x_th(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch34::mgga_x_th::exc_unpol::mgga_x_th_exc_unpol],
        [crate::kernel::mgga::batch34::mgga_x_th::vxc_unpol::mgga_x_th_vxc_unpol],
        "mgga_x_th"
    );
    Ok(())
}

/// Dispatch `mgga_x_2d_js17` (libxc id 609).
pub(crate) fn dispatch_mgga_x_2d_js17(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch34::mgga_x_2d_js17::exc_unpol::mgga_x_2d_js17_exc_unpol],
        [crate::kernel::mgga::batch34::mgga_x_2d_js17::vxc_unpol::mgga_x_2d_js17_vxc_unpol],
        "mgga_x_2d_js17"
    );
    Ok(())
}

/// Dispatch `mgga_k_gea4` (libxc id 628).
pub(crate) fn dispatch_mgga_k_gea4(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch34::mgga_k_gea4::exc_unpol::mgga_k_gea4_exc_unpol],
        [crate::kernel::mgga::batch34::mgga_k_gea4::vxc_unpol::mgga_k_gea4_vxc_unpol],
        "mgga_k_gea4"
    );
    Ok(())
}

/// Dispatch `mgga_x_rlda` (libxc id 688).
/// Kernel takes 1 per-functional scalar(s): param_prefactor
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_rlda(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(688u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

