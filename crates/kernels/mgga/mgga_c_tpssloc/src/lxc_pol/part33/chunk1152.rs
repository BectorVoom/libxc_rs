//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1152/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1152<F: Float>(t28195: F, t6883: F, t28199: F, t6897: F, t794: F, t3886: F, t6439: F, t1377: F, t6347: F, t28192: F, t80727: F, t22892: F, t7691: F, t90544: F, t28200: F, t225: F, t28053: F) -> (F, F, F, F, F, F, F, F) {
    let t97573 = t6883 * t28195;
    let t97599 = t6897 * t794 * t28199;
    let t97608 = t3886 * t6439;
    let t97637 = t1377 * t6347;
    let t97664 = t80727 * t28192;
    let t97732 = t22892 * t90544 * t7691;
    let t97750 = t6883 * t28200;
    let t97756 = t28053 * t225;
    (t97573, t97599, t97608, t97637, t97664, t97732, t97750, t97756)
}
