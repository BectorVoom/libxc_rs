//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1372/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372<F: Float>(t119: F, t1315: F, t1831: F, t20479: F, t210: F, t5240: F, t554: F, t559: F, t56795: F, t74311: F, t74395: F, t74401: F, t74403: F, t74405: F, t74578: F, t74584: F, t74597: F, t74618: F, t79984: F, t80175: F) -> (F,) {
    let t80375 = 7.0 / 36.0 * t74395 - 7.0 / 192.0 * t74401 + 7.0 / 288.0 * t74403 - 35.0 / 96.0 * t74405 + t80175 * t554 * t559 / 3072.0 - 119.0 / 288.0 * t56795 - t5240 * t20479 / 192.0 - t74311 * t1831 / 192.0 - 7.0 / 1152.0 * t74578 + 7.0 / 384.0 * t74584 - 7.0 / 96.0 * t74597 + 7.0 / 48.0 * t74618 - t1315 * t210 * t119 * t79984 / 48.0;
    (t80375,)
}
