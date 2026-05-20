//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1436/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1436<F: Float>(t43819: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F) -> F {
    let t44320 = F::cast_from(0.17757530864197530864e0_f64) * t43819;
    let t44327 = -F::cast_from(0.50735802469135802467e-1_f64) * t43811 + F::cast_from(0.4566222222222222222e-1_f64) * t43727 - F::cast_from(0.13698666666666666667e0_f64) * t43729 + F::cast_from(0.11415555555555555555e0_f64) * t43734 - F::cast_from(0.71030123456790123454e-1_f64) * t43816 + t44320 - F::cast_from(0.41095999999999999998e0_f64) * t43737 - F::cast_from(0.34246666666666666665e-1_f64) * t43823 - F::cast_from(0.4566222222222222222e-1_f64) * t43740 + F::new(0.61644e0) * t43743 + F::new(0.10274e0) * t43828 + F::cast_from(0.13698666666666666667e0_f64) * t43746;
    t44327
}
