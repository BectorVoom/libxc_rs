//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 379/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk379<F: Float>(t3236: F, t1229: F, t154: F, t636: F, t2296: F, t1097: F, t419: F, t409: F, t407: F, t410: F, t281: F, t2820: F, t415: F, t1176: F, t241: F, t422: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3237 = 0.23744444444444444444e-1 * t3236;
    let t3240 = t154 * t1229;
    let t3241 = t636 * t636;
    let t3242 = 1.0 / t3241;
    let t3247 = 1.0 / t2296;
    let t3262 = t1097 * t419;
    let t3263 = 1.0 / t3262;
    let t3264 = t409 * t3263;
    let t3270 = 1.0 / t410 / t407;
    let t3274 = 4.0 / 9.0 * t3236;
    let t3282 = 0.39862222222222222223e0 * t3236;
    let t3287 = 1.0/f64::sqrt(t407);
    let t3293 = t281 * t2820 * t415;
    let t3294 = 0.13692777777777777778e0 * t3293;
    let t3297 = t241 * t1176;
    let t3311 = t1097 * t1097;
    let t3312 = 1.0 / t3311;
    let t3313 = t409 * t3312;
    let t3314 = t422 * t422;
    (t3237, t3240, t3242, t3247, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3297, t3313, t3314)
}
