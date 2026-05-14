//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 937/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk937<F: Float>(t11399: F, t1156: F, t1119: F, t3307: F, t3264: F, t1117: F, t3315: F, t3313: F, t1128: F, t3324: F, t1124: F, t3356: F, t3355: F, t432: F, t427: F, t11306: F, t3359: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11400 = t11399 * t1156;
    let t11403 = t1119 * t3307;
    let t11405 = 6.0 * t3264 * t11403;
    let t11407 = t3307 * t3315 * t1117;
    let t11409 = 0.48245938496077605201e2 * t3313 * t11407;
    let t11410 = t3324 * t1128;
    let t11415 = t1124 * t3356;
    let t11419 = 1.0 / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11421 = t11306 * t3359;
    (t11400, t11403, t11405, t11407, t11409, t11410, t11415, t11419, t11420, t11421)
}
