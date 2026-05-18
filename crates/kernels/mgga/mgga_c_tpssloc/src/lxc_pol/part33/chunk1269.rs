//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1269/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1269<F: Float>(t28182: F, t6914: F, t22893: F, t28142: F, t80681: F, t28143: F, t80727: F, t1338: F, t28107: F, t28160: F, t6883: F, t6396: F, t80820: F) -> (F, F, F, F, F, F) {
    let t97148 = t6914 * t28182;
    let t97161 = t80681 * t22893 * t28142;
    let t97179 = t80727 * t28143;
    let t97193 = t1338 * t28107;
    let t97200 = t6883 * t28160;
    let t97219 = t80820 * t6396;
    (t97148, t97161, t97179, t97193, t97200, t97219)
}
