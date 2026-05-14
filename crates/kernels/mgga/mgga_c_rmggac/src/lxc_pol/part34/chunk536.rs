//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 536/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk536<F: Float>(t15252: F, t515: F, t7231: F, t3351: F, t8975: F, t3352: F, t2144: F, t8946: F, t1971: F, t875: F, t8936: F, t3154: F, t8571: F, t3076: F, t570: F, t2044: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15253 = t515 * t15252;
    let t15254 = t7231 * t15253;
    let t15255 = t3351 * t15254;
    let t15257 = t515 * t8975;
    let t15258 = t3352 * t15257;
    let t15259 = t3351 * t15258;
    let t15261 = t2144 * t8946;
    let t15262 = t1971 * t15261;
    let t15263 = t3351 * t15262;
    let t15265 = t875 * t8936;
    let t15266 = t1971 * t15265;
    let t15267 = t3351 * t15266;
    let t15269 = t8571 * t3154;
    let t15271 = t3076 * t570;
    let t15272 = t2044 * t15271;
    (t15254, t15255, t15258, t15259, t15262, t15263, t15266, t15267, t15269, t15272)
}
