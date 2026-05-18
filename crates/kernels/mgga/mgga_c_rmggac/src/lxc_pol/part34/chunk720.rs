//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 720/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk720<F: Float>(t13957: F, t36292: F, t739: F, t14012: F, t14371: F, t1341: F, t638: F, t669: F, t7310: F, t1302: F, t14148: F, t14149: F, t20: F, t7351: F) -> (F, F, F, F) {
    let t70225 = t739 * t36292 * t13957;
    let t70229 = t14371 * t14012;
    let t70237 = t638 * t7310 * t669 * t1341;
    let t70271 = t14148 * t7351 * t14149 * t1302 * t20;
    (t70225, t70229, t70237, t70271)
}
