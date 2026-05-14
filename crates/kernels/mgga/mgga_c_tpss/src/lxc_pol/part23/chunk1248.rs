//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1248/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1248<F: Float>(t2376: F, t339: F, t5550: F, t785: F, t17960: F, t2372: F, t17954: F, t789: F, t2165: F, t2367: F, t17942: F, t223: F, t764: F, t17946: F, t2153: F, t238: F, t5543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61050 = t339 * t5550 * t2376;
    let t61051 = t61050 * t785;
    let t61054 = t17960 * t2372;
    let t61057 = t339 * t17954 * t789;
    let t61058 = t61057 * t2165;
    let t61060 = t17960 * t2367;
    let t61062 = t17942 * t223;
    let t61063 = t61062 * t764;
    let t61065 = t17946 * t2153;
    let t61072 = t5543 * t238;
    (t61050, t61051, t61054, t61058, t61060, t61062, t61063, t61065, t61072)
}
