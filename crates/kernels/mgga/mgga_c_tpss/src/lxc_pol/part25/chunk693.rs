//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 693/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk693<F: Float>(t2073: F, t4645: F, t1324: F, t2083: F, t100: F, t4577: F, t1299: F, t1329: F, t2091: F, t108: F, t105: F, t109: F, t1327: F, t1330: F, t97: F, tau1: F) -> (F, F, F, F, F, F, F, F) {
    let t4646 = t2073 * t4645;
    let t4649 = t1324 * t1324;
    let t4650 = t2083 * t4649;
    let t4653 = t100 * t4577;
    let t4656 = tau1 * t1299;
    let t4661 = t1329 * t1329;
    let t4662 = t2091 * t4661;
    let t4665 = -t4577;
    let t4666 = t108 * t4665;
    let t4669 = 10.0 / 9.0 * t97 * t4650 + 5.0 / 3.0 * t97 * t4653 + 40.0 / 9.0 * t4656 * t109 - 50.0 / 9.0 * t1327 * t1330 + 10.0 / 9.0 * t105 * t4662 + 5.0 / 3.0 * t105 * t4666;
    (t4646, t4649, t4650, t4653, t4656, t4661, t4665, t4669)
}
