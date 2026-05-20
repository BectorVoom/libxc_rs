//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2006/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2006<F: Float>(t591: F, t9701: F, t2397: F, t39277: F, t39280: F, t693: F, t119: F, t133: F, t240: F, t39273: F, t39275: F, t39278: F, t39281: F, t39284: F, t39289: F) -> (F, F, F, F, F) {
    let t39291 = t9701 * t591;
    let t39293 = t2397 * t39277;
    let t39295 = t693 * t39280;
    let t39298 = t133 * t119 * t240;
    let t39300 = -F::cast_from(0.28769444444444444444e1_f64) * t39273 + F::cast_from(0.27618666666666666667e2_f64) * t39275 - F::cast_from(0.10229135802469135803e2_f64) * t39278 + F::cast_from(0.89504938271604938273e1_f64) * t39281 + F::cast_from(0.31310740740740740741e1_f64) * t39284 + F::new(0.366775e-1) * t39289 - F::new(0.58684e0) * t39291 + F::cast_from(0.65204444444444444445e0_f64) * t39293 + F::cast_from(0.5705388888888888889e0_f64) * t39295 + F::cast_from(0.13490888888888888889e1_f64) * t39298;
    (t39291, t39293, t39295, t39298, t39300)
}
