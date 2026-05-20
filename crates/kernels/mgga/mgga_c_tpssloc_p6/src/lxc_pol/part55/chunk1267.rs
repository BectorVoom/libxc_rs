//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1267/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1267<F: Float>(t2165: F, t26135: F, t652: F, t7423: F, t24969: F, t7467: F, t27921: F, t6534: F, t24972: F, t26542: F, t26545: F, t105108: F, t7769: F) -> (F, F, F, F, F, F, F) {
    let t123244 = t652 * t2165 * t26135;
    let t123272 = t7423 * t26135;
    let t123274 = t24969 * t7467;
    let t123282 = t27921 * t6534;
    let t123285 = t24972 * t26542;
    let t123287 = t24972 * t26545;
    let t123290 = t105108 * t7769;
    (t123244, t123272, t123274, t123282, t123285, t123287, t123290)
}
