//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 711/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk711<F: Float>(t1268: F, t1458: F, t2314: F, t4026: F, t4028: F, t4072: F, t5113: F, t671: F, t1390: F, t1845: F, t193: F, t531: F) -> (F, F, F) {
    let t5118 = 2.0 * t1268 * t4072 + 2.0 * t1458 * t2314 + 2.0 * t1458 * t5113 + 2.0 * t4028 * t671 + t4026;
    let t5122 = t1845 * t1390;
    let t5126 = t193 * t531;
    (t5118, t5122, t5126)
}
