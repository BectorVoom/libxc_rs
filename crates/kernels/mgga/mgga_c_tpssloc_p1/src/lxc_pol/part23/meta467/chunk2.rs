//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1370/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1370<F: Float>(t59657: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t77042: F, t77073: F, t77076: F) -> F {
    let t77189 = F::new(0.22076e0) * t68502 + F::new(0.132456e1) * t68504 - F::new(0.44152e0) * t68506 + F::cast_from(0.247573125e0_f64) * t77042 + F::new(0.11038e1) * t60168 - F::new(0.5519e0) * t60173 - F::cast_from(0.53675555555555555556e0_f64) * t59657 + F::new(0.99342e0) * t76880 + F::new(0.16504875e0) * t77073 - F::cast_from(0.485484375e1_f64) * t77076 - F::cast_from(0.18396666666666666667e0_f64) * t60204 - F::new(0.82785e-1) * t76877 - F::cast_from(0.8585111111111111111e-1_f64) * t76887 - F::new(0.82785e-1) * t76890;
    t77189
}
