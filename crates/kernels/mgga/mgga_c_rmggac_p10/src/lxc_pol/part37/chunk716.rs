//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 716/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk716<F: Float>(t70100: F, t13872: F, t14363: F, t13876: F, t13880: F, t13884: F, t14031: F, t14367: F, t14035: F, t1326: F, t14147: F, t3057: F) -> (F, F, F, F, F, F, F, F) {
    let t70101 = F::cast_from(0.15372131649401827112e-4_f64) * t70100;
    let t70104 = t14363 * t13872;
    let t70106 = t14363 * t13876;
    let t70108 = t14363 * t13880;
    let t70110 = t14363 * t13884;
    let t70123 = t14031 * t14367;
    let t70124 = t70123 * t14035;
    let t70127 = t14147 * t1326 * t3057;
    (t70101, t70104, t70106, t70108, t70110, t70123, t70124, t70127)
}
