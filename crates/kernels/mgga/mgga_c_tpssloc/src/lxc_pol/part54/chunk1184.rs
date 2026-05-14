//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1184/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1184<F: Float>(t22633: F, t31100: F, t90566: F, t32704: F, t81228: F, t81326: F, t22704: F, t32693: F, t80650: F, t114285: F, t26338: F, t1992: F, t22635: F, t26226: F, t26331: F, t31099: F, t5308: F) -> (F, F, F, F, F, F, F) {
    let t120213 = 0.3289868133696452873e-1 * t22633 * t90566 * t31100;
    let t120217 = t81228 * t81326 * t32704;
    let t120218 = 0.16449340668482264365e-1 * t120217;
    let t120220 = t22704 * t81326 * t32693;
    let t120221 = 0.16449340668482264365e-1 * t120220;
    let t120226 = 0.3289868133696452873e-1 * t22633 * t80650 * t32704;
    let t120229 = 0.3289868133696452873e-1 * t22633 * t114285 * t26338;
    let t120232 = 0.9869604401089358619e-1 * t1992 * t22635 * t26226;
    let t120239 = 0.9869604401089358619e-1 * t26331 * t22635 * t31099 * t5308;
    (t120213, t120218, t120221, t120226, t120229, t120232, t120239)
}
