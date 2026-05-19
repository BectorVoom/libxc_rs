//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 959/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk959<F: Float>(t114226: F, t1307: F, t22633: F, t22635: F, t31099: F, t3719: F, t31100: F, t81228: F, t81326: F, t31109: F, t6883: F, t1992: F, t26225: F, t3888: F) -> (F, F, F, F, F) {
    let t114230 = F::cast_from(0.6579736267392905746e-1_f64) * t22633 * t22635 * t114226 * t1307;
    let t114234 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t22635 * t31099 * t3719;
    let t114240 = t81228 * t81326 * t31100;
    let t114241 = F::cast_from(0.3289868133696452873e-1_f64) * t114240;
    let t114242 = t6883 * t31109;
    let t114243 = F::cast_from(0.76763589786250567036e-1_f64) * t114242;
    let t114247 = F::cast_from(0.9869604401089358619e-1_f64) * t1992 * t22635 * t26225 * t3888;
    (t114230, t114234, t114241, t114243, t114247)
}
