//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 821/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk821<F: Float>(t6337: F, t1395: F, t1805: F, t5572: F, t1378: F, t226: F, t5577: F, t1708: F, t228: F, t1396: F, t1707: F, t1809: F, t253: F, t5571: F, t5834: F, t6135: F) -> (F, F, F, F, F, F) {
    let t6338 = param_beta * t6337;
    let t6342 = t1805 * t1395;
    let t6343 = t5572 * t6342;
    let t6348 = t5577 * t1805 * t1378 * t226;
    let t6351 = t1708 * t228 * t6337;
    let t6353 = -t1396 * t5834 - t1707 * t6351 - t1809 * t6135 + t253 * t6338 + 2.0 * t5571 * t6343 + t5571 * t6348;
    (t6338, t6342, t6343, t6348, t6351, t6353)
}
