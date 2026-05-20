//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2912/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2912<F: Float>(t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F) -> F {
    let t60698 = -F::new(0.309885e1) * t59748 + F::cast_from(0.68863333333333333334e1_f64) * t59753 - F::new(0.123954e2) * t59757 + F::new(0.20659e1) * t59759 - F::cast_from(0.13772666666666666667e1_f64) * t59761 - F::new(0.309885e1) * t59765 + F::new(0.20659e1) * t59769 - F::cast_from(0.27785333333333333334e0_f64) * t60308 + F::cast_from(0.92617777777777777779e-1_f64) * t60310 + F::cast_from(0.61745185185185185186e-1_f64) * t60312 + F::new(0.20839e0) * t60315 + F::cast_from(0.55570666666666666666e0_f64) * t60318 - F::cast_from(0.69463333333333333334e-1_f64) * t60321 - F::cast_from(0.46308888888888888889e-1_f64) * t60324 - F::cast_from(0.10805407407407407407e0_f64) * t60327;
    t60698
}
