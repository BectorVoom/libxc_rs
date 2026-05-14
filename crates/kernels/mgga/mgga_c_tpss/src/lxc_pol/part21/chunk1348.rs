//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1348/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1348<F: Float>(t65569: F, t65602: F, t65632: F, t65652: F, t19506: F, t5570: F, t13032: F, t1705: F, t935: F, t1232: F, t4516: F, t520: F, t1656: F, t3259: F, t3260: F, t41437: F) -> (F, F, F, F, F, F, F) {
    let t65654 = t65569 + t65602 + t65632 + t65652;
    let t65667 = t19506 * t5570;
    let t65685 = t1705 * t13032 * t935;
    let t65691 = t4516 * t1232 * t520;
    let t65695 = t1656 * t3259;
    let t65696 = t65695 * t3260;
    let t65703 = t41437 * t520;
    (t65654, t65667, t65685, t65691, t65695, t65696, t65703)
}
