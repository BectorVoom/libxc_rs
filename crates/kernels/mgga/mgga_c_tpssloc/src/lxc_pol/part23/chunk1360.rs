//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1360/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1360<F: Float>(t28: F, t1302: F, t19618: F, t20390: F, t3711: F, t39877: F, t5178: F, t5966: F, t77953: F, t79873: F, t79878: F, t79970: F, t1297: F, t1390: F, t1845: F, t193: F, t20077: F, t20356: F, t3701: F, t3918: F, t39604: F, t39606: F, t39608: F, t39615: F, t39635: F, t39655: F, t533: F, t6347: F, t79942: F, t79946: F, t79947: F, t79952: F, t79953: F, t79954: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t79982 = piecewise3(t29, 0.0, -56.0 / 81.0 * t39877 * t79873 + 16.0 / 9.0 * t19618 * t5966 - 2.0 / 3.0 * t3711 * t79878 - 8.0 / 9.0 * t5178 * t20390 + 2.0 / 3.0 * t1302 * t77953);
    let t79984 = t79970 / 2.0 + t79982 / 2.0;
    let t79988 = 24.0 * t1390 * t1845 * t193 * t20356 - 3.0 * t193 * t3701 * t533 * t79947 + 3.0 * t1297 * t193 * t79984 - 18.0 * t20077 * t3918 * t6347 + t39604 + t39606 + t39608 + t39615 - t39635 - t39655 + t79942 - t79946 + t79952 + t79953 + t79954;
    (t79984, t79988)
}
