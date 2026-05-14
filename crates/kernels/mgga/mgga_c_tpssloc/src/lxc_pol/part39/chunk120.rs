//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 120/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk120<F: Float>(t315: F, t324: F, t293: F, t300: F, t302: F, t311: F, t194: F, t241: F, zeta_threshold: F) -> (F, F, F, F) {
    let t294 = 2.0 <= zeta_threshold;
    let t297 = 0.0 <= zeta_threshold;
    let t325 = t315 * t324;
    let t328 = t300 * (-0.310907e-1 * t302 * t311 + t293 - 0.19751673498613801407e-1 * t325);
    let t330 = 0.19751673498613801407e-1 * t300 * t325;
    let t331 = piecewise3(t294, t194, t241);
    let t332 = piecewise3(t297, t194, 0.0);
    let t334 = t331 / 2.0 + t332 / 2.0;
    let t335 = t334 * t334;
    (t328, t330, t334, t335)
}
