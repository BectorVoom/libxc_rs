//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1262/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1262<F: Float>(t17942: F, t223: F, t764: F, t17946: F, t2153: F, t238: F, t5543: F, t2149: F, t1695: F, t212: F, t60720: F, t17974: F, t2395: F, t2376: F, t339: F, t5557: F) -> (F, F, F, F, F, F, F) {
    let t61062 = t17942 * t223;
    let t61063 = t61062 * t764;
    let t61065 = t17946 * t2153;
    let t61072 = t5543 * t238;
    let t61073 = t61072 * t2149;
    let t61079 = t60720 * t212 * t1695;
    let t61080 = 455.0 / 1296.0 * t61079;
    let t61081 = t17974 * t2395;
    let t61086 = t339 * t5557 * t2376;
    (t61062, t61063, t61065, t61073, t61080, t61081, t61086)
}
