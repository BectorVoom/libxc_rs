//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 140/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk140<F: Float>(t392: F, t395: F, t398: F, t402: F) -> (F, F, F) {
    let t417 = 0.705945e1 * t395 + 0.1549425e1 * t392 + 0.420775e0 * t398 + 0.1562925e0 * t402;
    let t420 = 1.0 + 0.32163958997385070134e2 / t417;
    let t421 = f64::ln(t420);
    (t417, t420, t421)
}
