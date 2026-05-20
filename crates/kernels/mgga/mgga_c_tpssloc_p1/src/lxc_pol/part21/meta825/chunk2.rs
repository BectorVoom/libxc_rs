//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2902/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2902<F: Float>(t47787: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t60282: F, t60296: F) -> F {
    let t60546 = F::cast_from(0.16431333333333333333e0_f64) * t60282 - F::cast_from(0.79724444444444444445e0_f64) * t59700 + F::cast_from(0.26574814814814814814e0_f64) * t59702 + F::cast_from(0.22145679012345679012e0_f64) * t59704 - F::cast_from(0.33218518518518518518e0_f64) * t59708 - F::cast_from(0.88582716049382716048e0_f64) * t59713 + F::cast_from(0.11958666666666666667e1_f64) * t59717 - F::cast_from(0.39862222222222222222e0_f64) * t59721 - F::cast_from(0.33218518518518518518e0_f64) * t59727 + F::cast_from(0.11958666666666666667e1_f64) * t59732 - F::cast_from(0.13287407407407407407e1_f64) * t59735 + F::cast_from(0.47834666666666666668e1_f64) * t59738 + F::cast_from(0.16431333333333333333e0_f64) * t60296 + F::cast_from(0.62007901234567901235e0_f64) * t47787 + F::cast_from(0.11958666666666666667e1_f64) * t59744;
    t60546
}
