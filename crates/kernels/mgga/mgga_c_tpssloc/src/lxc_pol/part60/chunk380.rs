//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 380/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk380<F: Float>(t3314: F, t3236: F, t1127: F, t432: F, t427: F, t3293: F, t435: F, t1146: F, t445: F, t440: F, t448: F, t457: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t3359 = 1.0 / t3358;
    let t3363 = 0.12361111111111111111e-1 * t3236;
    let t3374 = t1146 * t445;
    let t3375 = 1.0 / t3374;
    let t3376 = t440 * t3375;
    let t3383 = 0.40256666666666666667e0 * t3236;
    let t3390 = 0.137975e0 * t3293;
    let t3399 = t1146 * t1146;
    let t3400 = 1.0 / t3399;
    let t3401 = t440 * t3400;
    let t3402 = t448 * t448;
    let t3403 = 1.0 / t3402;
    let t3426 = t697 * t457;
    (t3315, t3319, t3332, t3339, t3346, t3357, t3359, t3363, t3375, t3376, t3383, t3390, t3400, t3401, t3403, t3426)
}
