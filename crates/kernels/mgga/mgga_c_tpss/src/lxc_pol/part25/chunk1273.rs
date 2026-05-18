//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1273/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1273<F: Float>(t2376: F, t339: F, t5719: F, t1235: F, t159: F, t7091: F, t1695: F, t510: F, t527: F, t5543: F, t17942: F, t517: F) -> (F, F, F, F, F, F) {
    let t60706 = t339 * t5719 * t2376;
    let t60707 = t60706 * t1235;
    let t60720 = t7091 * t159;
    let t60722 = t60720 * t510 * t1695;
    let t60724 = t5543 * t527;
    let t60730 = t17942 * t517;
    (t60706, t60707, t60720, t60722, t60724, t60730)
}
