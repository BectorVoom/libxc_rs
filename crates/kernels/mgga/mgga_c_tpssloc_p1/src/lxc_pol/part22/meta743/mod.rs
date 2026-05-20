//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2464;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2465;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2466;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta743<F: Float>(t5866: F, t6739: F, t1022: F, t14211: F, t14218: F, t360: F, t1615: F, t883: F, t1539: F, t4649: F, t17906: F, t4644: F, t17607: F, t4571: F, t1011: F, t1019: F, t69923: F, t1025: F, t1622: F, t21405: F, t21580: F, t21609: F, t3048: F, t3117: F, t43211: F, t61659: F, t61663: F, t61665: F, t61710: F, t1040: F, t21482: F, t10876: F, t21396: F, t248: F, t3101: F, t1041: F, t21138: F, t3051: F, t10403: F, t10408: F, t1046: F, t18014: F, t3071: F, t42388: F, t43361: F, t4338: F, t4343: F, t4636: F, t49743: F, t5873: F, t5880: F, t61675: F, t62079: F, t21134: F, t14508: F, t17667: F, t14085: F, t17962: F, t21597: F, t3109: F, t42354: F, t4641: F, t48431: F, t50302: F, t5857: F, t5875: F, t61677: F, t61695: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t70081, t70082, t70086, t70100, t70106, t70122, t70132) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2464::<F>(t5866, t6739, t1022, t14211, t14218, t360, t1615, t883, t1539, t4649, t17906, t4644);
        let t70151 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2465::<F>(t17607, t4571, t1011, t1019, t69923, t1025, t1622, t21405, t21580, t21609, t3048, t3117, t43211, t61659, t61663, t61665, t61710, t70132);
        let t70189 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2466::<F>(t1040, t21482, t10876, t21396, t248, t3101, t1041, t21138, t3051, t10403, t10408, t1046, t14211, t17607, t18014, t3071, t42388, t43361, t4338, t4343, t4636, t49743, t5873, t5880, t61675, t62079, t70106);
        let t70211 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2467::<F>(t1041, t21134, t248, t3051, t14508, t17667, t14085, t1622, t17962, t21405, t21580, t21597, t3109, t3117, t42354, t4641, t48431, t50302, t5857, t5875, t61677, t61695);
    (t70081, t70082, t70086, t70100, t70106, t70122, t70151, t70189, t70211)
}
