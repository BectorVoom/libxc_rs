//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta696 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2654;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2656;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2657;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2659;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta696<F: Float>(t12283: F, t16271: F, t16383: F, t16370: F, t16060: F, t3798: F, t1354: F, t12345: F, t5310: F, t12339: F, t16150: F, t3866: F, t12240: F, t12379: F, t12392: F, t12409: F, t16242: F, t16261: F, t16394: F, t16401: F, t3803: F, t40000: F, t40168: F, t40169: F, t5235: F, t5246: F, t5248: F, t5249: F, t16155: F, t1827: F, t40123: F, t1824: F, t3850: F, t3802: F, t1799: F, t1340: F, t53909: F, t12255: F, t12305: F, t12336: F, t1307: F, t1363: F, t16018: F, t16217: F, t16224: F, t16225: F, t16305: F, t16306: F, t3783: F, t3807: F, t3809: F, t3851: F, t3870: F, t5240: F, t54013: F, t820: F, t39947: F, t16314: F, t16398: F, t16387: F, t12251: F, t12297: F, t12351: F, t12404: F, t16233: F, t16278: F, t16285: F, t3734: F, t3853: F, t40006: F, t40008: F, t40012: F, t40019: F, t40022: F, t5187: F, t40138: F, t5303: F, t16366: F, t16308: F, t1352: F, t16153: F, t16311: F, t3805: F, t3856: F, t40052: F, t40054: F, t40060: F, t40065: F, t40079: F, t40081: F, t40083: F, t40178: F, t54015: F, t3791: F, t12168: F, t12369: F, t16364: F, t40089: F, t40114: F, t40116: F, t40124: F, t40126: F, t40128: F, t40131: F, t40139: F, t40145: F, t53958: F, t54068: F, t12300: F, t5289: F, t16208: F, t3799: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t39320: F, t53778: F, t53780: F, t53783: F, t53788: F, t53797: F, t53799: F, t53800: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54114, t54116, t54118, t54125, t54132, t54133, t54135) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2654::<F>(t12283, t16271, t16383, t16370, t16060, t3798, t1354, t12345, t5310, t12339, t16150, t3866);
        let t54137 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2655::<F>(t12240, t12379, t12392, t12409, t16242, t16261, t16394, t16401, t3803, t40000, t40168, t40169, t5235, t5246, t5248, t5249, t54114, t54116, t54118, t54125, t54132, t54133, t54135);
        let t54183 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2656::<F>(t16155, t3866, t1827, t40123, t1824, t3850, t16060, t3802, t1799, t1340, t53909, t12255, t12305, t12336, t1307, t1354, t1363, t16018, t16150, t16217, t16224, t16225, t16305, t16306, t3783, t3803, t3807, t3809, t3851, t3870, t5240, t5246, t5248, t5249, t5310, t54013, t820);
        let t54215 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2657::<F>(t16217, t3866, t1827, t39947, t16314, t16398, t16387, t12251, t12297, t12351, t12404, t1363, t16233, t16278, t16285, t16394, t3734, t3853, t40006, t40008, t40012, t40019, t40022, t5187, t5248, t5249, t820);
        let t54245 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658::<F>(t40138, t5303, t12283, t16366, t16308, t1352, t16153, t16224, t16242, t16311, t3803, t3805, t3856, t40052, t40054, t40060, t40065, t40079, t40081, t40083, t40178, t5246, t5248, t5249, t54013, t54015);
        let (t54258, t54277) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2659::<F>(t1824, t3791, t12168, t12369, t1352, t16224, t16305, t16364, t3803, t3805, t3851, t40089, t40114, t40116, t40124, t40126, t40128, t40131, t40139, t40145, t5246, t5248, t5249, t53958, t54068);
        let (t54284, t54293, t54295, t54311) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2660::<F>(t1307, t16153, t12300, t5289, t16208, t3799, t39249, t39256, t39261, t39266, t39304, t39309, t39312, t39316, t39320, t53778, t53780, t53783, t53788, t53797, t53799, t53800);
    (t54137, t54183, t54215, t54245, t54258, t54277, t54284, t54293, t54295, t54311)
}
