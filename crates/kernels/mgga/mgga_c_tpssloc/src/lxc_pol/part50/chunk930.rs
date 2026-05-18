//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 930/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk930<F: Float>(t25306: F, t6637: F, t6552: F, t23168: F, t7521: F, t4119: F, t6638: F, t22893: F, t7520: F, t23164: F, t1519: F, t234: F) -> (F, F, F, F, F) {
    let t25307 = t6637 * t25306;
    let t25308 = t6552 * t25307;
    let t25310 = t23168 * t7521;
    let t25312 = t6638 * t4119;
    let t25313 = t6637 * t25312;
    let t25314 = t6552 * t25313;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    (t25308, t25310, t25314, t25317, t25319)
}
