//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 532/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk532<F: Float>(t2701: F, t4255: F, t820: F, t4119: F, t847: F, t1516: F, t2621: F, t2623: F, t2640: F, t2643: F, t2695: F, t2698: F, t4191: F, t4236: F, t4240: F, t4250: F, t4253: F, t817: F, t843: F) -> (F, F, F) {
    let t4257 = t2701 * t820 * t4255;
    let t4261 = t847 * t820 * t4119;
    let t4264 = t2643 * t4191 / 768.0 - t817 * t4236 / 3072.0 - t2643 * t4240 / 3072.0 - 7.0 / 4608.0 * t2621 + 7.0 / 4608.0 * t2640 + t2695 + 7.0 / 1152.0 * t2698 - t2623 * t1516 / 768.0 + t2643 * t4250 / 768.0 + 7.0 / 1152.0 * t4253 + 5.0 / 768.0 * t843 * t4257 - t843 * t4261 / 768.0;
    (t4257, t4261, t4264)
}
