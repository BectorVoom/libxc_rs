//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 114/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk114<F: Float>(t275: F, t276: F, t277: F, t267: F, t270: F, t273: F) -> (F, F, F, F) {
    let t279 = t275 * t276 * t277;
    let t281 = 0.379785e1 * t270 + 0.8969e0 * t267 + 0.204775e0 * t273 + 0.123235e0 * t279;
    let t284 = 1.0 + 0.16081979498692535067e2 / t281;
    let t285 = f64::ln(t284);
    (t279, t281, t284, t285)
}
