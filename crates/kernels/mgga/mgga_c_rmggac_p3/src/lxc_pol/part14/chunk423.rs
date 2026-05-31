//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 423/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk423<F: Float>(t1003: F, t230: F, t1001: F, t195: F, t1131: F, t388: F, t155: F, t1041: F, t971: F, t416: F, t171: F, t4157: F) -> (F, F, F, F, F, F) {
    let t4179 = F::cast_from(1.0_f64) / t1003 / t230;
    let t4182 = t195 * t1001;
    let t4186 = t388 * t1131;
    let t4187 = t155 * t4186;
    let t4189 = t971 * t1041;
    let t4190 = t4189 * t416;
    let t4202 = t171 * t4157;
    (t4179, t4182, t4187, t4189, t4190, t4202)
}
