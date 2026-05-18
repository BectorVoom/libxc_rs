//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 868/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk868<F: Float>(t14117: F, t21708: F, t8807: F, t8811: F, t15333: F, t68528: F, t13823: F, t38848: F, t7756: F, t2415: F, t70504: F, t7349: F) -> (F, F, F, F, F) {
    let t75561 = t21708 * t14117 * t8807;
    let t75564 = t21708 * t14117 * t8811;
    let t75566 = t68528 * t15333;
    let t75572 = t13823 * t38848 * t7756;
    let t75575 = t7349 * t2415 * t70504;
    (t75561, t75564, t75566, t75572, t75575)
}
