//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 995/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk995<F: Float>(t33273: F, t81159: F, t115545: F, t22633: F, t26215: F, t33272: F, t80650: F, t33250: F, t6914: F, t115614: F, t1842: F, t1992: F, t22635: F) -> (F, F, F, F, F) {
    let t122102 = t81159 * t33273;
    let t122107 = t22633 * t115545 * t26215;
    let t122110 = t22633 * t80650 * t33272;
    let t122112 = t6914 * t33250;
    let t122117 = t1992 * t22635 * t115614 * t1842;
    (t122102, t122107, t122110, t122112, t122117)
}
