//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1186/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1186<F: Float>(t22633: F, t22635: F, t31090: F, t97721: F, t1377: F, t7749: F, t1307: F, t1992: F, t32693: F, t80650: F, t31100: F, t90566: F, t32704: F, t81228: F, t81326: F, t22704: F) -> (F, F, F, F, F, F) {
    let t120196 = 0.6579736267392905746e-1 * t22633 * t22635 * t31090 * t97721;
    let t120197 = t1377 * t7749;
    let t120201 = 0.3289868133696452873e-1 * t22633 * t22635 * t120197 * t1307;
    let t120209 = 0.3289868133696452873e-1 * t1992 * t80650 * t32693;
    let t120213 = 0.3289868133696452873e-1 * t22633 * t90566 * t31100;
    let t120217 = t81228 * t81326 * t32704;
    let t120218 = 0.16449340668482264365e-1 * t120217;
    let t120220 = t22704 * t81326 * t32693;
    (t120196, t120201, t120209, t120213, t120218, t120220)
}
