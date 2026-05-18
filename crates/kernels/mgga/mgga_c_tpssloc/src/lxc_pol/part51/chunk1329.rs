//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1329/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1329<F: Float>(t1377: F, t7749: F, t1307: F, t22633: F, t22635: F, t1992: F, t32693: F, t80650: F, t31100: F, t90566: F, t32704: F, t81228: F, t81326: F) -> (F, F, F, F) {
    let t120197 = t1377 * t7749;
    let t120201 = F::new(0.3289868133696452873e-1) * t22633 * t22635 * t120197 * t1307;
    let t120209 = F::new(0.3289868133696452873e-1) * t1992 * t80650 * t32693;
    let t120213 = F::new(0.3289868133696452873e-1) * t22633 * t90566 * t31100;
    let t120217 = t81228 * t81326 * t32704;
    (t120201, t120209, t120213, t120217)
}
