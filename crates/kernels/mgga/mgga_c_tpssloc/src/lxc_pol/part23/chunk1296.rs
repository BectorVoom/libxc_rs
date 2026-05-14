//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1296/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1296<F: Float>(t449: F, t78211: F, t78223: F, t300: F, t14850: F, t21724: F, t1118: F, t11190: F, t78129: F, t6020: F, t3264: F, t3313: F, t3315: F, t78118: F, t78120: F, t78122: F, t78125: F, t78128: F, t78132: F, t78196: F, t78199: F) -> (F, F, F, F, F, F, F) {
    let t78225 = (t78211 + t78223) * t449;
    let t78227 = 0.19751673498613801407e-1 * t300 * t78225;
    let t78229 = 24.0 * t14850 * t21724;
    let t78232 = 24.0 * t11190 * t78129 * t1118;
    let t78233 = t6020 * t6020;
    let t78236 = 6.0 * t3264 * t78233 * t1118;
    let t78239 = 0.48245938496077605201e2 * t3313 * t78233 * t3315;
    let t78240 = -t78118 + t78120 - t78122 - t78125 - t78128 - t78132 + t78196 + t78199 + t78227 + t78229 - t78232 - t78236 + t78239;
    (t78225, t78227, t78229, t78232, t78236, t78239, t78240)
}
