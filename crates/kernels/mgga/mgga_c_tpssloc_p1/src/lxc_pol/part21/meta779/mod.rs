//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2702;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2703;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2704;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta779<F: Float>(t5303: F, t53945: F, t16336: F, t5310: F, t5286: F, t3792: F, t1827: F, t54124: F, t16288: F, t5289: F, t19805: F, t68: F, t1340: F, t12365: F, t6417: F, t12283: F, t19962: F, t19882: F, t19996: F, t3866: F, t40018: F, t6371: F, t119: F, t12351: F, t12419: F, t12420: F, t1343: F, t1354: F, t1363: F, t16321: F, t19871: F, t210: F, t3733: F, t3734: F, t3790: F, t3803: F, t54151: F, t54191: F, t54198: F, t56486: F, t6347: F, t820: F, t12189: F, t6375: F, t40138: F, t6396: F, t19951: F, t19991: F, t40281: F, t12407: F, t12429: F, t16224: F, t16225: F, t16305: F, t16306: F, t16311: F, t16366: F, t16370: F, t16394: F, t19921: F, t19926: F, t19976: F, t19981: F, t19989: F, t3783: F, t3805: F, t5246: F, t53973: F, t54013: F, t54162: F, t54202: F, t12339: F, t6427: F, t6431: F, t12345: F, t19815: F, t3865: F, t1369: F, t1362: F, t19904: F, t3870: F, t3872: F, t3876: F, t40006: F, t40008: F, t40019: F, t40060: F, t54213: F, t54220: F, t54222: F, t54237: F) -> (F, F, F, F, F, F) {
        let (t56906, t56909, t56913, t56914, t56919, t56921, t56923) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2702::<F>(t5303, t53945, t16336, t5310, t5286, t3792, t1827, t54124, t16288, t5289, t19805, t68);
        let t56952 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2703::<F>(t1340, t56923, t12365, t6417, t12283, t19962, t19882, t19996, t3866, t40018, t6371, t119, t12351, t12419, t12420, t1343, t1354, t1363, t16321, t19871, t210, t3733, t3734, t3790, t3803, t5310, t54151, t54191, t54198, t56486, t56906, t56909, t56914, t56919, t56921, t6347, t820);
        let t56996 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2704::<F>(t12189, t6375, t40138, t6396, t12283, t19951, t19991, t40281, t12407, t12429, t16224, t16225, t16305, t16306, t16311, t16366, t16370, t16394, t19871, t19921, t19926, t19976, t19981, t19989, t3783, t3803, t3805, t5246, t5303, t53973, t54013, t54162, t54202);
        let t57030 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2705::<F>(t12339, t6427, t6431, t12345, t19815, t3865, t1369, t1362, t56923, t1363, t19904, t3870, t3872, t3876, t40006, t40008, t40019, t40060, t54213, t54220, t54222, t54237, t56486, t820);
    (t56913, t56914, t56923, t56952, t56996, t57030)
}
