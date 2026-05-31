//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2546/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2546<F: Float>(t43816: F, t51040: F, t51051: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t64074: F, t64076: F, t64087: F, t64089: F) -> F {
    let t71527 = -F::cast_from(0.31310740740740740741e0_f64) * t43816 + t51040 - F::cast_from(0.91983333333333333333e-1_f64) * t51051 + F::cast_from(0.80513333333333333334e0_f64) * t63361 + F::cast_from(0.40256666666666666666e0_f64) * t63382 + F::cast_from(0.12077e1_f64) * t63384 - F::cast_from(0.12077e1_f64) * t63398 - F::cast_from(0.181155e1_f64) * t63400 + F::cast_from(0.11038e0_f64) * t64074 + F::cast_from(0.33114e0_f64) * t64076 - F::cast_from(0.66228e0_f64) * t64087 - F::cast_from(0.99342e0_f64) * t64089;
    t71527
}
