//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 734/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk734<F: Float>(t1388: F, t768: F, t1379: F, t220: F, t229: F, t2415: F, t339: F, t3630: F, t3665: F, t3692: F, t3703: F, t3704: F, t3713: F, t783: F, t813: F) -> (F, F) {
    let t3716 = t768 * t1388;
    let t3721 = -t1379 * t2415 * t339 + t220 * t229 * t3692 - t339 * t3665 * t813 - t339 * t3716 * t783 + F::new(2.0) * t3630 * t3703 * t3704 - t3704 * t3713 * t783;
    (t3716, t3721)
}
