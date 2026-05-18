//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 796/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk796<F: Float>(t25338: F, t6552: F, t4119: F, t6554: F, t6553: F, t23204: F, t7479: F, t23164: F, t1530: F, t776: F) -> (F, F, F, F, F) {
    let t25339 = t6552 * t25338;
    let t25341 = t6554 * t4119;
    let t25342 = t6553 * t25341;
    let t25343 = t6552 * t25342;
    let t25345 = t23204 * t7479;
    let t25346 = t23164 * t25345;
    let t25365 = t1530 * t776;
    (t25339, t25341, t25343, t25346, t25365)
}
