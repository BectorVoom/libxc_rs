//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 608/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk608<F: Float>(t193: F, t531: F, t1799: F, t571: F, t1408: F, t3664: F, t1649: F, t3672: F, t172: F, t1787: F) -> (F, F, F, F, F) {
    let t5126 = t193 * t531;
    let t5127 = t571 * t1799;
    let t5134 = t3664 * t1408;
    let t5142 = t3672 * t1649;
    let t5154 = t1787 * t172;
    (t5126, t5127, t5134, t5142, t5154)
}
