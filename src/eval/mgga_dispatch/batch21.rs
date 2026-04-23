//! Dispatch helpers for kernel-mgga-21 functionals.
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

/// Dispatch `hyb_mgga_x_dldf` (libxc id 36).
pub(crate) fn dispatch_hyb_mgga_x_dldf(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch21::hyb_mgga_x_dldf::exc_unpol::hyb_mgga_x_dldf_exc_unpol],
        [crate::kernel::mgga::batch21::hyb_mgga_x_dldf::vxc_unpol::hyb_mgga_x_dldf_vxc_unpol],
        "hyb_mgga_x_dldf"
    );
    Ok(())
}

/// Dispatch `mgga_x_rtpss` (libxc id 299).
/// Kernel takes 5 per-functional scalar(s): param_b,param_c,param_e,param_kappa,param_mu
/// libxc ext_params defaults are not yet wired (tracked for follow-up plan).
pub(crate) fn dispatch_mgga_x_rtpss(
    _ctx: &MggaLaunchCtx<'_>,
    _order: DerivativeOrder,
    _spin: Spin,
) -> Result<(), LibxcRsError> {
    Err(LibxcRsError::UnsupportedFunctional {
        id: crate::model::FunctionalId::from_raw(299u16).expect("registry-valid id"),
        reason: "MGGA functional requires per-functional scalar defaults; \
                 see Phase 4 follow-up plan for libxc ext_params wiring",
    })
}

