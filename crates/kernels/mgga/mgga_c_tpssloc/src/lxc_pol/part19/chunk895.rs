//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 895/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk895<F: Float>(t1060: F, t11027: F, t1022: F, t3166: F, t10947: F, t3185: F, t3199: F, t3196: F, t4684: F, t11007: F, t383: F, t1014: F, t10471: F, t10470: F, t10481: F, t381: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11028 = t11027 * t1060;
    let t11030 = t3166 * t1022;
    let t11031 = t11030 * t1060;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11040 = t3196 * t4684;
    let t11043 = t383 * t11007;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    let t11047 = t381 * t10481;
    (t11028, t11031, t11034, t11037, t11040, t11043, t11045, t11046, t11047)
}
