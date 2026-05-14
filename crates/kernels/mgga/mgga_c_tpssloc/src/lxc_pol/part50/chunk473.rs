//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 473/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk473<F: Float>(t371: F, t335: F, t368: F, t1015: F, t3033: F, t1030: F, t372: F, t364: F, t354: F, t1043: F, t121: F, t248: F, t884: F, t1041: F, t283: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3034 = t371 * t371;
    let t3036 = 1.0 / t3034 / t335;
    let t3037 = t368 * t3036;
    let t3038 = t1015 * t3037;
    let t3039 = t3033 * t3038;
    let t3046 = t1030 * t372;
    let t3047 = t364 * t3046;
    let t3048 = t354 * t3047;
    let t3051 = t121 * t1043;
    let t3053 = t248 * t3051 * t884;
    let t3054 = t1041 * t3053;
    let t3061 = 1.0 / t283 / t883;
    (t3034, t3036, t3037, t3039, t3046, t3048, t3051, t3053, t3054, t3061)
}
