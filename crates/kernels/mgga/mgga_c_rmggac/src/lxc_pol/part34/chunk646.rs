//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 646/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk646<F: Float>(t338: F, t3814: F, t1296: F, t20: F, t1318: F, t21: F, t41: F, t1342: F, t117: F, t25809: F, t128: F, t348: F) -> (F, F, F, F, F, F, F) {
    let t25877 = t3814 * t338;
    let t25987 = t1296 * t20;
    let t26004 = t1318 * t1318;
    let t26007 = t21 / t41 / t26004;
    let t26077 = t1342 * t1342;
    let t26078 = F::new(1.0) / t26077;
    let t26087 = t117 * t25809;
    let t26115 = t348 * t128;
    (t25877, t25987, t26004, t26007, t26078, t26087, t26115)
}
