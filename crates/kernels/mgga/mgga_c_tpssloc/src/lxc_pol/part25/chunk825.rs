//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 825/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk825<F: Float>(t3247: F, t405: F, t974: F, t11147: F, t461: F, t9288: F, t457: F, t63: F, t221: F, t456: F, t1186: F, t698: F, t1174: F, t135: F, t3471: F, t11168: F, t4908: F) -> (F, F, F, F, F, F, F) {
    let t11545 = 1.0 / t405 / t3247;
    let t11546 = t974 * t11545;
    let t11547 = t461 * t11147;
    let t11548 = t11547 * t9288;
    let t11549 = t11546 * t11548;
    let t11552 = t63 * t457;
    let t11553 = t11552 * t461;
    let t11554 = t221 * t11553;
    let t11556 = 0.3086419753086419753e-3 * t456 * t11554;
    let t11557 = t698 * t1186;
    let t11558 = t1174 * t11557;
    let t11560 = t135 * t3471;
    let t11561 = t1174 * t11560;
    let t11563 = t4908 * t11168;
    (t11545, t11549, t11552, t11556, t11558, t11561, t11563)
}
