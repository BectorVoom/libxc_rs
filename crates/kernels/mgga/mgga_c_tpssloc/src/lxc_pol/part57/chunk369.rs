//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 369/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk369<F: Float>(t3262: F, t409: F, t407: F, t410: F, t3236: F, t281: F, t2820: F, t415: F, t1176: F, t241: F, t1097: F, t422: F, t1127: F, t432: F, t427: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t3315 = 1.0 / t3314;
    let t3319 = 0.22831111111111111111e-1 * t3236;
    let t3330 = t1127 * t432;
    let t3331 = 1.0 / t3330;
    let t3332 = t427 * t3331;
    let t3339 = 0.68863333333333333333e0 * t3236;
    let t3346 = 0.17365833333333333333e0 * t3293;
    let t3355 = t1127 * t1127;
    let t3356 = 1.0 / t3355;
    let t3357 = t427 * t3356;
    let t3358 = t435 * t435;
    (t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3297, t3313, t3315, t3319, t3332, t3339, t3346, t3357, t3358)
}
