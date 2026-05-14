//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1257/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1257<F: Float>(t4019: F, t993: F, t1275: F, t5779: F, t1789: F, t3398: F, t18495: F, t5736: F, t10179: F, t1771: F, t5570: F, t1219: F, t5731: F, t10164: F, t1765: F, t18444: F, t339: F, t789: F) -> (F, F, F, F, F, F, F, F) {
    let t45417 = t993 * t4019;
    let t60626 = t1275 * t5779;
    let t60628 = t3398 * t1789;
    let t60649 = t5736 * t18495;
    let t60653 = t1771 * t5570 * t10179;
    let t60659 = t1219 * t5731;
    let t60684 = t1765 * t10164;
    let t60685 = 595.0 / 5184.0 * t60684;
    let t60695 = t339 * t18444 * t789;
    (t45417, t60626, t60628, t60649, t60653, t60659, t60685, t60695)
}
