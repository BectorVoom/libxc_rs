//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1366/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1366<F: Float>(t54460: F, t54462: F, t54467: F, t57235: F, t54477: F, t39655: F, t39658: F, t39660: F, t39844: F, t39856: F, t40224: F, t40228: F, t40230: F, t1347: F, t1819: F, t1821: F, t19708: F, t19715: F, t20416: F, t20536: F, t20544: F, t20547: F, t20550: F, t225: F, t3843: F, t40253: F, t5278: F, t5279: F, t546: F, t548: F, t6347: F, t6404: F, t6408: F, t6411: F, t79921: F, t79984: F, t80021: F, t80101: F, t80102: F, t80104: F, t80105: F, t80108: F, t80109: F, t80111: F) -> (F, F, F, F, F, F) {
    let t80112 = 960.0 * t54460;
    let t80113 = 480.0 * t54462;
    let t80114 = 0.4101607543286562663e4 * t54467;
    let t80115 = 0.65061487801810439052e-1 * t57235;
    let t80116 = 48.0 * t54477;
    let t80117 = -t39655 - t39658 + t39660 + t39844 - t80112 - t80113 - t39856 - t80114 + t40224 + t40228 - t40230 + t80115 - t80116;
    let t80150 = -(t80101 + t80102 + t80104 + t80105 + t80108 + t80109 + t80111 + t80117) * t225 * t548 + 12.0 * t20536 * t1821 - 72.0 * t6404 * t6408 + 18.0 * t6404 * t6411 + 240.0 * t1819 * t20544 - 144.0 * t19708 * t20547 + 12.0 * t1819 * t20550 - 360.0 * t546 * t40253 * t80021 + 360.0 * t5278 * t19715 * t6347 - 36.0 * t546 * t3843 * t79921 - 48.0 * t5278 * t5279 * t20416 + 3.0 * t546 * t1347 * t79984;
    (t80112, t80113, t80114, t80115, t80116, t80150)
}
