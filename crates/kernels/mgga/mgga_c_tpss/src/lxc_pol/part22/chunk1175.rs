//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1175/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1175<F: Float>(t1364: F, t2433: F, t14179: F, t782: F, t3664: F, t783: F, t18495: F, t5736: F, t10179: F, t1771: F, t5570: F, t10164: F, t1765: F, t18444: F, t339: F, t789: F) -> (F, F, F, F, F, F, F) {
    let t44474 = t1364 * t2433;
    let t44584 = t14179 * t782;
    let t44610 = t783 * t3664;
    let t60649 = t5736 * t18495;
    let t60653 = t1771 * t5570 * t10179;
    let t60684 = t1765 * t10164;
    let t60695 = t339 * t18444 * t789;
    (t44474, t44584, t44610, t60649, t60653, t60684, t60695)
}
