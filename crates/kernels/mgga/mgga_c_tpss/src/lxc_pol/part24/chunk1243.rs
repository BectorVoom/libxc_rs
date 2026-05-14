//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1243/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1243<F: Float>(t1699: F, t8202: F, t339: F, t5550: F, t790: F, t64: F, t8275: F, t2376: F, t785: F, t17954: F, t789: F, t17942: F, t223: F, t764: F, t238: F, t5543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61024 = t1699 * t8202;
    let t61025 = 595.0 / 5184.0 * t61024;
    let t61033 = t339 * t5550 * t790;
    let t61038 = t8275 * t64;
    let t61050 = t339 * t5550 * t2376;
    let t61051 = t61050 * t785;
    let t61057 = t339 * t17954 * t789;
    let t61062 = t17942 * t223;
    let t61063 = t61062 * t764;
    let t61072 = t5543 * t238;
    (t61025, t61033, t61038, t61050, t61051, t61057, t61062, t61063, t61072)
}
