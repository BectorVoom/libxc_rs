//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1241/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1241<F: Float>(t10164: F, t1765: F, t18444: F, t339: F, t789: F, t10085: F, t64: F, t2376: F, t5719: F, t1235: F, t159: F, t7091: F, t1695: F, t510: F, t527: F, t5543: F) -> (F, F, F, F, F, F, F, F) {
    let t60684 = t1765 * t10164;
    let t60685 = 595.0 / 5184.0 * t60684;
    let t60695 = t339 * t18444 * t789;
    let t60698 = t10085 * t64;
    let t60706 = t339 * t5719 * t2376;
    let t60707 = t60706 * t1235;
    let t60720 = t7091 * t159;
    let t60722 = t60720 * t510 * t1695;
    let t60723 = 455.0 / 1296.0 * t60722;
    let t60724 = t5543 * t527;
    (t60685, t60695, t60698, t60706, t60707, t60720, t60723, t60724)
}
