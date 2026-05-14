//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 856/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk856<F: Float>(t22716: F, t8455: F, t1985: F, t214: F, t225: F, t22870: F, t567: F, t22704: F, t31091: F, t81326: F, t2006: F, t213: F, t22633: F, t22637: F, t31138: F, t6883: F) -> (F, F, F, F, F) {
    let t114264 = 0.12793931631041761173e0 * t22716 * t8455;
    let t114270 = 0.16449340668482264365e-1 * t1985 * t214 * t22870 * t225 * t567;
    let t114278 = t22704 * t81326 * t31091;
    let t114279 = 0.3289868133696452873e-1 * t114278;
    let t114285 = t213 * t2006 * t225;
    let t114288 = 0.6579736267392905746e-1 * t22633 * t114285 * t22637;
    let t114291 = t6883 * t31138;
    (t114264, t114270, t114279, t114288, t114291)
}
