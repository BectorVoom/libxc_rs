//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1491;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1492;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1493;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta256<F: Float>(t3897: F, t6388: F, t1825: F, t5348: F, t1380: F, t6415: F, t6420: F, t553: F, t6434: F, t1336: F, t1814: F, t1838: F, t1840: F, t5234: F, t544: F, t564: F, t6378: F, t1378: F, t1375: F, t1843: F, t5215: F, t5321: F, t568: F, t6362: F, t6364: F, t6435: F, t6440: F, t1297: F, t1390: F, t193: F, t2486: F, t3701: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t3924: F, t533: F, t6324: F, t6329: F, t6330: F, t6347: F, t6399: F, t6400: F, t6323: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t4028: F, t510: F, t513: F, t5450: F, t5457: F, t5460: F, t5494: F, t574: F, t6287: F, t6295: F, t652: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6448, t6451, t6454, t6456, t6458, t6460) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1491::<F>(t3897, t6388, t1825, t5348, t1380, t6415, t6420, t553, t6434, t1336, t1814, t1838, t1840, t5234, t544, t564, t6378);
        let t6461 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1492::<F>(t1378, t6460);
        let (t6463, t6467) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1493::<F>(t1375, t1843, t5215, t5321, t568, t6362, t6364, t6435, t6440, t6461, t1297, t1390, t193, t2486, t3701, t3819, t3821, t3823, t3825, t3832, t3836, t3924, t533, t6324, t6329, t6330, t6347, t6399, t6400);
        let (t6468, t6470) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1494::<F>(t6323, t6467, t113, t1442, t1459, t1774, t1778, t1849, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t652);
    (t6448, t6451, t6454, t6456, t6458, t6460, t6461, t6463, t6468, t6470)
}
