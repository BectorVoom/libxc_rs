//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 416/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk416<F: Float>(t1090: F, t248: F, t3521: F, t1227: F, t1009: F, t1190: F, t1011: F, t1212: F, t374: F, t486: F, t677: F, t485: F, t1203: F, t1222: F, t221: F, t3426: F) -> (F, F, F, F, F, F) {
    let t3523 = t248 * t3521 * t1090;
    let t3524 = t1227 * t3523;
    let t3534 = t1190 * t1009;
    let t3535 = t3534 * t1011;
    let t3536 = t3535 * t1212;
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / 13824.0;
    let t3543 = t1203 * t1222;
    let t3545 = t221 * t3426;
    (t3524, t3534, t3536, t3542, t3543, t3545)
}
