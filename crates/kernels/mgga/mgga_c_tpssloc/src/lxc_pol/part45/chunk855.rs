//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 855/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk855<F: Float>(t22633: F, t22635: F, t31099: F, t3719: F, t31100: F, t81228: F, t81326: F, t31109: F, t6883: F, t1992: F, t26225: F, t3888: F, t31124: F, t31101: F, t81159: F, t26331: F, t3734: F) -> (F, F, F, F, F, F, F) {
    let t114234 = 0.3289868133696452873e-1 * t22633 * t22635 * t31099 * t3719;
    let t114240 = t81228 * t81326 * t31100;
    let t114241 = 0.3289868133696452873e-1 * t114240;
    let t114242 = t6883 * t31109;
    let t114243 = 0.76763589786250567036e-1 * t114242;
    let t114247 = 0.9869604401089358619e-1 * t1992 * t22635 * t26225 * t3888;
    let t114253 = t6883 * t31124;
    let t114254 = 0.76763589786250567036e-1 * t114253;
    let t114255 = t81159 * t31101;
    let t114256 = 0.15352717957250113407e0 * t114255;
    let t114262 = 0.9869604401089358619e-1 * t26331 * t22635 * t31099 * t3734;
    (t114234, t114241, t114243, t114247, t114254, t114256, t114262)
}
