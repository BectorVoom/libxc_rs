//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 443/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk443<F: Float>(t374: F, t376: F, t677: F, t370: F, t121: F, t1013: F, t361: F, t363: F, t3037: F, t3033: F, t360: F) -> (F, F, F, F, F, F, F, F) {
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0;
    let t3101 = t121 * t376;
    let t3127 = 1.0 / t1013 / t361;
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    let t3131 = t360 * t360;
    (t3082, t3084, t3101, t3127, t3128, t3129, t3130, t3131)
}
