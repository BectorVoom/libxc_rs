//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1182/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1182<F: Float>(t2074: F, t61873: F, t18394: F, t2100: F, t68: F, t7594: F, t5506: F, t619: F, t1679: F, t2049: F, t582: F, t7682: F, t1982: F, t1981: F, t1993: F, t19050: F, t546: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61874 = t61873 * t2074;
    let t61876 = t18394 * t2100;
    let t61877 = t68 * t7594;
    let t61938 = t5506 * t619;
    let t61942 = t1679 * t2049;
    let t62007 = t7682 * t582;
    let t62020 = t1679 * t1982;
    let t62024 = t1981 * t1993;
    let t62171 = t546 * t19050;
    (t61874, t61876, t61877, t61938, t61942, t62007, t62020, t62024, t62171)
}
