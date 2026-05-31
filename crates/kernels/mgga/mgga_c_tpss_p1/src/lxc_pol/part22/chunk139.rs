//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 139/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk139<F: Float>(t275: F, t276: F, t400: F, t392: F, t395: F, t398: F) -> (F, F, F, F) {
    let t402 = t275 * t276 * t400;
    let t404 = F::cast_from(0.379785e1_f64) * t395 + F::cast_from(0.8969e0_f64) * t392 + F::cast_from(0.204775e0_f64) * t398 + F::cast_from(0.123235e0_f64) * t402;
    let t407 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t404;
    let t408 = F::ln(t407);
    (t402, t404, t407, t408)
}
