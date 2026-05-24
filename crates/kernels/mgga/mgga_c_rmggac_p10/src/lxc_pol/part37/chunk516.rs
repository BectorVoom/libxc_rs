//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 516/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk516<F: Float>(t14150: F, t7351: F, t14148: F, t262: F, t352: F, t3068: F, t10570: F, t384: F, t464: F, t220: F, t1966: F, t209: F, t26: F, t476: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14151 = t7351 * t14150;
    let t14152 = t14148 * t14151;
    let t14154 = t262 * t352;
    let t14155 = t3068 * t14154;
    let t14156 = t10570 * t14155;
    let t14161 = t464 * t384;
    let t14162 = t14161 * t220;
    let t14163 = t1966 * t14162;
    let t14165 = t26 * t476 * t209;
    (t14151, t14152, t14154, t14155, t14156, t14161, t14162, t14163, t14165)
}
