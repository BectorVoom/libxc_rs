//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1152/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1152<F: Float>(t114: F, t3499: F, t5522: F, t112: F, t234: F, t599: F, t630: F, t640: F, t2073: F, t68: F, t2074: F, t2100: F, t5527: F) -> (F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t18388 = 4.0 * t3499 * t5522;
    let t18392 = t234 * t112;
    let t18393 = 11.0 / 9.0 * t18392;
    let t18394 = t599 * t630;
    let t18395 = t18394 * t640;
    let t18396 = 2.0 / 3.0 * t18395;
    let t18397 = t68 * t2073;
    let t18398 = t18397 * t2074;
    let t18400 = t5527 * t2100;
    let t18403 = piecewise3(t115, 0.0, t18393 + t18396 + t18398 / 4.0 - t18400 / 8.0);
    (t18388, t18393, t18394, t18395, t18397, t18403)
}
