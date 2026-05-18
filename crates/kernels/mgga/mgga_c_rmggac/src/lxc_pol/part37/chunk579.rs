//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 579/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk579<F: Float>(t15035: F, t638: F, t639: F, t2405: F, t640: F, t2046: F, t2339: F, t3047: F, t2323: F, t2060: F, t2367: F, t739: F) -> (F, F, F, F, F, F, F) {
    let t15037 = t638 * t639 * t15035;
    let t15039 = t640 * t2405;
    let t15041 = t638 * t639 * t15039;
    let t15044 = t2046 * t3047 * t2339;
    let t15047 = t2046 * t3047 * t2323;
    let t15049 = t2060 * t2367;
    let t15050 = t739 * t15049;
    (t15037, t15039, t15041, t15044, t15047, t15049, t15050)
}
