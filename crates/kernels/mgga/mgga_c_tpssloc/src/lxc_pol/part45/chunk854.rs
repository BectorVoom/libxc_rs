//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 854/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk854<F: Float>(t31092: F, t6914: F, t22751: F, t31145: F, t22916: F, t31137: F, t6888: F, t22685: F, t22686: F, t22724: F, t31104: F, t1377: F, t6992: F, t1307: F, t22633: F, t22635: F) -> (F, F, F, F, F, F) {
    let t114208 = t6914 * t31092;
    let t114209 = 0.15352717957250113407e0 * t114208;
    let t114216 = t22751 * t31145;
    let t114217 = 0.15352717957250113407e0 * t114216;
    let t114220 = 0.3289868133696452873e-1 * t6888 * t31137 * t22916;
    let t114223 = 0.9869604401089358619e-1 * t22685 * t31137 * t22686;
    let t114225 = 0.52089578783527170489e-1 * t22724 * t31104;
    let t114226 = t1377 * t6992;
    let t114230 = 0.6579736267392905746e-1 * t22633 * t22635 * t114226 * t1307;
    (t114209, t114217, t114220, t114223, t114225, t114230)
}
