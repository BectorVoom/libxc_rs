//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 778/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk778<F: Float>(t2710: F, t814: F, t829: F, t252: F, t9971: F, t9976: F, t2728: F, t9981: F, t2684: F, t2732: F, t6647: F, t9632: F) -> (F, F, F, F, F) {
    let t10076 = t814 * t2710;
    let t10077 = t10076 * t829;
    let t10080 = t9971 * t252;
    let t10081 = t10080 * t9976;
    let t10084 = t2728 * t9981;
    let t10091 = t2732 * t2684;
    let t10094 = t6647 * t9632;
    (t10077, t10081, t10084, t10091, t10094)
}
