//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 600/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk600<F: Float>(t262: F, t558: F, t3068: F, t12200: F, t559: F, t797: F, t1986: F, t3141: F, t305: F, t571: F, t13848: F, t13850: F, t2314: F) -> (F, F, F, F, F, F, F, F) {
    let t15313 = t262 * t558;
    let t15314 = t3068 * t15313;
    let t15315 = t12200 * t15314;
    let t15317 = t797 * t559;
    let t15318 = t1986 * t15317;
    let t15319 = t3141 * t15318;
    let t15321 = t305 * t571;
    let t15322 = t1986 * t15321;
    let t15323 = t3141 * t15322;
    let t15326 = t2314 * t13848 * t13850;
    (t15313, t15314, t15315, t15318, t15319, t15322, t15323, t15326)
}
