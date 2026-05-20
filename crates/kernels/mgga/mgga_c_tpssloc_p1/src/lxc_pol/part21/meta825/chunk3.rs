//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2903/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2903<F: Float>(t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F) -> F {
    let t60562 = -F::new(0.17938e1) * t59748 + F::cast_from(0.39862222222222222223e1_f64) * t59753 - F::cast_from(0.71752000000000000002e1_f64) * t59757 + F::cast_from(0.11958666666666666667e1_f64) * t59759 - F::cast_from(0.79724444444444444445e0_f64) * t59761 - F::new(0.17938e1) * t59765 + F::cast_from(0.11958666666666666667e1_f64) * t59769 - F::cast_from(0.21908444444444444444e0_f64) * t60308 + F::cast_from(0.73028148148148148149e-1_f64) * t60310 + F::cast_from(0.48685432098765432099e-1_f64) * t60312 + F::cast_from(0.16431333333333333333e0_f64) * t60315 + F::cast_from(0.43816888888888888889e0_f64) * t60318 - F::cast_from(0.54771111111111111112e-1_f64) * t60321 - F::cast_from(0.36514074074074074075e-1_f64) * t60324 - F::cast_from(0.85199506172839506175e-1_f64) * t60327;
    t60562
}
