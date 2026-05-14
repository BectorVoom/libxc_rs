//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 931/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk931<F: Float>(t116135: F, t127107: F, t128306: F, t128375: F, t128377: F, t128381: F, t128383: F, t128385: F, t129164: F, t2039: F, t2096: F, t27188: F, t29205: F, t29211: F, t29243: F, t29247: F, t29501: F, t29848: F, t32350: F, t34150: F, t5460: F, t652: F, t7042: F, t7266: F, t7458: F, t7801: F, t7989: F, t8103: F, t8690: F) -> (F,) {
    let t130326 = -2.0 * t2039 * t29848 * t652 - 4.0 * t652 * t7801 * t8103 - 6.0 * t116135 * t29247 + t129164 * t2096 - 4.0 * t27188 * t7989 - 4.0 * t29205 * t7266 - 2.0 * t29211 * t7266 + 2.0 * t29243 * t8690 - 4.0 * t29501 * t7042 - 4.0 * t32350 * t5460 - 4.0 * t34150 * t7458 - t127107 - t128306 - t128375 - t128377 - t128381 - t128383 - t128385;
    (t130326,)
}
