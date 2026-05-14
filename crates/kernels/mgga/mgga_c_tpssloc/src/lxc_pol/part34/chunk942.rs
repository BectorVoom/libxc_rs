//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 942/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk942<F: Float>(t22960: F, t28248: F, t1408: F, t1484: F, t25: F, t5544: F, t5657: F, t6571: F, t6553: F, t1880: F, t1527: F, t25191: F, t23270: F, t22986: F, t225: F, t258: F, t5631: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28249 = t22960 * t28248;
    let t28252 = t1408 * t1484;
    let t28256 = t25 * t5544;
    let t28263 = t6571 * t5657;
    let t28264 = t6553 * t28263;
    let t28265 = t1880 * t28264;
    let t28267 = t25191 * t1527;
    let t28268 = t23270 * t28267;
    let t28269 = t22986 * t28268;
    let t28272 = t5631 * t225 * t258;
    (t28249, t28252, t28256, t28263, t28264, t28265, t28267, t28268, t28269, t28272)
}
