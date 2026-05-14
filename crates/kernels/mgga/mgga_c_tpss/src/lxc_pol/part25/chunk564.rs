//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 564/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk564<F: Float>(t1068: F, t294: F, t215: F, t442: F, t671: F, t441: F, t1102: F, t140: F, t1098: F, t1014: F, t390: F, t2840: F, t1100: F, t2845: F, t1106: F, t451: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3009 = t294 * t1068;
    let t3025 = t215 * t671 * t442;
    let t3027 = t441 * t3025 / 432.0;
    let t3028 = t140 * t1102;
    let t3029 = t1098 * t3028;
    let t3032 = 1.0 / t390 / t1014;
    let t3033 = t3032 * t2840;
    let t3038 = t1100 * t2845;
    let t3048 = 1.0 / t1106 / t451;
    (t3009, t3025, t3027, t3028, t3029, t3032, t3033, t3038, t3048)
}
