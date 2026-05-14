//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 427/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk427<F: Float>(t1102: F, t3270: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t1100: F, t407: F, t1107: F, t281: F, t2820: F, t415: F, t1114: F, t699: F, t1176: F, t241: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3271 = t1102 * t1102;
    let t3272 = t3270 * t3271;
    let t3274 = 4.0 / 9.0 * t3236;
    let t3279 = t3274 - 2.0 / 9.0 * t3238 - 2.0 / 9.0 * t3245 + 2.0 / 3.0 * t3250 + t3254 / 3.0;
    let t3280 = t1100 * t3279;
    let t3282 = 0.39862222222222222223e0 * t3236;
    let t3287 = 1.0/f64::sqrt(t407);
    let t3288 = t3287 * t3271;
    let t3290 = t1107 * t3279;
    let t3293 = t281 * t2820 * t415;
    let t3294 = 0.13692777777777777778e0 * t3293;
    let t3295 = t699 * t1114;
    let t3297 = t241 * t1176;
    (t3272, t3280, t3282, t3288, t3290, t3293, t3294, t3295, t3297)
}
