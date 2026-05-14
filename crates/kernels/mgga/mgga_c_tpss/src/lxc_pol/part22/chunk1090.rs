//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1090/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1090<F: Float>(t1679: F, t619: F, t615: F, t77: F, t2049: F, t84: F, t1985: F, t578: F, t1993: F, t112: F, t234: F, t599: F, t630: F, t640: F, t2073: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18351 = t1679 * t619;
    let t18356 = t77 * t615 * t619;
    let t18360 = t77 * t84 * t2049;
    let t18363 = t578 * t1985;
    let t18366 = t578 * t1993;
    let t18392 = t234 * t112;
    let t18394 = t599 * t630;
    let t18395 = t18394 * t640;
    let t18396 = 2.0 / 3.0 * t18395;
    let t18397 = t68 * t2073;
    (t18351, t18356, t18360, t18363, t18366, t18392, t18394, t18395, t18396, t18397)
}
