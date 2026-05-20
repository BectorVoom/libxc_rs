//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta784 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2691;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2692;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2693;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2694;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2695;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2696;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2697;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2698;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2699;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2700;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2701;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta784<F: Float>(t20670: F, t225: F, t1834: F, t6414: F, t6387: F, t20553: F, t562: F, t20489: F, t16036: F, t16047: F, t16055: F, t1825: F, t19654: F, t19661: F, t19735: F, t19743: F, t19744: F, t19810: F, t20018: F, t20473: F, t20638: F, t5250: F, t5287: F, t5333: F, t5334: F, t5336: F, t5344: F, t54963: F, t57704: F, t6378: F, t74599: F, t12250: F, t1824: F, t6434: F, t1336: F, t1352: F, t19657: F, t19748: F, t19815: F, t20490: F, t20568: F, t20622: F, t3777: F, t3901: F, t40492: F, t5335: F, t5349: F, t57618: F, t74174: F, t12171: F, t16060: F, t16132: F, t1840: F, t19658: F, t19660: F, t19752: F, t19805: F, t20495: F, t20648: F, t26322: F, t5234: F, t5339: F, t5341: F, t6420: F, t6454: F, t1338: F, t20601: F, t16040: F, t19668: F, t19732: F, t20625: F, t20643: F, t5348: F, t5351: F, t57659: F, t6415: F, t6448: F, t6451: F, t12021: F, t1332: F, t1351: F, t1375: F, t1378: F, t1380: F, t1381: F, t1383: F, t1386: F, t16033: F, t1814: F, t1838: F, t1842: F, t1843: F, t19648: F, t19674: F, t19733: F, t19736: F, t19739: F, t19740: F, t19745: F, t19756: F, t19761: F, t19763: F, t19804: F, t19813: F, t20010: F, t20014: F, t20022: F, t20026: F, t20029: F, t20044: F, t20051: F, t20060: F, t20554: F, t20595: F, t20609: F, t20616: F, t20630: F, t20632: F, t20635: F, t20645: F, t20651: F, t26318: F, t3882: F, t3887: F, t5215: F, t5230: F, t5321: F, t5326: F, t5343: F, t5345: F, t5353: F, t5354: F, t544: F, t54905: F, t553: F, t564: F, t56640: F, t568: F, t56923: F, t57530: F, t57568: F, t6388: F, t6439: F, t6456: F, t6458: F, t74289: F, t74564: F, t74768: F, t74837: F, t1390: F, t16497: F, t193: F, t19577: F, t19631: F, t20063: F, t20067: F, t20077: F, t20085: F, t3918: F, t39483: F, t5122: F, t5126: F, t5160: F, t5161: F, t5308: F, t533: F, t54409: F, t6330: F, t74086: F, t74470: F, t74868: F, t74899: F, t74929: F, t1388: F, t6463: F, t1307: F, t15899: F, t20563: F, t3919: F, t39529: F, t39539: F, t39549: F, t74476: F, t74477: F, t74478: F, t74479: F, t19924: F, t19994: F, t39585: F, t39590: F, t39593: F, t39595: F, t54431: F, t54436: F, t74484: F, t74485: F, t74486: F, t12461: F, t20684: F, t39655: F, t39658: F, t39844: F, t5356: F, t54453: F, t74490: F, t74491: F, t74493: F, t74494: F, t74496: F, t74497: F, t571: F, t1297: F, t40224: F, t40230: F, t54470: F, t54472: F, t54473: F, t54475: F, t54478: F, t74355: F, t74502: F, t74503: F, t74504: F, t4025: F, t5456: F, t20193: F, t604: F, t1411: F, t1434: F, t19322: F, t19363: F, t19441: F, t20207: F, t20264: F, t20285: F, t3962: F, t3966: F, t3968: F, t3971: F, t3976: F, t5398: F, t5442: F, t55653: F, t608: F, t609: F, t65: F, t6509: F, t67: F, t80: F, t1409: F, t1426: F, t16558: F, t17635: F, t1864: F, t19323: F, t19331: F, t19334: F, t20218: F, t20219: F, t20222: F, t31: F, t3997: F, t5399: F, t628: F, t642: F, t67060: F, t70458: F, t7445: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t74930, t74937, t74941, t74949, t74967, t74996) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2691::<F>(t20670, t225, t1834, t6414, t6387, t20553, t562, t20489, t16036, t16047, t16055, t1825, t19654, t19661, t19735, t19743, t19744, t19810, t20018, t20473, t20638, t5250, t5287, t5333, t5334, t5336, t5344, t54963, t57704, t6378, t74599);
        let (t75008, t75053) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2692::<F>(t12250, t6414, t1824, t6434, t1336, t1352, t16047, t1825, t19654, t19657, t19744, t19748, t19815, t20490, t20568, t20622, t3777, t3901, t40492, t5250, t5287, t5334, t5335, t5344, t5349, t57618, t74174, t74941);
        let t75101 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2693::<F>(t12171, t1336, t1352, t16060, t16132, t1840, t19658, t19660, t19752, t19805, t19815, t20495, t20648, t26322, t3777, t5234, t5339, t5341, t5344, t6420, t6454, t74967);
        let t75150 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2694::<F>(t1338, t20601, t1336, t1352, t16040, t16060, t16132, t1825, t19668, t19732, t20473, t20625, t20643, t3777, t5234, t5334, t5348, t5351, t57659, t6378, t6415, t6448, t6451);
        let t75183 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2695::<F>(t12021, t1332, t1336, t1351, t1352, t1375, t1378, t1380, t1381, t1383, t1386, t16033, t16047, t16060, t1814, t1834, t1838, t1842, t1843, t19648, t19654, t19660, t19674, t19733, t19735, t19736, t19739, t19740, t19743, t19745, t19756, t19761, t19763, t19804, t19810, t19813, t20010, t20014, t20022, t20026, t20029, t20044, t20051, t20060, t20554, t20595, t20609, t20616, t20630, t20632, t20635, t20645, t20651, t26318, t3777, t3882, t3887, t3901, t5215, t5230, t5234, t5250, t5287, t5321, t5326, t5334, t5335, t5343, t5344, t5345, t5353, t5354, t544, t54905, t553, t564, t56640, t568, t56923, t57530, t57568, t6378, t6388, t6439, t6456, t6458, t74289, t74564, t74768, t74837, t74930, t74937, t74941, t74949, t74996, t75008, t75053, t75101, t75150);
        let t75198 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2696::<F>(t1390, t16497, t193, t19577, t19631, t20063, t20067, t20077, t20085, t3918, t39483, t5122, t5126, t5160, t5161, t5308, t533, t54409, t6330, t74086, t74470, t74868, t74899, t74929, t75183);
        let t75218 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2697::<F>(t1388, t6330, t6463, t1307, t15899, t20563, t3918, t3919, t39529, t39539, t39549, t5126, t5160, t5161, t74476, t74477, t74478, t74479);
        let t75237 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2698::<F>(t19924, t19994, t39585, t39590, t39593, t39595, t5122, t5126, t54431, t54436, t74484, t74485, t74486);
        let (t75240, t75254) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2699::<F>(t12461, t20684, t20085, t39655, t39658, t39844, t5160, t5356, t54453, t74490, t74491, t74493, t74494, t74496, t74497);
        let (t75256, t75267) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2700::<F>(t571, t6330, t1297, t193, t40224, t40230, t54470, t54472, t54473, t54475, t54478, t74355, t74502, t74503, t74504);
        let (t75275, t75284, t75356) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2701::<F>(t4025, t5456, t20193, t604, t1411, t1434, t19322, t19363, t19441, t20207, t20264, t20285, t3962, t3966, t3968, t3971, t3976, t5398, t5442, t55653, t608, t609, t65, t6509, t67, t80);
        let t75392 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2702::<F>(t1409, t1426, t67, t1434, t16558, t17635, t1864, t19322, t19323, t19331, t19334, t20218, t20219, t20222, t31, t3966, t3997, t5399, t628, t642, t65, t67060, t70458, t7445, t80);
    (t75198, t75218, t75237, t75240, t75254, t75256, t75267, t75275, t75284, t75356, t75392)
}
