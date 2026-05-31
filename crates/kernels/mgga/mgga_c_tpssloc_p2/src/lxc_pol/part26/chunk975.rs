//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 975/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk975<F: Float>(t11383: F, t11398: F, t1156: F, t1119: F, t3307: F, t3264: F, t1117: F, t3315: F, t3313: F, t1128: F, t3324: F, t1124: F, t3356: F) -> (F, F, F, F, F, F) {
    let t11399 = t11383 + t11398;
    let t11400 = t11399 * t1156;
    let t11403 = t1119 * t3307;
    let t11405 = F::cast_from(6.0_f64) * t3264 * t11403;
    let t11407 = t3307 * t3315 * t1117;
    let t11409 = F::cast_from(0.48245938496077605201e2_f64) * t3313 * t11407;
    let t11410 = t3324 * t1128;
    let t11415 = t1124 * t3356;
    (t11399, t11400, t11405, t11409, t11410, t11415)
}
