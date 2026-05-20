//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1896/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1896<F: Float>(t1437: F, t2303: F, t72: F, t4021: F, t641: F, t645: F, t7445: F, t12619: F, t71: F, t1433: F, t2307: F, t12719: F, t79: F) -> (F, F, F, F, F, F) {
    let t90227 = t72 * t2303 * t1437;
    let t90232 = t72 * t641 * t4021;
    let t90247 = t7445 * t645;
    let t90257 = t71 * t12619;
    let t90297 = t72 * t1433 * t2307;
    let t90334 = t72 * t79 * t12719;
    (t90227, t90232, t90247, t90257, t90297, t90334)
}
