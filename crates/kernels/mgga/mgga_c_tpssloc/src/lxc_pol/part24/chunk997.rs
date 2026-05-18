//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 997/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk997<F: Float>(t3402: F, t448: F, t11129: F, t11282: F, t1164: F, t3411: F, t3415: F, t3399: F, t445: F, t3403: F, t1143: F, t3375: F) -> (F, F, F, F, F, F) {
    let t11285 = F::new(1.0) / t3402 / t448;
    let t11286 = t11282 * t11129 * t11285;
    let t11288 = F::new(0.10254018858216406658e4) * t1164 * t11286;
    let t11290 = F::new(0.35089341735807877242e1) * t3411 * t3415;
    let t11292 = F::new(1.0) / t3399 / t445;
    let t11294 = t11292 * t11129 * t3403;
    let t11296 = F::new(0.10389515463408878255e3) * t1164 * t11294;
    let t11297 = t1143 * t3375;
    (t11285, t11288, t11290, t11292, t11296, t11297)
}
