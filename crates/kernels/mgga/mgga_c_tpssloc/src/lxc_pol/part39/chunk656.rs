//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 656/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk656<F: Float>(t1137: F, t3351: F, t1127: F, t427: F, t435: F, t3333: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t449: F, t1143: F, t1147: F, t1146: F, t445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3352 = t3351 * t1137;
    let t3355 = t1127 * t1127;
    let t3356 = 1.0 / t3355;
    let t3357 = t427 * t3356;
    let t3358 = t435 * t435;
    let t3359 = 1.0 / t3358;
    let t3360 = t3333 * t3359;
    let t3363 = 0.12361111111111111111e-1 * t3236;
    let t3368 = t3363 - 0.61805555555555555556e-2 * t3238 - 0.61805555555555555555e-2 * t3245 + 0.18541666666666666667e-1 * t3250 + 0.92708333333333333333e-2 * t3254;
    let t3369 = t3368 * t449;
    let t3371 = t1143 * t1147;
    let t3374 = t1146 * t445;
    (t3352, t3355, t3356, t3357, t3358, t3359, t3360, t3363, t3368, t3369, t3371, t3374)
}
