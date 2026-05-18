//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 491/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk491<F: Float>(t13966: F, t2040: F, t2046: F, t3167: F, t7508: F, t209: F, t476: F, t664: F, t515: F, t1971: F, t1970: F, t2164: F, t668: F) -> (F, F, F, F, F) {
    let t13968 = t2046 * t13966 * t2040;
    let t13970 = t7508 * t3167;
    let t13973 = t664 * t476 * t209;
    let t13974 = t515 * t13973;
    let t13975 = t1971 * t13974;
    let t13976 = t1970 * t13975;
    let t13980 = t2164 * t668;
    (t13968, t13970, t13975, t13976, t13980)
}
