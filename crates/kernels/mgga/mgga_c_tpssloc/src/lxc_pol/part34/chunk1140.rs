//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1140/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1140<F: Float>(t6371: F, t80827: F, t28073: F, t80888: F, t1361: F, t22690: F, t6330: F, t80840: F, t22792: F, t6347: F, t22804: F, t28077: F) -> (F, F, F, F, F) {
    let t97402 = t80827 * t6371;
    let t97404 = t80888 * t28073;
    let t97427 = t80840 * t22690 * t1361 * t6330;
    let t97431 = t22792 * t22690 * t1361 * t6347;
    let t97439 = t22804 * t28077;
    (t97402, t97404, t97427, t97431, t97439)
}
