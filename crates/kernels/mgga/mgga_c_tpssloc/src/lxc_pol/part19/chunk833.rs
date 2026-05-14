//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 833/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk833<F: Float>(t2827: F, t699: F, t2830: F, t2833: F, t241: F, t2978: F, t10216: F, t9288: F) -> (F, F, F, F, F) {
    let t10298 = t699 * t2827;
    let t10300 = t699 * t2830;
    let t10302 = t699 * t2833;
    let t10304 = t241 * t2978;
    let t10305 = t10216 * t9288;
    (t10298, t10300, t10302, t10304, t10305)
}
