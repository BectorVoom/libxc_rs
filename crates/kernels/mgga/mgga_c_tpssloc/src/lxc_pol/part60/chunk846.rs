//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 846/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk846<F: Float>(t22751: F, t32731: F, t1377: F, t7749: F, t32704: F, t81228: F, t81326: F, t22704: F, t32693: F, t32698: F, t6883: F, t32705: F, t81159: F, t6897: F, t8458: F, t90544: F) -> (F, F, F, F, F, F, F) {
    let t120179 = t22751 * t32731;
    let t120197 = t1377 * t7749;
    let t120217 = t81228 * t81326 * t32704;
    let t120220 = t22704 * t81326 * t32693;
    let t120269 = t6883 * t32698;
    let t120276 = t81159 * t32705;
    let t120296 = t6897 * t90544 * t8458;
    (t120179, t120197, t120217, t120220, t120269, t120276, t120296)
}
