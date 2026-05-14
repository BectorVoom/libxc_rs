//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 541/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk541<F: Float>(t15296: F, t3144: F, t3076: F, t551: F, t2044: F, t12200: F, t558: F, t7273: F, t262: F, t570: F, t3068: F, t10570: F, t559: F, t797: F, t1986: F, t3141: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15297 = t15296 * t3144;
    let t15301 = t3076 * t551;
    let t15302 = t2044 * t15301;
    let t15303 = t12200 * t15302;
    let t15305 = t3076 * t558;
    let t15306 = t2044 * t15305;
    let t15307 = t7273 * t15306;
    let t15309 = t262 * t570;
    let t15310 = t3068 * t15309;
    let t15311 = t10570 * t15310;
    let t15313 = t262 * t558;
    let t15314 = t3068 * t15313;
    let t15315 = t12200 * t15314;
    let t15317 = t797 * t559;
    let t15318 = t1986 * t15317;
    let t15319 = t3141 * t15318;
    (t15297, t15302, t15303, t15306, t15307, t15309, t15310, t15311, t15313, t15314, t15315, t15318, t15319)
}
