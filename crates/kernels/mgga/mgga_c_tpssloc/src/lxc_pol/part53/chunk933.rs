//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 933/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk933<F: Float>(t40611: F, t8807: F, t115925: F, t120719: F, t120721: F, t120728: F, t120730: F, t120735: F, t122654: F, t124538: F, t1266: F, t12725: F, t1799: F, t22574: F, t24432: F, t26114: F, t26161: F, t26163: F, t26179: F, t26559: F, t26872: F, t27150: F, t32111: F, t32200: F, t32213: F, t32220: F, t33916: F, t4028: F, t510: F, t7042: F, t7216: F, t7458: F, t7685: F, t8721: F) -> (F,) {
    let t124580 = t8807 * t40611;
    let t124584 = 3.0 * t7685 * t32111 - t33916 * t1266 - 3.0 * t7685 * t32213 - 4.0 * t4028 * t32200 - 4.0 * t26114 * t8721 - 4.0 * t26179 * t8721 - 4.0 * t7458 * t32220 - 4.0 * t7042 * t27150 - 4.0 * t12725 * t8721 - t120719 - t120721 - t120728 - t120730 - t120735 - 2.0 * t124538 * t510 + 4.0 * t122654 * t26559 - 6.0 * t115925 * t26872 - 6.0 * t22574 * t24432 * t1799 * t7216 - 6.0 * t26161 * t124580 * t26163;
    (t124584,)
}
