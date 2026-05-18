//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 945/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk945<F: Float>(t2047: F, t794: F, t6562: F, t6572: F, t82133: F, t8547: F, t7106: F, t857: F, t23030: F, t31405: F, t31315: F, t23012: F, t8548: F) -> (F, F, F, F, F, F, F) {
    let t114790 = t794 * t2047;
    let t114792 = t6562 * t114790 * t6572;
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114814 = t23030 * t31405;
    let t114827 = t6562 * t794 * t31315;
    let t114864 = t23012 * t8548;
    (t114790, t114792, t114795, t114797, t114814, t114827, t114864)
}
