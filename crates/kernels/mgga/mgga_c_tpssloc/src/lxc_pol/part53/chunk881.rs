//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 881/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk881<F: Float>(t22633: F, t33272: F, t80650: F, t33250: F, t6914: F, t115614: F, t1842: F, t1992: F, t22635: F, t115352: F, t6897: F, t7700: F, t1377: F, t7936: F, t1307: F, t31558: F, t5353: F) -> (F, F, F, F, F, F) {
    let t122110 = t22633 * t80650 * t33272;
    let t122112 = t6914 * t33250;
    let t122117 = t1992 * t22635 * t115614 * t1842;
    let t122121 = t6897 * t115352 * t7700;
    let t122124 = t1377 * t7936;
    let t122127 = t22633 * t22635 * t122124 * t1307;
    let t122131 = t1992 * t22635 * t31558 * t5353;
    (t122110, t122112, t122117, t122121, t122127, t122131)
}
