//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 473/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk473<F: Float>(t2627: F, t68: F, t226: F, t1509: F, t252: F, t814: F, t1519: F, t1530: F, t870: F, t193: F, t200: F) -> (F, F, F, F, F, F) {
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4282 = t252 * t1509;
    let t4290 = t68 * t814;
    let t4291 = t226 * t4290;
    let t4295 = t814 * t1519;
    let t4310 = t1530 * t870;
    let t4314 = t193 * t200;
    (t4281, t4282, t4291, t4295, t4310, t4314)
}
