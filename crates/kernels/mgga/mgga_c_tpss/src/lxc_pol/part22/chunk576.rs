//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 576/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk576<F: Float>(t2454: F, t2455: F, t2462: F, t2467: F, t2471: F, t285: F, t841: F, t845: F, t867: F, t281: F, t844: F, t269: F) -> (F, F, F, F, F, F) {
    let t2473 = t2454 + F::new(0.11872222222222222222e-1) * t2455 - F::new(0.11872222222222222222e-1) * t2462 + F::new(0.35616666666666666666e-1) * t2467 - F::new(0.17808333333333333333e-1) * t2471;
    let t2475 = F::new(0.621814e-1) * t2473 * t285;
    let t2476 = t841 * t845;
    let t2478 = F::new(2.0) * t2476 * t867;
    let t2479 = t844 * t281;
    let t2480 = F::new(1.0) / t2479;
    let t2481 = t269 * t2480;
    (t2473, t2475, t2476, t2478, t2480, t2481)
}
