//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 793/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk793<F: Float>(t1146: F, t3399: F, t3402: F, t448: F, t445: F, t440: F, t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t432: F, t11153: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11282 = 1.0 / t3399 / t1146;
    let t11285 = 1.0 / t3402 / t448;
    let t11292 = 1.0 / t3399 / t445;
    let t11310 = t440 * t11282;
    let t11314 = 0.16068111111111111111e1 * t11135;
    let t11317 = 0.46308888888888888888e0 * t11203;
    let t11349 = 1.0 / t3355 / t1127;
    let t11350 = t427 * t11349;
    let t11352 = 1.0 / t3358 / t435;
    let t11365 = t440 * t11292;
    let t11369 = 0.93932222222222222223e0 * t11135;
    let t11372 = 0.36793333333333333333e0 * t11203;
    let t11419 = 1.0 / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11444 = 0.53272592592592592592e-1 * t11135;
    let t11459 = 0.55403703703703703703e-1 * t11135;
    let t11487 = 20.0 / 27.0 * t11203;
    let t11516 = t461 * t11153;
    (t11282, t11285, t11292, t11310, t11314, t11317, t11350, t11352, t11365, t11369, t11372, t11420, t11444, t11459, t11487, t11516)
}
