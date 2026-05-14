//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 936/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk936<F: Float>(t3411: F, t3415: F, t3399: F, t445: F, t11129: F, t3403: F, t1164: F, t1143: F, t3375: F, t1156: F, t1124: F, t3331: F, t1136: F, t3333: F, t1137: F, t11282: F, t440: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11290 = 0.35089341735807877242e1 * t3411 * t3415;
    let t11292 = 1.0 / t3399 / t445;
    let t11294 = t11292 * t11129 * t3403;
    let t11296 = 0.10389515463408878255e3 * t1164 * t11294;
    let t11297 = t1143 * t3375;
    let t11300 = t11129 * t1156;
    let t11303 = t1124 * t3331;
    let t11306 = t3333 * t1136;
    let t11307 = t11306 * t1137;
    let t11310 = t440 * t11282;
    (t11290, t11292, t11296, t11297, t11300, t11303, t11306, t11307, t11310)
}
