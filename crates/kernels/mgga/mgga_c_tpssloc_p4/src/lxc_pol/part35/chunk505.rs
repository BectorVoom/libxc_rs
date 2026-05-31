//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 505/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk505<F: Float>(t1097: F, t419: F, t409: F, t407: F, t410: F, t3236: F, t281: F, t2820: F, t415: F, t1176: F, t241: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3262 = t1097 * t419;
    let t3263 = F::cast_from(1.0_f64) / t3262;
    let t3264 = t409 * t3263;
    let t3270 = F::cast_from(1.0_f64) / t410 / t407;
    let t3274 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3236;
    let t3282 = F::cast_from(0.39862222222222222223e0_f64) * t3236;
    let t3287 = F::cast_from(1.0_f64)/F::sqrt(t407);
    let t3293 = t281 * t2820 * t415;
    let t3294 = F::cast_from(0.13692777777777777778e0_f64) * t3293;
    let t3297 = t241 * t1176;
    let t3311 = t1097 * t1097;
    let t3312 = F::cast_from(1.0_f64) / t3311;
    let t3313 = t409 * t3312;
    (t3263, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3297, t3311, t3312, t3313)
}
