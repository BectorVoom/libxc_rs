//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 465/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk465<F: Float>(t3787: F, t562: F, t193: F, t532: F, t531: F, t571: F, t111: F, t576: F) -> (F, F, F, F) {
    let t3897 = t3787 * t562;
    let t3918 = t193 * t532;
    let t3924 = t531 * t571;
    let t3941 = t576 * t111;
    (t3897, t3918, t3924, t3941)
}
