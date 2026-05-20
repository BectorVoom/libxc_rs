//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta777 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2655;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2656;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2657;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2659;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2660;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2661;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta777<F: Float>(t12283: F, t20460: F, t20565: F, t3866: F, t1827: F, t57056: F, t20492: F, t39944: F, t12215: F, t1307: F, t16394: F, t1810: F, t19631: F, t19962: F, t19996: F, t20511: F, t210: F, t3733: F, t40025: F, t5187: F, t5226: F, t5240: F, t5259: F, t5293: F, t53882: F, t53901: F, t56878: F, t6347: F, t6370: F, t16288: F, t6417: F, t12385: F, t20497: F, t120: F, t12369: F, t12429: F, t1352: F, t1363: F, t16278: F, t19735: F, t19871: F, t19951: F, t19989: F, t20356: F, t20416: F, t20454: F, t3803: F, t3805: F, t40070: F, t5246: F, t5248: F, t53918: F, t53920: F, t54023: F, t54162: F, t6390: F, t6396: F, t6422: F, t74120: F, t820: F, t20433: F, t16336: F, t6431: F, t1831: F, t57021: F, t53945: F, t20450: F, t16233: F, t19873: F, t19876: F, t20000: F, t40192: F, t5250: F, t5303: F, t53928: F, t56685: F, t56687: F, t57081: F, t57568: F, t74090: F, t20595: F, t68: F, t1340: F, t20556: F, t3799: F, t20570: F, t1362: F, t1354: F, t1369: F, t16321: F, t19868: F, t19904: F, t19930: F, t19991: F, t20479: F, t3783: F, t39936: F, t40035: F, t5235: F, t5314: F, t57024: F, t25: F, t1298: F, t15989: F, t15992: F, t16557: F, t19606: F, t20216: F, t20376: F, t2219: F, t3704: F, t39861: F, t5170: F, t606: F, t67059: F, t73975: F, t73978: F, zeta_threshold: F, t28: F, t1081: F, t1302: F, t16003: F, t16006: F, t18196: F, t19618: F, t20385: F, t20390: F, t3711: F, t39877: F, t5178: F, t71090: F, t73995: F, t73998: F, t20512: F, t40021: F, t1351: F, t1367: F, t16225: F, t16305: F, t16311: F, t19855: F, t20473: F, t5289: F, t5310: F, t53985: F, t53998: F, t56693: F, t56710: F, t56738: F, t56924: F, t57342: F) -> (F, F, F, F, F, F, F) {
        let t74216 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2655::<F>(t12283, t20460, t20565, t3866, t1827, t57056, t20492, t39944, t12215, t1307, t16394, t1810, t19631, t19962, t19996, t20511, t210, t3733, t40025, t5187, t5226, t5240, t5259, t5293, t53882, t53901, t56878, t6347, t6370);
        let t74253 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2656::<F>(t16288, t6417, t12385, t20497, t120, t12369, t12429, t1307, t1352, t1363, t16278, t16394, t19735, t19871, t19951, t19989, t20356, t20416, t20454, t3803, t3805, t40070, t5246, t5248, t53918, t53920, t54023, t54162, t6390, t6396, t6422, t74120, t820);
        let t74286 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2657::<F>(t20433, t3866, t16336, t6431, t1831, t57021, t53945, t6396, t12283, t20450, t16233, t19871, t19873, t19876, t20000, t3805, t40192, t5246, t5248, t5250, t5303, t53928, t56685, t56687, t56878, t57081, t57568, t74090, t74120);
        let (t74289, t74316) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658::<F>(t20595, t68, t1340, t20556, t3799, t20570, t1362, t1354, t1369, t16278, t16321, t16394, t1831, t19868, t19904, t19930, t19991, t20479, t20492, t3783, t39936, t40035, t5235, t5240, t5314, t57024, t6417, t6431);
        let t74335 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2659::<F>(t25, t1298, t15989, t15992, t16557, t19606, t20216, t20376, t2219, t3704, t39861, t5170, t606, t67059, t73975, t73978, zeta_threshold);
        let t74353 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2660::<F>(t28, t1081, t1302, t16003, t16006, t18196, t19618, t20385, t20390, t2219, t3711, t39877, t5178, t71090, t73995, t73998, zeta_threshold);
        let t74355 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2661::<F>(t74335, t74353);
        let t74386 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2662::<F>(t20512, t40021, t1351, t6347, t16288, t6422, t1363, t1367, t16225, t16233, t16305, t16311, t1827, t19855, t19904, t20473, t5246, t5289, t5310, t53985, t53998, t56693, t56710, t56738, t56924, t57342, t74355, t820);
    (t74216, t74253, t74286, t74289, t74316, t74355, t74386)
}
