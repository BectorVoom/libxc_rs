//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2674;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2675;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2676;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta704<F: Float>(t54467: F, t15892: F, t2535: F, t2528: F, t40225: F, t15921: F, t588: F, t40231: F, t15971: F, t40224: F, t40230: F, t54455: F, t54457: F, t54459: F, t54461: F, t54463: F, t54464: F, t54465: F, t54466: F, t12156: F, t12157: F, t12161: F, t12303: F, t1307: F, t1345: F, t1365: F, t16018: F, t16186: F, t16191: F, t16192: F, t16195: F, t16202: F, t1799: F, t1819: F, t19708: F, t1995: F, t3719: F, t3734: F, t3839: F, t3844: F, t5187: F, t5272: F, t5278: F, t5280: F, t68: F, t6924: F, t12012: F, t12147: F, t12164: F, t1347: F, t1348: F, t16176: F, t16196: F, t16199: F, t1821: F, t225: F, t3847: F, t5279: F, t5283: F, t53856: F, t54311: F, t54377: F, t54391: F, t54415: F, t54426: F, t54440: F, t54454: F, t546: F, t548: F, t550: F, t12364: F, t5234: F, t1354: F, t16288: F, t3858: F, t1351: F, t12168: F, t12413: F, t1341: F, t1343: F, t1363: F, t16101: F, t16208: F, t16224: F, t16311: F, t16394: F, t221: F, t3778: F, t3803: F, t3805: F, t3870: F, t40160: F, t5246: F, t5248: F, t5250: F, t5301: F, t53958: F, t54284: F, t54293: F, t54295: F, t820: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54468, t54470, t54472, t54473, t54475, t54476, t54478, t54479) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2674::<F>(t54467, t15892, t2535, t2528, t40225, t15921, t588, t40231, t15971, t40224, t40230, t54455, t54457, t54459, t54461, t54463, t54464, t54465, t54466);
        let t54525 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2675::<F>(t12156, t12157, t12161, t12303, t1307, t1345, t1365, t16018, t16186, t16191, t16192, t16195, t16202, t1799, t1819, t19708, t1995, t3719, t3734, t3839, t3844, t5187, t5272, t5278, t5280, t68, t6924);
        let t54527 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2676::<F>(t12012, t12147, t12164, t1347, t1348, t16176, t16186, t16196, t16199, t1819, t1821, t225, t3839, t3847, t5272, t5278, t5279, t5283, t53856, t54311, t54377, t54391, t54415, t54426, t54440, t54454, t54479, t54525, t546, t548, t550);
        let t54552 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2677::<F>(t12364, t5234, t1354, t16288, t3858, t1351, t3734, t12012, t12168, t12413, t1341, t1343, t1363, t16101, t16208, t16224, t16311, t16394, t1799, t221, t3719, t3778, t3803, t3805, t3870, t40160, t5187, t5246, t5248, t5250, t5301, t53958, t54284, t54293, t54295, t54527, t820);
    (t54468, t54470, t54472, t54473, t54475, t54476, t54478, t54527, t54552)
}
