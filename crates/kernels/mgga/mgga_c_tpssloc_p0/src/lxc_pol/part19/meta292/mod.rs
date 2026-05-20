//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1065;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1066;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1067;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta292<F: Float>(t12407: F, t3805: F, t3806: F, t12402: F, t1352: F, t5248: F, t1995: F, t67: F, t246: F, t3734: F, t550: F, t12368: F, t3807: F, t3777: F, t3802: F, t12392: F, t12395: F, t12397: F, t12404: F, t1341: F, t1354: F, t3778: F, t3783: F, t3803: F, t3809: F, t3853: F, t3872: F, t12279: F, t12284: F, t12286: F, t12291: F, t12293: F, t12297: F, t12301: F, t12305: F, t12308: F, t12310: F, t12313: F, t12348: F, t12390: F, t1315: F, t1363: F, t3790: F, t3795: F, t5246: F, t553: F, t12169: F, t12172: F, t12179: F, t12181: F, t12238: F, t12241: F, t12244: F, t12252: F, t12256: F, t12260: F, t12267: F, t12273: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3898: F, t3902: F, t3905: F, t3907: F, t3909: F, t5334: F, t5344: F, t544: F, t564: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12409, t12413, t12418, t12419, t12420, t12422, t12426) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1065::<F>(t12407, t3805, t3806, t12402, t1352, t5248, t1995, t67, t246, t3734, t550, t12368, t3807);
        let (t12429, t12432) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1066::<F>(t3777, t3802, t12392, t12395, t12397, t12404, t12409, t12413, t12422, t12426, t1341, t1354, t3778, t3783, t3803, t3809, t3853, t3872);
        let t12434 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1067::<F>(t12279, t12284, t12286, t12291, t12293, t12297, t12301, t12305, t12308, t12310, t12313, t12348, t12390, t12432, t1315, t1363, t3790, t3795, t5246);
        let (t12435, t12437) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1068::<F>(t12434, t553, t12169, t12172, t12179, t12181, t12238, t12241, t12244, t12252, t12256, t12260, t12267, t12273, t1332, t1336, t1381, t1383, t3773, t3777, t3898, t3902, t3905, t3907, t3909, t5334, t5344, t544, t564);
    (t12409, t12413, t12418, t12419, t12420, t12422, t12426, t12429, t12434, t12435, t12437)
}
