//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 873/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk873<F: Float>(t3355: F, t432: F, t427: F, t1094: F, t3263: F, t11135: F, t11203: F, t1176: F, t698: F, t1179: F, t1174: F, t135: F, t3439: F, t3247: F, t405: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11419 = 1.0 / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11424 = t1094 * t3263;
    let t11444 = 0.53272592592592592592e-1 * t11135;
    let t11459 = 0.55403703703703703703e-1 * t11135;
    let t11487 = 20.0 / 27.0 * t11203;
    let t11529 = t698 * t1176;
    let t11530 = t11529 * t1179;
    let t11531 = t1174 * t11530;
    let t11539 = t135 * t3439;
    let t11545 = 1.0 / t405 / t3247;
    let t11546 = t974 * t11545;
    (t11420, t11424, t11444, t11459, t11487, t11529, t11531, t11539, t11545, t11546)
}
