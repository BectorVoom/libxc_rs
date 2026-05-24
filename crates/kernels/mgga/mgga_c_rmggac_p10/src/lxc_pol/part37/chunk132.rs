//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 132/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk132<F: Float>(t338: F, t570: F, t118: F, t553: F, t560: F) -> (F, F, F) {
    let t571 = t338 * t570;
    let t572 = t118 * t571;
    let t574 = -F::cast_from(0.59871208509319042821e-1_f64) * t553 + F::cast_from(0.59871208509319042821e-1_f64) * t560 + F::cast_from(0.19957069503106347607e-1_f64) * t572;
    (t571, t572, t574)
}
