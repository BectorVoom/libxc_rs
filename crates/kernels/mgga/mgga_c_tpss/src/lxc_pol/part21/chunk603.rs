//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 603/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk603<F: Float>(t304: F, t2551: F, t2453: F, t2455: F, t2462: F, t2467: F, t2471: F, t318: F, t891: F, t895: F, t314: F, t894: F) -> (F, F, F, F, F, F, F, F) {
    let t2576 = t304 * t304;
    let t2577 = 1.0 / t2576;
    let t2578 = t2551 * t2577;
    let t2581 = 0.12361111111111111111e-1 * t2453;
    let t2586 = t2581 + 0.61805555555555555556e-2 * t2455 - 0.61805555555555555555e-2 * t2462 + 0.18541666666666666667e-1 * t2467 - 0.92708333333333333333e-2 * t2471;
    let t2587 = t2586 * t318;
    let t2589 = t891 * t895;
    let t2592 = t894 * t314;
    let t2593 = 1.0 / t2592;
    (t2576, t2577, t2578, t2581, t2586, t2587, t2589, t2593)
}
