//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1101/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1101<F: Float>(t10289: F, t38: F, t3482: F, t76: F, t1313: F, t619: F, t77: F, t3418: F, t582: F, t1317: F, t615: F, t3486: F, t84: F, t1290: F, t1976: F, t3426: F, t578: F) -> (F, F, F, F, F, F, F, F) {
    let t19352 = t10289 * t38;
    let t19380 = t76 * t3482;
    let t19388 = t77 * t1313 * t619;
    let t19396 = t3418 * t582;
    let t19403 = t615 * t1317;
    let t19404 = t77 * t19403;
    let t19407 = t84 * t3486;
    let t19408 = t77 * t19407;
    let t19411 = t1976 * t1290;
    let t19414 = t578 * t3426;
    (t19352, t19380, t19388, t19396, t19404, t19408, t19411, t19414)
}
