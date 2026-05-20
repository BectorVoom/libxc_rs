//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta695 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2647;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2648;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2649;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2650;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2651;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2652;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta695<F: Float>(t16060: F, t3865: F, t1369: F, t16123: F, t68: F, t1362: F, t1831: F, t40292: F, t12345: F, t5314: F, t12211: F, t16296: F, t40018: F, t5223: F, t16379: F, t40021: F, t12156: F, t12240: F, t12353: F, t12407: F, t1307: F, t16225: F, t16305: F, t16306: F, t16321: F, t16355: F, t1810: F, t210: F, t3733: F, t3803: F, t3876: F, t39936: F, t40025: F, t5240: F, t5246: F, t12282: F, t5234: F, t3809: F, t120: F, t16205: F, t12283: F, t16227: F, t1351: F, t5286: F, t12429: F, t1352: F, t16148: F, t16224: F, t16308: F, t16311: F, t16314: F, t16401: F, t3805: F, t3807: F, t39945: F, t39948: F, t39950: F, t39956: F, t39958: F, t39960: F, t40197: F, t12189: F, t5227: F, t16232: F, t3777: F, t40281: F, t5303: F, t16300: F, t5247: F, t820: F, t12250: F, t1824: F, t3850: F, t12290: F, t3789: F, t12012: F, t12215: F, t12293: F, t12303: F, t12420: F, t16048: F, t16233: F, t16235: F, t16242: F, t1825: F, t3719: F, t3734: F, t3795: F, t39971: F, t5226: F, t5248: F, t16288: F, t3853: F, t12384: F, t5293: F, t12397: F, t1363: F, t16257: F, t16271: F, t16275: F, t16278: F, t1799: F, t1827: F, t3858: F, t39973: F, t39975: F, t39983: F, t39989: F, t40070: F, t40119: F, t5289: F, t16405: F, t40167: F, t3791: F, t40138: F, t5259: F, t16248: F, t12178: F, t16018: F, t16364: F, t16370: F, t16387: F, t16391: F, t3793: F, t5249: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53907, t53909, t53910, t53918, t53920, t53921) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2647::<F>(t16060, t3865, t1369, t16123, t68, t1362, t1831, t40292, t12345, t5314, t12211, t16296);
        let t53943 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2648::<F>(t40018, t5223, t16379, t40021, t12156, t12240, t12353, t12407, t1307, t1369, t16225, t16305, t16306, t16321, t16355, t1810, t210, t3733, t3803, t3876, t39936, t40025, t5240, t5246, t53907, t53910, t53918, t53920, t53921);
        let (t53958, t53978) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2649::<F>(t12282, t5234, t3809, t120, t16205, t12283, t16227, t1351, t5286, t12429, t1352, t16148, t16224, t16305, t16308, t16311, t16314, t16401, t3803, t3805, t3807, t39945, t39948, t39950, t39956, t39958, t39960, t40197, t5246);
        let (t53985, t53990, t53998, t54003, t54013, t54014) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2650::<F>(t12189, t5227, t16232, t3777, t40281, t5303, t12211, t16300, t5247, t820, t12250, t1824);
        let (t54015, t54026) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2651::<F>(t1351, t3850, t12290, t5234, t16060, t3789, t12012, t12215, t12293, t12303, t12420, t16048, t16224, t16233, t16235, t16242, t16306, t1810, t1825, t210, t3719, t3733, t3734, t3795, t3803, t39971, t5226, t5248, t53985, t53990, t53998, t54003, t54013, t54014);
        let t54058 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2652::<F>(t16288, t3853, t12384, t5234, t3795, t40281, t5293, t12156, t12397, t12429, t1363, t16257, t16271, t16275, t16278, t16401, t1799, t1827, t3858, t39973, t39975, t39983, t39989, t40070, t40119, t5289, t820);
        let (t54068, t54100) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2653::<F>(t12283, t16405, t40167, t820, t1799, t3791, t40138, t5259, t16248, t5293, t16275, t120, t12178, t12420, t12429, t1352, t16018, t16224, t16225, t16227, t16364, t16370, t16387, t16391, t16401, t3793, t3803, t3805, t3807, t5246, t5248, t5249);
    (t53909, t53943, t53958, t53978, t54013, t54014, t54015, t54026, t54058, t54068, t54100)
}
