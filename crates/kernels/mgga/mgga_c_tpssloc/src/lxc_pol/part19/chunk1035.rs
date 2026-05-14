//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1035/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1035<F: Float>(t119: F, t133: F, t240: F, t39273: F, t39275: F, t39278: F, t39281: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t739: F, t746: F, t1294: F, t3691: F, t9722: F) -> (F, F, F, F, F) {
    let t39298 = t133 * t119 * t240;
    let t39300 = -0.28769444444444444444e1 * t39273 + 0.27618666666666666667e2 * t39275 - 0.10229135802469135803e2 * t39278 + 0.89504938271604938273e1 * t39281 + 0.31310740740740740741e1 * t39284 + 0.366775e-1 * t39289 - 0.58684e0 * t39291 + 0.65204444444444444445e0 * t39293 + 0.5705388888888888889e0 * t39295 + 0.13490888888888888889e1 * t39298;
    let t39302 = t739 * t39300 * t746;
    let t39304 = 0.5848223622634646207e0 * t1294 * t39302;
    let t39305 = t3691 * t9722;
    (t39298, t39300, t39302, t39304, t39305)
}
