//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 626/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk626<F: Float>(t3826: F, t8645: F, t7595: F, t7597: F, t7618: F, t7620: F, t8759: F, t8762: F, t8765: F, t8767: F, t8769: F, t8771: F) -> F {
    let t8773 = t3826 * t8645;
    let t8778 = -F::cast_from(0.63504270469206447408e-3_f64) * t8759 + F::cast_from(0.84672360625608596544e-3_f64) * t8762 + F::cast_from(0.68186654135613354324e-2_f64) * t8765 - F::cast_from(0.13637330827122670865e-1_f64) * t8767 + t7595 + F::cast_from(0.2993560425465952141e-1_f64) * t8769 - F::cast_from(0.13276154105060581339e-2_f64) * t8771 + F::cast_from(0.19914231157590872008e-2_f64) * t8773 + F::cast_from(0.2660942600414179681e-1_f64) * t7597 - F::cast_from(0.39914139006212695215e-1_f64) * t7618 + F::cast_from(0.88507694033737208925e-3_f64) * t7620;
    t8778
}
