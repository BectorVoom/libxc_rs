//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 484/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk484<F: Float>(t374: F, t486: F, t677: F, t485: F, t1203: F, t1222: F, t221: F, t3426: F, t456: F, t1197: F, t135: F, t1174: F, t121: F, t1216: F, t248: F, t1213: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / 13824.0;
    let t3543 = t1203 * t1222;
    let t3545 = t221 * t3426;
    let t3547 = t456 * t3545 / 432.0;
    let t3548 = t135 * t1197;
    let t3549 = t1174 * t3548;
    let t3570 = t121 * t486;
    let t3572 = t248 * t3570 * t1216;
    let t3573 = t1213 * t3572;
    (t3540, t3542, t3543, t3545, t3547, t3548, t3549, t3570, t3572, t3573)
}
