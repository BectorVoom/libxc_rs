//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 540/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk540<F: Float>(t15339: F, t201: F, t14022: F, t14027: F, t13862: F, t15204: F, t14032: F, t2319: F, t14041: F, t13888: F, t2282: F, t3133: F, t553: F, t560: F, t3157: F, t5058: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15340 = t15339 * t201;
    let t15342 = t15340 * t14022 * t14027;
    let t15344 = t13862 * t15204;
    let t15345 = t14032 * t15344;
    let t15347 = t13862 * t2319;
    let t15348 = t14041 * t15347;
    let t15350 = t13888 * t2282;
    let t15351 = t3133 * t15350;
    let t15353 = t13862 * t553;
    let t15354 = t3133 * t15353;
    let t15356 = t13862 * t560;
    let t15357 = t3133 * t15356;
    let t15359 = t5058 * t3157;
    (t15340, t15342, t15344, t15345, t15347, t15348, t15350, t15351, t15353, t15354, t15356, t15357, t15359)
}
