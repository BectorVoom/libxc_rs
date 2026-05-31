//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 175/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk175<F: Float>(t440: F, t449: F, t300: F, t425: F, t427: F, t436: F, t338: F, t51: F) -> (F, F, F) {
    let t450 = t440 * t449;
    let t453 = t300 * (-F::cast_from(0.310907e-1_f64) * t427 * t436 + t425 - F::cast_from(0.19751673498613801407e-1_f64) * t450);
    let t455 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t450;
    let t456 = t51 * t338;
    (t453, t455, t456)
}
