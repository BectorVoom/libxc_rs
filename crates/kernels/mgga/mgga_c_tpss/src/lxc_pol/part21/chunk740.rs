//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 740/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk740<F: Float>(t3692: F, t1389: F, t219: F, t1395: F, t818: F, t2406: F, t2157: F, t220: F, t73: F, t1378: F, t246: F, t768: F, t1388: F, t1379: F, t229: F, t2415: F, t339: F, t3630: F, t3665: F, t783: F, t813: F) -> (F, F, F, F, F, F, F, F) {
    let t3693 = param_beta * t3692;
    let t3695 = t1389 * t219;
    let t3698 = t1395 * t818;
    let t3699 = t2406 * t3698;
    let t3703 = t220 * t73 * t2157;
    let t3704 = t246 * t1378;
    let t3713 = t220 * t73 * t768;
    let t3716 = t768 * t1388;
    let t3721 = -t1379 * t2415 * t339 + t220 * t229 * t3692 - t339 * t3665 * t813 - t339 * t3716 * t783 + 2.0 * t3630 * t3703 * t3704 - t3704 * t3713 * t783;
    (t3693, t3695, t3699, t3703, t3704, t3713, t3716, t3721)
}
