//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1149/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1149<F: Float>(t22852: F, t3792: F, t80798: F, t97312: F, t22705: F, t236: F, t550: F, t6414: F, t22765: F, t6417: F, t6390: F, t80997: F, t22797: F, t6375: F, t22779: F, t28057: F) -> (F, F, F, F, F, F) {
    let t97367 = t22852 * t80798 * t97312 * t3792;
    let t97372 = t22852 * t22705 * t236 * t6414 * t550;
    let t97378 = t22765 * t6417;
    let t97380 = t80997 * t6390;
    let t97394 = t22797 * t6375;
    let t97400 = t22779 * t28057;
    (t97367, t97372, t97378, t97380, t97394, t97400)
}
