//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 249/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk249<F: Float>(t238: F, t790: F, t242: F, t232: F, t228: F, t230: F, t234: F, t339: F) -> (F, F, F, F) {
    let t791 = t790 * t238;
    let t792 = t791 * t242;
    let t794 = 7.0 / 4608.0 * t232 * t792;
    let t795 = t228 * t230;
    let t797 = t339 * t795 * t234;
    (t792, t794, t795, t797)
}
