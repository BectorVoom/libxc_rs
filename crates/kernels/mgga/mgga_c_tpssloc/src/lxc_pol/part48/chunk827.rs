//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 827/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk827<F: Float>(t114121: F, t1351: F, t1992: F, t550: F, t6955: F, t6976: F, t31091: F, t80650: F, t22633: F, t31100: F, t1985: F, t22666: F, t31123: F, t22674: F, t6897: F, t22635: F, t31090: F, t3911: F) -> (F, F, F, F, F, F, F) {
    let t114122 = 0.16449340668482264365e-1 * t114121;
    let t114127 = 0.3289868133696452873e-1 * t1992 * t6976 * t6955 * t1351 * t550;
    let t114140 = 0.6579736267392905746e-1 * t1992 * t80650 * t31091;
    let t114145 = 0.6579736267392905746e-1 * t22633 * t80650 * t31100;
    let t114150 = 0.3289868133696452873e-1 * t1985 * t22666 * t31123;
    let t114154 = t6897 * t22674 * t31123;
    let t114155 = 0.16449340668482264365e-1 * t114154;
    let t114159 = 0.3289868133696452873e-1 * t1992 * t22635 * t31090 * t3911;
    (t114122, t114127, t114140, t114145, t114150, t114155, t114159)
}
