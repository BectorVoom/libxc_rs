//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1420;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1421;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1422;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1423;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1424;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta240<F: Float>(t1060: F, t5936: F, t3201: F, t5928: F, t383: F, t5914: F, t1058: F, t1610: F, t1630: F, t1632: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F, t5929: F, t5933: F, t1055: F, t1052: F, t1635: F, t388: F, t4557: F, t4660: F, t5849: F, t5851: F, t5915: F, t5920: F, t1637: F, t1070: F, t193: F, t3216: F, t336: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F, t25: F, t265: F, t394: F, t5669: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t40: F, t5397: F, t5398: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3242: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5937, t5939, t5941, t5943) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1420::<F>(t1060, t5936, t3201, t5928, t383, t5914, t1058, t1610, t1630, t1632, t3186, t3200, t353, t384, t4669, t5903, t5929, t5933);
        let (t5944, t5946, t5950) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1421::<F>(t1055, t5943, t1052, t1635, t388, t4557, t4660, t5849, t5851, t5915, t5920, t1637);
        let t5954 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1422::<F>(t1070, t193, t3216, t336, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814, t5946, t5950);
        let (t5955, t5962) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1423::<F>(t25, t265, t394, t5669, t5954, t1408, t1409, t1534, t1642, t396, t40, t5397, t5398, dens_threshold, rho0, zeta_threshold);
        let t5966 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1424::<F>(t5397);
        let t5971 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1425::<F>(t3242, t5392);
    (t5937, t5939, t5941, t5943, t5944, t5946, t5950, t5955, t5962, t5966, t5971)
}
