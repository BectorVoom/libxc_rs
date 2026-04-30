//! Dispatch helpers for kernel-mgga-30 functionals.
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

/// Dispatch `mgga_x_tm` (libxc id 540).
pub(crate) fn dispatch_mgga_x_tm(
    ctx: &MggaLaunchCtx<'_>,
    order: DerivativeOrder,
    spin: Spin,
) -> Result<(), LibxcRsError> {
    mgga_zero_scalar_unpol_dispatch!(
        ctx, order, spin,
        [crate::kernel::mgga::batch30::mgga_x_tm::exc_unpol::mgga_x_tm_exc_unpol],
        [crate::kernel::mgga::batch30::mgga_x_tm::vxc_unpol::mgga_x_tm_vxc_unpol],
        "mgga_x_tm"
    );
    Ok(())
}

