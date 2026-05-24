//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 690/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk690<F: Float>(t504: F, t7190: F, t14189: F, t16156: F, t13966: F, t2046: F, t7305: F, t14199: F, t13962: F, t7311: F, t14185: F, t2040: F, t2048: F, t3056: F, t4789: F, t71: F) -> (F, F, F, F, F, F, F) {
    let t69054 = t504 * t7190;
    let t69057 = t16156 * t14189;
    let t69060 = t2046 * t13966 * t7305;
    let t69064 = t16156 * t14199;
    let t69067 = t2046 * t13962 * t7311;
    let t69069 = t16156 * t14185;
    let t69082 = t3056 * t2048 * t4789 * t71 * t2040;
    (t69054, t69057, t69060, t69064, t69067, t69069, t69082)
}
