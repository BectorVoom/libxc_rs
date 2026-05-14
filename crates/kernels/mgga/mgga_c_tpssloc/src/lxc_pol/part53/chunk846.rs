//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 846/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk846<F: Float>(t31608: F, t6883: F, t1377: F, t7213: F, t22716: F, t8622: F, t6897: F, t80645: F, t8621: F, t22704: F, t31559: F, t81326: F, t2085: F, t212: F, t22642: F, t6890: F) -> (F, F, F, F, F, F) {
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    let t115305 = t22716 * t8622;
    let t115308 = t6897 * t80645 * t8621;
    let t115318 = t22704 * t81326 * t31559;
    let t115330 = t22642 * t212 * t2085 * t6890;
    (t115294, t115296, t115305, t115308, t115318, t115330)
}
