//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1252/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1252<F: Float>(t5506: F, t619: F, t1679: F, t2049: F, t789: F, t582: F, t7682: F, t1982: F, t1981: F, t1993: F, t116: F, t18403: F, t1275: F, t6071: F, t1138: F, t1883: F, t61283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61938 = t5506 * t619;
    let t61942 = t1679 * t2049;
    let t61976 = 1232.0 / 27.0 * t789;
    let t62007 = t7682 * t582;
    let t62020 = t1679 * t1982;
    let t62024 = t1981 * t1993;
    let t62124 = t116 * t18403;
    let t63175 = t1275 * t6071;
    let t63200 = t1883 * t61283 * t1138;
    (t61938, t61942, t61976, t62007, t62020, t62024, t62124, t63175, t63200)
}
