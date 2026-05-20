//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1133/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1133<F: Float>(t39273: F, t39275: F, t39278: F, t39281: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t683: F, t702: F) -> F {
    let t39563 = F::new(1.0) * t683 * (-F::cast_from(0.21099166666666666667e1_f64) * t39273 + F::new(0.202552e2) * t39275 - F::cast_from(0.75019259259259259258e1_f64) * t39278 + F::cast_from(0.6564185185185185185e1_f64) * t39281 + F::cast_from(0.31003950617283950618e1_f64) * t39284 + F::cast_from(0.68258333333333333335e-1_f64) * t39289 - F::cast_from(0.10921333333333333333e1_f64) * t39291 + F::cast_from(0.12134814814814814815e1_f64) * t39293 + F::cast_from(0.10617962962962962963e1_f64) * t39295 + F::cast_from(0.13388493827160493828e1_f64) * t39298) * t702;
    t39563
}
