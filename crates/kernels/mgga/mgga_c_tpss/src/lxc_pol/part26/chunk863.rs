//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 863/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk863<F: Float>(t6130: F, t1388: F, t1705: F, t935: F, t1395: F, t1702: F, t5572: F, t1378: F, t226: F, t5577: F, t1708: F, t228: F, t1396: F, t1707: F, t1710: F, t253: F, t5565: F, t5571: F) -> (F, F, F, F, F, F, F, F) {
    let t6131 = param_beta * t6130;
    let t6134 = t1705 * t1388;
    let t6135 = t6134 * t935;
    let t6137 = t1702 * t1395;
    let t6138 = t5572 * t6137;
    let t6142 = t1702 * t1378 * t226;
    let t6143 = t5577 * t6142;
    let t6146 = t1708 * t228 * t6130;
    let t6148 = -t1396 * t5565 - t1707 * t6146 - t1710 * t6135 + t253 * t6131 + 2.0 * t5571 * t6138 + t5571 * t6143;
    (t6131, t6134, t6135, t6137, t6138, t6143, t6146, t6148)
}
