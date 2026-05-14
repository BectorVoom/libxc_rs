//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 501/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk501<F: Float>(t1210: F, t3504: F, t3500: F, t121: F, t1229: F, t374: F, t486: F, t677: F, t485: F, t221: F, t3426: F, t456: F, t1176: F, t3247: F, t3242: F, t3439: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    let t3521 = t121 * t1229;
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / 13824.0;
    let t3545 = t221 * t3426;
    let t3547 = t456 * t3545 / 432.0;
    let t3555 = t1176 * t3247;
    let t3560 = t3439 * t3242;
    (t3514, t3515, t3521, t3540, t3542, t3545, t3547, t3555, t3560)
}
