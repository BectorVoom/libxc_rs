//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1339;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1340;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1341;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1342;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1343;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1344;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1345;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta220<F: Float>(t1297: F, t1390: F, t193: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5167: F, t5169: F, t5187: F, t5263: F, t5265: F, t5267: F, t5268: F, t5269: F, t533: F, t5356: F, t5165: F, t113: F, t1266: F, t1271: F, t1393: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t2314: F, t4026: F, t4028: F, t4034: F, t4037: F, t4073: F, t4077: F, t510: F, t5107: F, t5118: F, t513: F, t574: F, t650: F, t652: F, t672: F, t3: F, t112: F, t1851: F, t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t577: F, t2218: F, t2220: F, t2222: F, t2224: F, t2226: F, t2228: F, t2232: F, t1437: F, t1409: F, t65: F, t11: F, t2219: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5360 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1339::<F>(t1297, t1390, t193, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5167, t5169, t5187, t5263, t5265, t5267, t5268, t5269, t533, t5356);
        let (t5361, t5363) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1340::<F>(t5165, t5360, t113, t1266, t1271, t1393, t1442, t1459, t1774, t1778, t1849, t2314, t4026, t4028, t4034, t4037, t4073, t4077, t510, t5107, t5118, t513, t574, t650, t652, t672);
        let (t5364, t5371) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1341::<F>(t3, t5363, t112, t1851);
        let (t5376, t5381, t5385) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1342::<F>(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2218, t2220, t2222, t2224, t2226, t2228, t2232);
        let t5389 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1343::<F>(t1437);
        let t5392 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1344::<F>(t1409);
        let (t5393, t5396) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1345::<F>(t5392, t65, t11, t2219);
        let t5397 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1346::<F>(t5396);
    (t5361, t5363, t5364, t5371, t5376, t5381, t5385, t5389, t5392, t5393, t5396, t5397)
}
