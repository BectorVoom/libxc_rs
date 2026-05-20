//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1314/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1314<F: Float>(t225: F, t28108: F, t22674: F, t28232: F, t6897: F, t28195: F, t6883: F, t28199: F, t794: F, t3886: F, t6439: F, t1377: F, t6347: F) -> (F, F, F, F, F, F) {
    let t97558 = t28108 * t225;
    let t97571 = t6897 * t22674 * t28232;
    let t97573 = t6883 * t28195;
    let t97599 = t6897 * t794 * t28199;
    let t97608 = t3886 * t6439;
    let t97637 = t1377 * t6347;
    (t97558, t97571, t97573, t97599, t97608, t97637)
}
