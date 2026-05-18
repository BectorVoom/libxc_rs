//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 524/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk524<F: Float>(t158: F, t2332: F, t581: F, t725: F, t681: F, t157: F, t37: F, t72: F, t727: F, t732: F, t2211: F, t2319: F, t720: F) -> (F, F, F, F, F, F, F) {
    let t2333 = t158 * t2332;
    let t2334 = t725 * t581;
    let t2335 = t681 * t2334;
    let t2337 = t37 * t157;
    let t2341 = t727 * t72;
    let t2342 = t2341 * t732;
    let t2345 = t2319 * t2211 * t720;
    (t2333, t2334, t2335, t2337, t2341, t2342, t2345)
}
