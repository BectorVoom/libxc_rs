//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3092/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3092<F: Float>(t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F) -> F {
    let t64094 = -F::cast_from(0.79724444444444444445e0_f64) * t63398 - F::cast_from(0.11958666666666666667e1_f64) * t63400 + F::cast_from(0.17938e1_f64) * t63404 + F::cast_from(0.71752e1_f64) * t63408 + F::cast_from(0.11958666666666666667e1_f64) * t63412 + F::cast_from(0.33218518518518518518e0_f64) * t63417 - F::cast_from(0.88582716049382716048e0_f64) * t63422 + F::cast_from(0.73028148148148148149e-1_f64) * t64074 + F::cast_from(0.21908444444444444444e0_f64) * t64076 - F::cast_from(0.54771111111111111112e-1_f64) * t64079 - F::cast_from(0.16431333333333333333e0_f64) * t64082 - F::cast_from(0.98587999999999999998e0_f64) * t64085 - F::cast_from(0.43816888888888888888e0_f64) * t64087 - F::cast_from(0.65725333333333333332e0_f64) * t64089 + F::cast_from(0.32862666666666666666e0_f64) * t64092;
    t64094
}
