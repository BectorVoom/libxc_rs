//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta857 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3108;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3109;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3110;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3111;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3112;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3113;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta857<F: Float>(t43780: F, t43782: F, t43816: F, t43895: F, t50968: F, t50970: F, t50972: F, t50978: F, t51039: F, t51041: F, t64028: F, t64031: F, t64033: F, t64042: F, t64045: F, t51043: F, t51051: F, t51053: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F, t64309: F, t64325: F, t64342: F, t64358: F, t64374: F, t1117: F, t51460: F, t51638: F, t3313: F, t3315: F, t63287: F, t15061: F, t50819: F, t11361: F, t11365: F, t1137: F, t11420: F, t1148: F, t1155: F, t1156: F, t15126: F, t15136: F, t15146: F, t15179: F, t15219: F, t15229: F, t18603: F, t3332: F, t3333: F, t3334: F, t3357: F, t3359: F, t3377: F, t3401: F, t44188: F, t4840: F, t4862: F, t51371: F, t51385: F, t51651: F, t51677: F, t6037: F, t6053: F, t6069: F, t6085: F, t6088: F, t64261: F, t64292: F, t300: F, t63457: F, t63506: F, t63561: F, t63611: F, t63715: F, t63760: F, t64260: F, t1254: F, t5091: F, t11282: F, t6084: F, t1164: F, t14854: F, t18926: F, t3411: F, t14858: F, t4884: F, t18915: F, t3419: F, t18283: F, t14855: F, t4869: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t64389 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3108::<F>(t43780, t43782, t43816, t43895, t50968, t50970, t50972, t50978, t51039, t51041, t64028, t64031, t64033, t64042, t64045);
        let t64406 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3109::<F>(t51043, t51051, t51053, t63355, t63359, t63361, t63365, t63370, t63374, t63380, t63382, t63384, t63388, t63392, t63396);
        let t64422 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3110::<F>(t63398, t63400, t63404, t63408, t63412, t63417, t63422, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let (t64425, t64433, t64436) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3111::<F>(t64309, t64325, t64342, t64358, t64374, t64389, t64406, t64422, t1117, t51460, t51638, t3313, t3315, t63287);
        let (t64441, t64442) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3112::<F>(t15061, t50819, t11361, t11365, t1137, t11420, t1148, t1155, t1156, t15126, t15136, t15146, t15179, t15219, t15229, t18603, t3332, t3333, t3334, t3357, t3359, t3377, t3401, t44188, t4840, t4862, t51371, t51385, t51651, t51677, t6037, t6053, t6069, t6085, t6088, t64261, t64292, t64425, t64433, t64436);
        let (t64446, t64447, t64451) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3113::<F>(t300, t63457, t63506, t63561, t63611, t63715, t63760, t64260, t64442, t1254, t5091, t11282, t6084);
        let (t64454, t64456, t64458, t64460, t64462, t64464) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3114::<F>(t1164, t14854, t64451, t18926, t3411, t14858, t4884, t18915, t3419, t18283, t14855, t4869);
    (t64425, t64433, t64436, t64441, t64446, t64447, t64454, t64456, t64458, t64460, t64462, t64464)
}
