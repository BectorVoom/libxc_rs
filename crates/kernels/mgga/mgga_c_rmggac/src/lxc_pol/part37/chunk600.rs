//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 600/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk600<F: Float>(t2411: F, t3148: F, t3151: F, t14011: F, t560: F, t3120: F, t572: F, t3112: F, t597: F, t201: F, t14022: F, t14027: F) -> (F, F, F, F, F, F, F, F) {
    let t15331 = t2411 * t3148 * t3151;
    let t15333 = t14011 * t560;
    let t15334 = t3120 * t15333;
    let t15336 = t14011 * t572;
    let t15337 = t3120 * t15336;
    let t15339 = t3112 * t597;
    let t15340 = t15339 * t201;
    let t15342 = t15340 * t14022 * t14027;
    (t15331, t15333, t15334, t15336, t15337, t15339, t15340, t15342)
}
