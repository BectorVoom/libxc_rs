//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta697 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2661;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2662;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta697<F: Float>(t1788: F, t9212: F, t9214: F, t2223: F, t5168: F, t39328: F, t39330: F, t39334: F, t39339: F, t39341: F, t15977: F, t588: F, t25: F, t5157: F, t9874: F, t5137: F, t591: F, t11988: F, t12061: F, t1408: F, t15937: F, t15940: F, t16: F, t2: F, t3664: F, t39419: F, t5134: F, t514: F, t53805: F, t53808: F, t53814: F, t53817: F, t584: F, t606: F, t9257: F, zeta_threshold: F, t28: F, t5145: F, t1081: F, t11122: F, t12001: F, t12072: F, t15952: F, t15955: F, t1649: F, t3672: F, t39436: F, t5142: F, t517: F, t53832: F, t53835: F, t53841: F, t53844: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54323) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2661::<F>(t1788, t9212, t9214, t2223, t5168, t39328, t39330, t39334, t39339, t39341, t15977, t588);
        let (t54324, t54326, t54349) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2662::<F>(t25, t54323, t5157, t9874, t5137, t591, t11988, t12061, t1408, t15937, t15940, t16, t2, t3664, t39419, t5134, t514, t53805, t53808, t53814, t53817, t584, t606, t9257, zeta_threshold);
        let t54372 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2663::<F>(t28, t5145, t591, t1081, t11122, t12001, t12072, t15952, t15955, t16, t1649, t2, t3672, t39436, t5142, t517, t53832, t53835, t53841, t53844, t584, zeta_threshold);
    (t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54324, t54326, t54349, t54372)
}
