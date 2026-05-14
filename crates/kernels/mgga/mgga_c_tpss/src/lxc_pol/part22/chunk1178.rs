//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1178/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1178<F: Float>(t18464: F, t3354: F, t18480: F, t5570: F, t31297: F, t522: F, t2436: F, t580: F, t1699: F, t8202: F, t339: F, t5550: F, t790: F, t2179: F, t64: F, t8275: F) -> (F, F, F, F, F, F, F, F) {
    let t60752 = t18464 * t3354;
    let t60778 = t18480 * t5570;
    let t60811 = t31297 * t522;
    let t60960 = t2436 * t580;
    let t61024 = t1699 * t8202;
    let t61033 = t339 * t5550 * t790;
    let t61034 = t61033 * t2179;
    let t61038 = t8275 * t64;
    (t60752, t60778, t60811, t60960, t61024, t61033, t61034, t61038)
}
