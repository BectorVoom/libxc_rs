//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1186/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1186<F: Float>(t31464: F, t5784: F, t18669: F, t7690: F, t18347: F, t1791: F, t61938: F, t61942: F, t60684: F, t60722: F, t18948: F, t219: F, t1219: F, t5918: F, t198: F, t206: F, t5848: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t62345 = t31464 * t5784;
    let t62348 = t7690 * t18669;
    let t62349 = t62348 * t18347;
    let t62351 = t1791 * t61938;
    let t62356 = t1791 * t61942;
    let t62375 = 595.0 / 2592.0 * t60684;
    let t62390 = 455.0 / 648.0 * t60722;
    let t62453 = t18948 * t219;
    let t62508 = t1219 * t5918;
    let t62610 = t198 * t206 * t5848;
    (t62345, t62348, t62349, t62351, t62356, t62375, t62390, t62453, t62508, t62610)
}
