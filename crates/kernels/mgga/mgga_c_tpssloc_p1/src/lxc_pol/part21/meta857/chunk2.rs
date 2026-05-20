//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3110/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3110<F: Float>(t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F) -> F {
    let t64422 = -F::cast_from(0.80513333333333333333e0_f64) * t63398 - F::new(0.12077e1) * t63400 + F::new(0.181155e1) * t63404 + F::new(0.72462e1) * t63408 + F::new(0.12077e1) * t63412 + F::cast_from(0.33547222222222222222e0_f64) * t63417 - F::cast_from(0.89459259259259259259e0_f64) * t63422 + F::cast_from(0.73586666666666666667e-1_f64) * t64074 + F::new(0.22076e0) * t64076 - F::new(0.5519e-1) * t64079 - F::new(0.16557e0) * t64082 - F::new(0.99342e0) * t64085 - F::new(0.44152e0) * t64087 - F::new(0.66228e0) * t64089 + F::new(0.33114e0) * t64092;
    t64422
}
