//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 717/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk717<F: Float>(t23168: F, t7521: F, t4119: F, t6638: F, t6637: F, t6552: F, t22893: F, t7520: F, t23164: F, t1519: F, t234: F, t776: F, t1894: F, t4265: F, t214: F, t1880: F) -> (F, F, F, F, F) {
    let t25310 = t23168 * t7521;
    let t25312 = t6638 * t4119;
    let t25313 = t6637 * t25312;
    let t25314 = t6552 * t25313;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25320 = t25319 * t776;
    let t25321 = t6637 * t25320;
    let t25322 = t6552 * t25321;
    let t25324 = t1894 * t4265;
    let t25325 = t214 * t25324;
    let t25326 = t1880 * t25325;
    (t25310, t25314, t25317, t25322, t25326)
}
