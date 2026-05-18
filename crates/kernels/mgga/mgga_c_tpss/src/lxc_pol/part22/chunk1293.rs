//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1293/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1293<F: Float>(t339: F, t5550: F, t790: F, t2179: F, t64: F, t8275: F, t2376: F, t785: F, t17960: F, t2372: F, t17954: F, t789: F) -> (F, F, F, F, F, F, F) {
    let t61033 = t339 * t5550 * t790;
    let t61034 = t61033 * t2179;
    let t61038 = t8275 * t64;
    let t61050 = t339 * t5550 * t2376;
    let t61051 = t61050 * t785;
    let t61054 = t17960 * t2372;
    let t61057 = t339 * t17954 * t789;
    (t61033, t61034, t61038, t61050, t61051, t61054, t61057)
}
