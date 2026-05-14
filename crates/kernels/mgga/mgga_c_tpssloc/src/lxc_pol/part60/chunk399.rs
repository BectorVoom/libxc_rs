//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 399/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk399<F: Float>(t1509: F, t252: F, t1519: F, t814: F, t1530: F, t870: F, t193: F, t200: F) -> (F, F, F, F) {
    let t4282 = t252 * t1509;
    let t4295 = t814 * t1519;
    let t4310 = t1530 * t870;
    let t4314 = t193 * t200;
    (t4282, t4295, t4310, t4314)
}
