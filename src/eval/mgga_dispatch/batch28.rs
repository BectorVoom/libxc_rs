//! Dispatch helpers for kernel-mgga-28 functionals.
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

/// Dispatch `mgga_x_pkzb` (libxc id 213).
pub(crate) fn dispatch_mgga_x_pkzb(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch28::mgga_x_pkzb::exc_unpol::mgga_x_pkzb_exc_unpol],
        [crate::kernel::mgga::batch28::mgga_x_pkzb::vxc_unpol::mgga_x_pkzb_vxc_unpol],
        "mgga_x_pkzb"
    );
    Ok(())
}

