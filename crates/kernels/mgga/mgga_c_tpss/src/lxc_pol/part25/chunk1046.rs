//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1046/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1046<F: Float>(t13335: F, t836: F, t861: F, t141: F, t4573: F, t8444: F, t581: F, t2457: F, t128: F) -> (F, F, F, F) {
    let t14452 = t836 * t13335;
    let t14453 = t861 * t14452;
    let t14454 = t141 * t14453;
    let t14456 = t8444 * t4573;
    let t14457 = t14456 * t581;
    let t14458 = t2457 * t14457;
    let t14459 = t128 * t14458;
    (t14452, t14454, t14457, t14459)
}
