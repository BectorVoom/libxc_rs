//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 536/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk536<F: Float>(t841: F, t845: F, t281: F, t844: F, t269: F, t267: F, t270: F, t2453: F, t235: F, t68: F, t275: F, t277: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2476 = t841 * t845;
    let t2479 = t844 * t281;
    let t2480 = F::new(1.0) / t2479;
    let t2481 = t269 * t2480;
    let t2487 = F::new(1.0) / t270 / t267;
    let t2491 = F::new(4.0) / F::new(9.0) * t2453;
    let t2499 = F::new(0.39862222222222222223e0) * t2453;
    let t2504 = F::new(1.0)/f64::sqrt(t267);
    let t2509 = t68 * t235;
    let t2511 = t275 * t2509 * t277;
    (t2476, t2480, t2481, t2487, t2491, t2499, t2504, t2509, t2511)
}
