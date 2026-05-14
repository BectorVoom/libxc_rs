//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1147/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1147<F: Float>(t28143: F, t80727: F, t1338: F, t28107: F, t28160: F, t6883: F, t6396: F, t80820: F, t28101: F, t80958: F, t1827: F, t91285: F, t19815: F, t6944: F, t22765: F, t6422: F) -> (F, F, F, F, F, F, F, F) {
    let t97179 = t80727 * t28143;
    let t97193 = t1338 * t28107;
    let t97200 = t6883 * t28160;
    let t97219 = t80820 * t6396;
    let t97238 = t80958 * t28101;
    let t97240 = t91285 * t1827;
    let t97246 = t19815 * t6944;
    let t97253 = t22765 * t6422;
    (t97179, t97193, t97200, t97219, t97238, t97240, t97246, t97253)
}
