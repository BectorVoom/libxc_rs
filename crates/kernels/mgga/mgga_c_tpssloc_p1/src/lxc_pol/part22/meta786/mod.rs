//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta786 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2715;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2716;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2717;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2718;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2719;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta786<F: Float>(t113: F, t1307: F, t1388: F, t1390: F, t1442: F, t1458: F, t15868: F, t16497: F, t1799: F, t1849: F, t19289: F, t193: F, t19456: F, t19537: F, t20067: F, t20127: F, t20143: F, t20356: F, t20416: F, t20689: F, t20702: F, t2314: F, t28002: F, t3918: F, t39309: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39472: F, t39476: F, t39490: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39563: F, t39570: F, t39615: F, t39620: F, t4028: F, t4034: F, t4073: F, t510: F, t5126: F, t513: F, t5160: F, t5161: F, t5187: F, t5361: F, t54315: F, t54317: F, t54401: F, t54403: F, t54429: F, t54430: F, t54437: F, t54438: F, t54439: F, t54447: F, t54457: F, t54459: F, t54461: F, t54465: F, t54466: F, t5494: F, t55224: F, t56358: F, t571: F, t574: F, t6295: F, t6347: F, t6463: F, t652: F, t71077: F, t73953: F, t74020: F, t74024: F, t74037: F, t74040: F, t74041: F, t74042: F, t74043: F, t74044: F, t74058: F, t74060: F, t74064: F, t74068: F, t74073: F, t74075: F, t74078: F, t74474: F, t74475: F, t74481: F, t74482: F, t74483: F, t74489: F, t74499: F, t74500: F, t74501: F, t75198: F, t75218: F, t75237: F, t75240: F, t75254: F, t75256: F, t75267: F, t75275: F, t75704: F, t1266: F, t1393: F, t1774: F, t19450: F, t19451: F, t19461: F, t19534: F, t20293: F, t20347: F, t20350: F, t20720: F, t5107: F, t5118: F, t5450: F, t5457: F, t6468: F, t75555: F, t1271: F, t1459: F, t1778: F, t20098: F, t20136: F, t20296: F, t20698: F, t22425: F, t26114: F, t26179: F, t4026: F, t4037: F, t55943: F, t6287: F, t650: F, t671: F, t7458: F, t75560: F, t75701: F, t67030: F, t1858: F, t6470: F, t1851: F, t6483: F, t22453: F, t576: F, t112: F, t22430: F, t12524: F, t1395: F, t1401: F, t16521: F, t16524: F, t20162: F, t20173: F, t20176: F, t20181: F, t22445: F, t22448: F, t28893: F, t3938: F, t3941: F, t4072: F, t5371: F, t5376: F, t5456: F, t5493: F, t55353: F, t55388: F, t577: F, t66958: F, t1396: F, t1398: F, t1404: F, t1852: F, t20149: F, t20186: F, t22431: F, t3: F, t5364: F, t5381: F, t580: F, t6471: F, t66964: F, t66967: F, t66976: F, t66987: F, t66989: F, t66991: F, t67000: F) -> F {
        let t75706 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2715::<F>(t113, t1307, t1388, t1390, t1442, t1458, t15868, t16497, t1799, t1849, t19289, t193, t19456, t19537, t20067, t20127, t20143, t20356, t20416, t20689, t20702, t2314, t28002, t3918, t39309, t39312, t39316, t39320, t39324, t39327, t39364, t39373, t39384, t39393, t39397, t39472, t39476, t39490, t39496, t39499, t39502, t39505, t39508, t39518, t39521, t39563, t39570, t39615, t39620, t4028, t4034, t4073, t510, t5126, t513, t5160, t5161, t5187, t5361, t54315, t54317, t54401, t54403, t54429, t54430, t54437, t54438, t54439, t54447, t54457, t54459, t54461, t54465, t54466, t5494, t55224, t56358, t571, t574, t6295, t6347, t6463, t652, t71077, t73953, t74020, t74024, t74037, t74040, t74041, t74042, t74043, t74044, t74058, t74060, t74064, t74068, t74073, t74075, t74078, t74474, t74475, t74481, t74482, t74483, t74489, t74499, t74500, t74501, t75198, t75218, t75237, t75240, t75254, t75256, t75267, t75275, t75704);
        let t75733 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2716::<F>(t1266, t1393, t1774, t19450, t19451, t19461, t19534, t20293, t20347, t20350, t20720, t2314, t4034, t4073, t510, t5107, t5118, t5450, t5457, t6468, t652, t75555);
        let t75762 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2717::<F>(t1266, t1271, t1459, t1778, t19451, t20098, t20136, t20143, t20296, t20698, t22425, t26114, t26179, t4026, t4028, t4037, t510, t5494, t55943, t6287, t650, t652, t671, t7458, t75560, t75701);
        let (t75764, t75768, t75774, t75780, t75784) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2718::<F>(t67030, t75706, t75733, t75762, t1858, t6470, t1851, t6483, t22453, t576, t112, t22430);
        let t75827 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2719::<F>(t1851, t671, t12524, t1395, t1401, t1458, t16521, t16524, t19534, t20162, t20173, t20176, t20181, t20347, t22445, t22448, t28893, t3938, t3941, t4072, t5371, t5376, t5456, t5493, t55353, t55388, t577, t66958, t75701, t75764, t75784);
        let tv4rho43 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2720::<F>(t1396, t1398, t1404, t1852, t1858, t20149, t20186, t22431, t22453, t3, t5364, t5381, t580, t6471, t6483, t66964, t66967, t66976, t66987, t66989, t66991, t67000, t75764, t75768, t75774, t75780, t75827);
    tv4rho43
}
