//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1041/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1041<F: Float>(t22751: F, t31145: F, t22724: F, t31104: F, t1377: F, t6992: F, t31100: F, t81228: F, t81326: F, t31109: F, t6883: F, t31124: F, t31101: F, t81159: F, t22716: F, t8455: F) -> (F, F, F, F, F, F, F, F) {
    let t114216 = t22751 * t31145;
    let t114225 = 0.52089578783527170489e-1 * t22724 * t31104;
    let t114226 = t1377 * t6992;
    let t114240 = t81228 * t81326 * t31100;
    let t114242 = t6883 * t31109;
    let t114253 = t6883 * t31124;
    let t114255 = t81159 * t31101;
    let t114264 = 0.12793931631041761173e0 * t22716 * t8455;
    (t114216, t114225, t114226, t114240, t114242, t114253, t114255, t114264)
}
