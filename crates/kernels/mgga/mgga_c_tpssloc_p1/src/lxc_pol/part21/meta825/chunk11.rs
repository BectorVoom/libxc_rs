//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2911/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2911<F: Float>(t47787: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t60282: F, t60296: F) -> F {
    let t60682 = F::new(0.20839e0) * t60282 - F::cast_from(0.13772666666666666666e1_f64) * t59700 + F::cast_from(0.45908888888888888889e0_f64) * t59702 + F::cast_from(0.38257407407407407407e0_f64) * t59704 - F::cast_from(0.57386111111111111112e0_f64) * t59708 - F::cast_from(0.15302962962962962963e1_f64) * t59713 + F::new(0.20659e1) * t59717 - F::cast_from(0.68863333333333333334e0_f64) * t59721 - F::cast_from(0.57386111111111111112e0_f64) * t59727 + F::new(0.20659e1) * t59732 - F::cast_from(0.22954444444444444444e1_f64) * t59735 + F::cast_from(0.82636000000000000001e1_f64) * t59738 + F::new(0.20839e0) * t60296 + F::cast_from(0.10712074074074074074e1_f64) * t47787 + F::new(0.20659e1) * t59744;
    t60682
}
