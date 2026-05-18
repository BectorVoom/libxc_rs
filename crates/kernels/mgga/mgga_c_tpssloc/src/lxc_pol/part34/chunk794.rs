//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 794/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk794<F: Float>(t5689: F, t892: F, t3216: F, t5946: F, t5717: F, t699: F, t5720: F, t5723: F, t5769: F, t942: F, t5737: F, t923: F) -> (F, F, F, F, F, F, F) {
    let t17195 = t5689 * t892;
    let t17202 = t5946 * t3216;
    let t17286 = t699 * t5717;
    let t17288 = t699 * t5720;
    let t17290 = t699 * t5723;
    let t17355 = t5769 * t942;
    let t17428 = t5737 * t923;
    (t17195, t17202, t17286, t17288, t17290, t17355, t17428)
}
