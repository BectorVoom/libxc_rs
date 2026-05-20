//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk959;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk960;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk961;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk962;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta275<F: Float>(t1378: F, t20661: F, t20594: F, t562: F, t1834: F, t6361: F, t1375: F, t1843: F, t20029: F, t20044: F, t20060: F, t20420: F, t20602: F, t20609: F, t20613: F, t5215: F, t5321: F, t568: F, t6440: F, t6461: F, t12044: F, t12046: F, t12048: F, t12053: F, t12055: F, t12057: F, t12059: F, t1297: F, t1390: F, t1799: F, t193: F, t20067: F, t20372: F, t20398: F, t20416: F, t20520: F, t3918: F, t533: F, t9780: F, t9789: F, t5127: F, t6347: F, t1845: F, t6324: F, t5122: F, t6330: F, t12087: F, t12094: F, t12103: F, t12105: F, t12109: F, t12114: F, t12461: F, t20523: F, t20524: F, t5126: F, t9793: F, t9797: F, t9820: F, t9824: F, t12116: F, t12118: F, t12121: F, t12123: F, t12133: F, t12141: F, t20526: F, t20527: F, t20528: F, t20529: F, t20530: F, t20532: F, t9853: F, t9859: F, t20371: F, t1458: F, t6287: F, t1774: F, t5493: F, t20347: F, t510: F, t16578: F, t12861: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20662, t20670, t20672, t20675) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk959::<F>(t1378, t20661, t20594, t562, t1834, t6361, t1375, t1843, t20029, t20044, t20060, t20420, t20602, t20609, t20613, t5215, t5321, t568, t6440, t6461);
        let t20679 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk960::<F>(t12044, t12046, t12048, t12053, t12055, t12057, t12059, t1297, t1390, t1799, t193, t20067, t20372, t20398, t20416, t20520, t20675, t3918, t533, t9780, t9789);
        let (t20684, t20692) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk961::<F>(t5127, t6347, t1845, t6324, t5122, t6330, t12087, t12094, t12103, t12105, t12109, t12114, t12461, t193, t20523, t20524, t5126, t533, t9793, t9797, t9820, t9824);
        let t20696 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk962::<F>(t12116, t12118, t12121, t12123, t12133, t12141, t20526, t20527, t20528, t20529, t20530, t20532, t3918, t5122, t6347, t9853, t9859);
        let (t20698, t20702, t20717, t20720, t20723, t20724) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk963::<F>(t20371, t20679, t20692, t20696, t1458, t6287, t1774, t5493, t20347, t510, t16578, t12861);
    (t20662, t20670, t20672, t20675, t20684, t20698, t20702, t20717, t20720, t20723, t20724)
}
