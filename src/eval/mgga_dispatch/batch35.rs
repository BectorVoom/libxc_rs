//! Dispatch helpers for kernel-mgga-35 functionals.
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

/// Dispatch `mgga_x_tb09` (libxc id 208) — VXC-only (no exc kernels).
/// Kernel takes 2 per-functional scalar(s): param_alpha,param_c
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_tb09(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(208u16).expect("registry-valid id"),
        reason: "MGGA vxc-only functional (mgga_x_tb09) requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_xc_cc06` (libxc id 229).
pub(crate) fn dispatch_mgga_xc_cc06(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch35::mgga_xc_cc06::exc_unpol::mgga_xc_cc06_exc_unpol],
        [crate::kernel::mgga::batch35::mgga_xc_cc06::vxc_unpol::mgga_xc_cc06_vxc_unpol],
        "mgga_xc_cc06"
    );
    Ok(())
}

/// Dispatch `mgga_x_jk` (libxc id 256).
/// Kernel takes 2 per-functional scalar(s): param_beta,param_gamma
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_jk(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(256u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_x_mvs` (libxc id 257).
/// Kernel takes 4 per-functional scalar(s): param_b,param_c1,param_e1,param_k0
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_mvs(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(257u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

/// Dispatch `mgga_c_cc` (libxc id 387).
pub(crate) fn dispatch_mgga_c_cc(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch35::mgga_c_cc::exc_unpol::mgga_c_cc_exc_unpol],
        [crate::kernel::mgga::batch35::mgga_c_cc::vxc_unpol::mgga_c_cc_vxc_unpol],
        "mgga_c_cc"
    );
    Ok(())
}

