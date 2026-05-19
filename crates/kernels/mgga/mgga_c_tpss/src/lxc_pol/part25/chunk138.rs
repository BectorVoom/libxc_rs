//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 138/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk138<F: Float>(t275: F, t276: F, t400: F, t392: F, t395: F, t398: F) -> (F, F, F, F) {
    let t402 = t275 * t276 * t400;
    let t404 = F::new(0.379785e1) * t395 + F::new(0.8969e0) * t392 + F::new(0.204775e0) * t398 + F::new(0.123235e0) * t402;
    let t407 = F::new(1.0) + F::cast_from(0.16081979498692535067e2_f64) / t404;
    let t408 = F::ln(t407);
    (t402, t404, t407, t408)
}
