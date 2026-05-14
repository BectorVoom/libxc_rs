//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1244/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1244<F: Float>(t18495: F, t5736: F, t10179: F, t1771: F, t5570: F, t1219: F, t5731: F, t10164: F, t1765: F, t18444: F, t339: F, t789: F, t3263: F, t10085: F, t64: F, t2376: F, t5719: F) -> (F, F, F, F, F, F, F) {
    let t60649 = t5736 * t18495;
    let t60653 = t1771 * t5570 * t10179;
    let t60659 = t1219 * t5731;
    let t60684 = t1765 * t10164;
    let t60685 = 595.0 / 5184.0 * t60684;
    let t60695 = t339 * t18444 * t789;
    let t60696 = t60695 * t3263;
    let t60698 = t10085 * t64;
    let t60706 = t339 * t5719 * t2376;
    (t60649, t60653, t60659, t60685, t60696, t60698, t60706)
}
