//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 825/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk825<F: Float>(t32704: F, t81228: F, t81326: F, t22704: F, t32693: F, t32698: F, t6883: F, t32705: F, t81159: F, t6897: F, t8458: F, t90544: F, t114172: F, t22892: F, t7691: F, t3886: F, t7749: F) -> (F, F, F, F, F, F, F) {
    let t120217 = t81228 * t81326 * t32704;
    let t120220 = t22704 * t81326 * t32693;
    let t120269 = t6883 * t32698;
    let t120276 = t81159 * t32705;
    let t120296 = t6897 * t90544 * t8458;
    let t120308 = t22892 * t114172 * t7691;
    let t120317 = t3886 * t7749;
    (t120217, t120220, t120269, t120276, t120296, t120308, t120317)
}
