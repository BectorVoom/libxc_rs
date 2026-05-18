//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1328/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1328<F: Float>(t23041: F, t2681: F, t6621: F, t9618: F, t23053: F, t6614: F, t9960: F, t22690: F, t23122: F, t2553: F, t841: F, t22813: F, t6589: F, t80782: F) -> (F, F, F, F, F, F) {
    let t81889 = t23041 * t2681;
    let t81891 = t6621 * t9618;
    let t81893 = t23053 * t2681;
    let t81895 = t6614 * t9960;
    let t81899 = t23122 * t22690 * t841 * t2553;
    let t81902 = t22813 * t6589 * t80782;
    (t81889, t81891, t81893, t81895, t81899, t81902)
}
