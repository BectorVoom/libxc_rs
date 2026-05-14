//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 585/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk585<F: Float>(t422: F, t3236: F, t1124: F, t1128: F, t1127: F, t432: F, t427: F, t3293: F, t435: F, t1143: F, t1147: F, t1146: F, t445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3314 = t422 * t422;
    let t3315 = 1.0 / t3314;
    let t3319 = 0.22831111111111111111e-1 * t3236;
    let t3327 = t1124 * t1128;
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
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    (t3314, t3315, t3319, t3327, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358, t3359, t3363, t3371, t3374)
}
