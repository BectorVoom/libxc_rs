//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 905/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk905<F: Float>(t109: F, t1873: F, t26114: F, t4072: F, t88: F, t6534: F, t7676: F, t2314: F, t7467: F, t5113: F, t1453: F, t22470: F, t666: F, t22473: F, t4067: F, t6530: F, t22469: F, t22471: F) -> (F, F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t26116 = 2.0 * t26114 * t1873;
    let t26117 = t88 * t4072;
    let t26119 = 2.0 * t26117 * t1873;
    let t26121 = 2.0 * t7676 * t6534;
    let t26123 = 2.0 * t2314 * t7467;
    let t26125 = 2.0 * t5113 * t7467;
    let t26127 = t22470 * t1453;
    let t26129 = t1453 * t666;
    let t26130 = t22473 * t26129;
    let t26132 = t6530 * t4067;
    let t26135 = piecewise3(t110, 0.0, t22469 + t22471 / 3.0 + t26127 / 3.0 + t26130 / 4.0 - t26132 / 8.0);
    (t26116, t26117, t26119, t26121, t26123, t26125, t26135)
}
