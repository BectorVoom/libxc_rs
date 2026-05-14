//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 539/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk539<F: Float>(t305: F, t571: F, t1986: F, t3141: F, t13848: F, t13850: F, t2314: F, t2411: F, t3148: F, t3151: F, t14011: F, t560: F, t3120: F, t572: F, t3112: F, t597: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15321 = t305 * t571;
    let t15322 = t1986 * t15321;
    let t15323 = t3141 * t15322;
    let t15326 = t2314 * t13848 * t13850;
    let t15331 = t2411 * t3148 * t3151;
    let t15333 = t14011 * t560;
    let t15334 = t3120 * t15333;
    let t15336 = t14011 * t572;
    let t15337 = t3120 * t15336;
    let t15339 = t3112 * t597;
    (t15322, t15323, t15326, t15331, t15333, t15334, t15336, t15337, t15339)
}
