//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1013/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1013<F: Float>(t28116: F, t81228: F, t81326: F, t6897: F, t7700: F, t90544: F, t214: F, t6434: F, t22751: F, t28213: F, t28210: F, t28233: F, t6883: F, t22674: F, t28232: F, t28195: F) -> (F, F, F, F, F, F, F, F) {
    let t97503 = t81228 * t81326 * t28116;
    let t97509 = t6897 * t90544 * t7700;
    let t97511 = t214 * t6434;
    let t97529 = t22751 * t28213;
    let t97537 = t22751 * t28210;
    let t97548 = t6883 * t28233;
    let t97571 = t6897 * t22674 * t28232;
    let t97573 = t6883 * t28195;
    (t97503, t97509, t97511, t97529, t97537, t97548, t97571, t97573)
}
