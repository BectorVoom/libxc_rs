//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1206/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1206<F: Float>(t1993: F, t3418: F, t1313: F, t1982: F, t77: F, t1333: F, t61870: F, t19590: F, t61873: F, t18394: F, t3532: F, t2074: F, t61877: F, t640: F, t18397: F, t2100: F) -> (F, F, F, F, F, F, F, F) {
    let t65403 = t3418 * t1993;
    let t65410 = t77 * t1313 * t1982;
    let t65440 = t61870 * t1333;
    let t65442 = t61873 * t19590;
    let t65444 = t18394 * t3532;
    let t65446 = t1333 * t2074;
    let t65447 = t61877 * t65446;
    let t65449 = t3532 * t640;
    let t65450 = t18397 * t65449;
    let t65452 = t1333 * t2100;
    (t65403, t65410, t65440, t65442, t65444, t65447, t65450, t65452)
}
