//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 783/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk783<F: Float>(t1114: F, t4231: F, t3931: F, t1569: F, t943: F, t1108: F, t938: F, t1120: F, t1571: F, t357: F, t339: F, t454: F, t1501: F, t3068: F, t3090: F, t242: F) -> (F, F, F, F, F, F, F, F) {
    let t4252 = t4231 * t1114;
    let t4253 = t3931 * t4252;
    let t4256 = t1569 * t943;
    let t4258 = t938 * t1108 * t4256;
    let t4261 = t1571 * t1120;
    let t4263 = t1569 * t357;
    let t4265 = t339 * t454 * t4263;
    let t4270 = t1501 * t1114;
    let t4271 = t3068 * t4270;
    let t4274 = t3090 * t1501;
    let t4275 = t242 * t4274;
    (t4252, t4253, t4258, t4261, t4265, t4270, t4271, t4275)
}
