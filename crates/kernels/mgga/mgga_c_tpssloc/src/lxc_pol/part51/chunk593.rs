//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 593/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk593<F: Float>(t1268: F, t1458: F, t2314: F, t4026: F, t4028: F, t4072: F, t5113: F, t671: F, t1390: F, t1845: F, t193: F, t531: F, t1799: F, t571: F, t3919: F, t1408: F, t3664: F) -> (F, F, F, F, F, F) {
    let t5118 = 2.0 * t1268 * t4072 + 2.0 * t1458 * t2314 + 2.0 * t1458 * t5113 + 2.0 * t4028 * t671 + t4026;
    let t5122 = t1845 * t1390;
    let t5126 = t193 * t531;
    let t5127 = t571 * t1799;
    let t5131 = t3919 * t1799;
    let t5134 = t3664 * t1408;
    (t5118, t5122, t5126, t5127, t5131, t5134)
}
