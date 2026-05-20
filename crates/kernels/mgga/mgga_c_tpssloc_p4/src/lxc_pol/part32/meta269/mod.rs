//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1225;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1226;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1227;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1228;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1229;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1230;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta269<F: Float>(t671: F, t6867: F, t6869: F, t6871: F, t7264: F, t7266: F, t113: F, t1266: F, t1393: F, t2114: F, t2165: F, t2167: F, t510: F, t574: F, t650: F, t652: F, t6522: F, t6524: F, t6527: F, t6537: F, t672: F, t6877: F, t6882: F, t6998: F, t7001: F, t7271: F, t7408: F, t3: F, t112: F, t2169: F, t577: F, t7014: F, t7017: F, t7019: F, t33: F, t3953: F, t1437: F, t79: F, t72: F, t1410: F, t605: F, t1433: F, t71: F, t1874: F, t4028: F, t1458: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7412, t7415) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1225::<F>(t671, t6867, t6869, t6871, t7264, t7266, t113, t1266, t1393, t2114, t2165, t2167, t510, t574, t650, t652, t6522, t6524, t6527, t6537, t672, t6877, t6882, t6998, t7001, t7271, t7408);
        let (t7416, t7423) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1226::<F>(t3, t7415, t112, t2169);
        let (t7426, t7428) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1227::<F>(t577, t671, t7014, t7017, t7019, t7415, t7423, t33, t3953);
        let (t7431, t7432) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1228::<F>(t1437, t79, t72);
        let t7435 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1229::<F>(t1410, t605);
        let t7445 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1230::<F>(t1433, t71);
        let (t7457, t7458) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1231::<F>(t1874, t4028, t1458, t89);
    (t7412, t7415, t7416, t7423, t7426, t7428, t7431, t7432, t7435, t7445, t7457, t7458)
}
