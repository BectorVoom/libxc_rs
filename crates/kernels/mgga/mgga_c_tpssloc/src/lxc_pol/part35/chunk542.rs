//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 542/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk542<F: Float>(t1390: F, t1845: F, t193: F, t531: F, t1799: F, t571: F, t1408: F, t3664: F, t1649: F, t3672: F, t172: F, t1787: F, t763: F, t67: F, t758: F, t533: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5122 = t1845 * t1390;
    let t5126 = t193 * t531;
    let t5127 = t571 * t1799;
    let t5134 = t3664 * t1408;
    let t5142 = t3672 * t1649;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5160 = t193 * t533;
    (t5122, t5126, t5127, t5134, t5142, t5154, t5155, t5157, t5158, t5160)
}
