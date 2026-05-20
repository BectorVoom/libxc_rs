//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1509;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1510;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta492<F: Float>(t54460: F, t54462: F, t54467: F, t57235: F, t54477: F, t39655: F, t39658: F, t39660: F, t39844: F, t39856: F, t40224: F, t40228: F, t40230: F, t1347: F, t1819: F, t1821: F, t19708: F, t19715: F, t20416: F, t20536: F, t20544: F, t20547: F, t20550: F, t225: F, t3843: F, t40253: F, t5278: F, t5279: F, t546: F, t548: F, t6347: F, t6404: F, t6408: F, t6411: F, t79921: F, t79984: F, t80021: F, t80101: F, t80102: F, t80104: F, t80105: F, t80108: F, t80109: F, t80111: F, t550: F, t1336: F, t1380: F, t19654: F, t19739: F, t19743: F, t19810: F, t19815: F, t20473: F, t20554: F, t20568: F, t20632: F, t20638: F, t20643: F, t20645: F, t3897: F, t5234: F, t5334: F, t5344: F, t5348: F, t6415: F, t6454: F, t80085: F) -> (F, F, F, F, F, F, F) {
        let (t80112, t80113, t80114, t80115, t80116, t80117) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1509::<F>(t54460, t54462, t54467, t57235, t54477, t39655, t39658, t39660, t39844, t39856, t40224, t40228, t40230);
        let t80150 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1510::<F>(t1347, t1819, t1821, t19708, t19715, t20416, t20536, t20544, t20547, t20550, t225, t3843, t40253, t5278, t5279, t546, t548, t6347, t6404, t6408, t6411, t79921, t79984, t80021, t80101, t80102, t80104, t80105, t80108, t80109, t80111, t80117);
        let (t80151, t80164) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1511::<F>(t550, t80150, t1336, t1380, t19654, t19739, t19743, t19810, t19815, t20473, t20554, t20568, t20632, t20638, t20643, t20645, t3897, t5234, t5334, t5344, t5348, t6415, t6454, t80085);
    (t80112, t80113, t80114, t80115, t80116, t80151, t80164)
}
