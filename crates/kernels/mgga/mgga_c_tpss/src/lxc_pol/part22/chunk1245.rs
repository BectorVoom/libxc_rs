//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1245/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1245<F: Float>(t1791: F, t65157: F, t65165: F, t19342: F, t62348: F, t18350: F, t20264: F, t62024: F, t62259: F, t62262: F, t62264: F, t62266: F, t62270: F, t62273: F, t62275: F, t62345: F, t65182: F) -> (F,) {
    let t67349 = t1791 * t65157;
    let t67352 = t1791 * t65165;
    let t67358 = 160.0 / 3.0 * t62348 * t19342;
    let t67362 = 176.0 / 27.0 * t62259 + 176.0 / 27.0 * t62262 - 8.0 / 9.0 * t62264 - 16.0 / 9.0 * t62266 + 10.0 / 3.0 * t62024 * t20264 + 20.0 / 3.0 * t18350 * t67349 + 20.0 / 3.0 * t18350 * t67352 - 70.0 * t62345 * t65182 - t67358 + 40.0 / 9.0 * t62270 + 16.0 / 9.0 * t62273 + 32.0 / 9.0 * t62275;
    (t67362,)
}
