//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1352/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1352<F: Float>(t1920: F, t21122: F, t21126: F, t21520: F, t21562: F, t21574: F, t23419: F, t28558: F, t2987: F, t4509: F, t6717: F, t7574: F, t88645: F, t99774: F, t99779: F, t99785: F, t99789: F) -> F {
    let t106328 = -F::new(0.30279567070605293142e-3) * t7574 * t28558 + t23419 * t21574 / F::new(768.0) + t6717 * t21562 / F::new(48.0) - t88645 / F::new(2304.0) - F::new(0.30279567070605293142e-3) * t99774 + t1920 * t4509 * t21122 / F::new(72.0) - t23419 * t21520 / F::new(384.0) + F::new(0.30279567070605293142e-3) * t99779 + t99785 / F::new(288.0) + t99789 / F::new(216.0) - t1920 * t2987 * t21126 / F::new(48.0);
    t106328
}
