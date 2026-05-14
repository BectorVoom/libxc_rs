//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 804/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk804<F: Float>(t10046: F, t218: F, t225: F, t2592: F, t2627: F, t852: F, t2633: F, t235: F, t860: F, t9958: F, t2679: F, t2732: F, t2710: F, t814: F, t829: F, t252: F, t9971: F) -> (F, F, F, F, F, F, F, F) {
    let t10047 = t218 * t10046;
    let t10049 = t2592 * t225;
    let t10054 = t2627 * t852;
    let t10055 = t10054 * t2633;
    let t10058 = t235 * t10046;
    let t10069 = t860 * t9958;
    let t10073 = t2732 * t2679;
    let t10076 = t814 * t2710;
    let t10077 = t10076 * t829;
    let t10080 = t9971 * t252;
    (t10047, t10049, t10055, t10058, t10069, t10073, t10077, t10080)
}
