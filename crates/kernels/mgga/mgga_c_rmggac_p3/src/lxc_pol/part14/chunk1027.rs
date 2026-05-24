//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1027/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1027<F: Float>(t118: F, t25877: F, t326: F, t338: F, t352: F, t40940: F, t41129: F, t41386: F, t41393: F, t41395: F, t41402: F, t41405: F, t41409: F, t41412: F, t41414: F, t5155: F, t5266: F, t839: F, t848: F, t8946: F, t8975: F) -> F {
    let t41420 = -t41129 + F::cast_from(0.19957069503106347607e-1_f64) * t118 * t338 * t41386 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t40940 * t352 + F::cast_from(0.44903406381989282115e-1_f64) * t41393 + F::cast_from(0.35922725105591425692e0_f64) * t41395 + F::cast_from(0.71845450211182851384e0_f64) * t25877 * t8975 * t839 + F::cast_from(0.13637330827122670864e0_f64) * t41402 + F::cast_from(0.16364796992547205037e0_f64) * t41405 + F::cast_from(0.40911992481368012592e-1_f64) * t41409 - F::cast_from(0.81823984962736025184e-1_f64) * t41412 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t41414 + F::cast_from(0.23948483403727617128e0_f64) * t5155 * t8946 * t848;
    t41420
}
