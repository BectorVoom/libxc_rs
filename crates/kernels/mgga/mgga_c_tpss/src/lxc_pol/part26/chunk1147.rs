//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1147/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1147<F: Float>(t508: F, t5753: F, t234: F, t1981: F, t582: F, t1679: F, t619: F, t112: F, t599: F, t630: F, t640: F, t2073: F, t68: F, t1695: F, t17942: F, t510: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18289 = t508 * t5753;
    let t18322 = 88.0 / 9.0 * t234;
    let t18350 = t1981 * t582;
    let t18351 = t1679 * t619;
    let t18392 = t234 * t112;
    let t18393 = 11.0 / 9.0 * t18392;
    let t18394 = t599 * t630;
    let t18395 = t18394 * t640;
    let t18397 = t68 * t2073;
    let t18434 = t17942 * t510 * t1695;
    (t18289, t18322, t18350, t18351, t18393, t18394, t18395, t18397, t18434)
}
