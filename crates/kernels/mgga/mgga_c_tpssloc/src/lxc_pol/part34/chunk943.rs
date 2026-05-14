//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 943/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk943<F: Float>(t214: F, t28272: F, t1880: F, t5544: F, t6554: F, t6553: F, t6552: F, t25224: F, t7479: F, t23195: F, t5636: F, t5527: F, t23035: F, t16815: F, t232: F, t6646: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28273 = t214 * t28272;
    let t28274 = t1880 * t28273;
    let t28276 = t6554 * t5544;
    let t28277 = t6553 * t28276;
    let t28278 = t6552 * t28277;
    let t28288 = t25224 * t7479;
    let t28289 = t6552 * t28288;
    let t28294 = t23195 * t5636;
    let t28295 = t6553 * t28294;
    let t28296 = t1880 * t28295;
    let t28298 = t6554 * t5527;
    let t28299 = t6553 * t28298;
    let t28300 = t23035 * t28299;
    let t28321 = t16815 * t232;
    let t28322 = t6646 * t28321;
    (t28273, t28274, t28276, t28277, t28278, t28288, t28289, t28294, t28295, t28296, t28298, t28299, t28300, t28321, t28322)
}
