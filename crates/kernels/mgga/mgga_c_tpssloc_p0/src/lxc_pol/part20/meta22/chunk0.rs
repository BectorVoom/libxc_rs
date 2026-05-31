//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 170/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk170<F: Float>(t407: F, t410: F, t413: F, t417: F) -> (F, F, F) {
    let t419 = F::cast_from(0.379785e1_f64) * t410 + F::cast_from(0.8969e0_f64) * t407 + F::cast_from(0.204775e0_f64) * t413 + F::cast_from(0.123235e0_f64) * t417;
    let t422 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t419;
    let t423 = F::ln(t422);
    (t419, t422, t423)
}
