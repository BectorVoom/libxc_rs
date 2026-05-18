//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 507/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk507<F: Float>(t1752: F, t225: F, t1243: F, t5000: F, t1390: F, t1845: F, t193: F, t531: F, t1799: F, t571: F, t1408: F, t3664: F) -> (F, F, F, F, F, F) {
    let t5055 = t1752 * t225;
    let t5064 = t5000 * t1243;
    let t5122 = t1845 * t1390;
    let t5126 = t193 * t531;
    let t5127 = t571 * t1799;
    let t5134 = t3664 * t1408;
    (t5055, t5064, t5122, t5126, t5127, t5134)
}
