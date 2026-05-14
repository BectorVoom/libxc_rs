//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1212/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1212<F: Float>(t13015: F, t5728: F, t1646: F, t60749: F, t19506: F, t5570: F, t13032: F, t1705: F, t935: F, t1232: F, t4516: F, t520: F, t1656: F, t3259: F, t3260: F, t41437: F) -> (F, F, F, F, F, F, F, F) {
    let t65645 = t5728 * t13015;
    let t65647 = t60749 * t1646;
    let t65667 = t19506 * t5570;
    let t65685 = t1705 * t13032 * t935;
    let t65691 = t4516 * t1232 * t520;
    let t65695 = t1656 * t3259;
    let t65696 = t65695 * t3260;
    let t65703 = t41437 * t520;
    (t65645, t65647, t65667, t65685, t65691, t65695, t65696, t65703)
}
