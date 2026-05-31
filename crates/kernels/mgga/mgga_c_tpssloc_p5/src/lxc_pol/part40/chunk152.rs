//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 152/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk152<F: Float>(t407: F, t410: F, t413: F, t417: F) -> (F, F, F) {
    let t432 = F::cast_from(0.705945e1_f64) * t410 + F::cast_from(0.1549425e1_f64) * t407 + F::cast_from(0.420775e0_f64) * t413 + F::cast_from(0.1562925e0_f64) * t417;
    let t435 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t432;
    let t436 = F::ln(t435);
    (t432, t435, t436)
}
