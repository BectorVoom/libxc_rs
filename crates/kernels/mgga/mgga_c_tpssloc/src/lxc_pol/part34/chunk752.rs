//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 752/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk752<F: Float>(t11292: F, t440: F, t11135: F, t11203: F, t3355: F, t432: F, t427: F, t11153: F, t461: F, t1176: F, t698: F, t135: F, t3439: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11365 = t440 * t11292;
    let t11369 = F::cast_from(0.93932222222222222223e0_f64) * t11135;
    let t11372 = F::cast_from(0.36793333333333333333e0_f64) * t11203;
    let t11419 = F::new(1.0) / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11444 = F::cast_from(0.53272592592592592592e-1_f64) * t11135;
    let t11459 = F::cast_from(0.55403703703703703703e-1_f64) * t11135;
    let t11487 = F::new(20.0) / F::new(27.0) * t11203;
    let t11516 = t461 * t11153;
    let t11529 = t698 * t1176;
    let t11539 = t135 * t3439;
    (t11365, t11369, t11372, t11420, t11444, t11459, t11487, t11516, t11529, t11539)
}
