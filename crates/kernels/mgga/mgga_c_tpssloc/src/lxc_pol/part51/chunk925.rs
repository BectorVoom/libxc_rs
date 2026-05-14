//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 925/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk925<F: Float>(t25249: F, t829: F, t6646: F, t22986: F, t22996: F, t4283: F, t1888: F, t1484: F, t23153: F, t6637: F, t6552: F, t23168: F, t7521: F, t4119: F, t6638: F, t22893: F, t7520: F) -> (F, F, F, F, F, F) {
    let t25299 = t25249 * t829;
    let t25300 = t6646 * t25299;
    let t25301 = t22986 * t25300;
    let t25303 = t22996 * t4283;
    let t25304 = t1888 * t25303;
    let t25306 = t23153 * t1484;
    let t25307 = t6637 * t25306;
    let t25308 = t6552 * t25307;
    let t25310 = t23168 * t7521;
    let t25312 = t6638 * t4119;
    let t25313 = t6637 * t25312;
    let t25314 = t6552 * t25313;
    let t25316 = t22893 * t7520;
    (t25301, t25304, t25308, t25310, t25314, t25316)
}
