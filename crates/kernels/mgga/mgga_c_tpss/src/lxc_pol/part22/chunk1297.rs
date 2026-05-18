//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1297/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1297<F: Float>(t234: F, t630: F, t640: F, t2073: F, t599: F, t2074: F, t18394: F, t2100: F, t68: F, t7594: F, t5506: F, t619: F) -> (F, F, F, F, F, F, F) {
    let t61870 = t234 * t630;
    let t61871 = t61870 * t640;
    let t61873 = t599 * t2073;
    let t61874 = t61873 * t2074;
    let t61876 = t18394 * t2100;
    let t61877 = t68 * t7594;
    let t61938 = t5506 * t619;
    (t61870, t61871, t61873, t61874, t61876, t61877, t61938)
}
