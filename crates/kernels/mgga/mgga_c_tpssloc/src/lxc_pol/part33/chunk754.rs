//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 754/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk754<F: Float>(t655: F, t93: F, t94: F, t101: F, t102: F, t195: F, t40: F, t197: F, t52: F, t138: F, t2409: F, t125: F, t2412: F, t701: F, t2414: F) -> (F, F, F, F, F, F, F) {
    let t9364 = t655 * t655;
    let t9365 = 1.0 / t9364;
    let t9383 = t94 * t93;
    let t9384 = 1.0 / t9383;
    let t9397 = t102 * t101;
    let t9398 = 1.0 / t9397;
    let t9427 = 1.0 / t195 / t40;
    let t9438 = 1.0 / t197 / t52;
    let t9452 = 1.0 / t2409 / t138;
    let t9453 = t125 * t9452;
    let t9454 = t2412 * t701;
    let t9455 = t9454 * t2414;
    let t9457 = 0.96491876992155210402e2 * t9453 * t9455;
    (t9365, t9384, t9398, t9427, t9438, t9454, t9457)
}
