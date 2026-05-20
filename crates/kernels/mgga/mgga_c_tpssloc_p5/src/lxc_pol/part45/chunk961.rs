//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 961/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk961<F: Float>(t22704: F, t31091: F, t81326: F, t2006: F, t213: F, t225: F, t22633: F, t22637: F, t31138: F, t6883: F, t31120: F, t31108: F, t6897: F, t794: F) -> (F, F, F, F, F) {
    let t114278 = t22704 * t81326 * t31091;
    let t114279 = F::cast_from(0.3289868133696452873e-1_f64) * t114278;
    let t114285 = t213 * t2006 * t225;
    let t114288 = F::cast_from(0.6579736267392905746e-1_f64) * t22633 * t114285 * t22637;
    let t114291 = t6883 * t31138;
    let t114292 = F::cast_from(0.76763589786250567036e-1_f64) * t114291;
    let t114296 = t6883 * t31120;
    let t114297 = F::cast_from(0.76763589786250567036e-1_f64) * t114296;
    let t114299 = t6897 * t794 * t31108;
    (t114279, t114288, t114292, t114297, t114299)
}
