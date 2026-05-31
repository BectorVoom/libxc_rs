//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2898/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2898<F: Float>(t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t59657: F, t60161: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60176: F) -> F {
    let t60482 = -F::cast_from(0.1898925e1_f64) * t60161 + F::cast_from(0.10954222222222222222e0_f64) * t60163 - F::cast_from(0.82156666666666666667e-1_f64) * t60166 + F::cast_from(0.18257037037037037037e0_f64) * t60168 + F::cast_from(0.32862666666666666666e0_f64) * t60171 - F::cast_from(0.91285185185185185185e-1_f64) * t60173 - F::cast_from(0.88582716049382716049e-1_f64) * t59657 + F::cast_from(0.1898925e1_f64) * t60176 + F::cast_from(0.73028148148148148147e0_f64) * t48155 - F::cast_from(0.12171358024691358024e0_f64) * t48157 - F::cast_from(0.43816888888888888888e0_f64) * t48159 - F::cast_from(0.21908444444444444444e0_f64) * t48161 - F::cast_from(0.21908444444444444444e0_f64) * t48163 + F::cast_from(0.73028148148148148146e-1_f64) * t48165 + F::cast_from(0.36514074074074074073e-1_f64) * t48167;
    t60482
}
