//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1273/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1273<F: Float>(t6390: F, t80997: F, t22797: F, t6375: F, t22779: F, t28057: F, t6371: F, t80827: F, t28073: F, t80888: F, t1361: F, t22690: F, t6330: F, t80840: F) -> (F, F, F, F, F, F) {
    let t97380 = t80997 * t6390;
    let t97394 = t22797 * t6375;
    let t97400 = t22779 * t28057;
    let t97402 = t80827 * t6371;
    let t97404 = t80888 * t28073;
    let t97427 = t80840 * t22690 * t1361 * t6330;
    (t97380, t97394, t97400, t97402, t97404, t97427)
}
