//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta786 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2727;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2728;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2729;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2730;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2731;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2732;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2733;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2734;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2735;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2736;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta786<F: Float>(t16398: F, t19890: F, t12283: F, t19972: F, t225: F, t56570: F, t16150: F, t16155: F, t16233: F, t16308: F, t16387: F, t16394: F, t19855: F, t19871: F, t19876: F, t19956: F, t3803: F, t3851: F, t3858: F, t40335: F, t40443: F, t40449: F, t5240: F, t5248: F, t54764: F, t54785: F, t54787: F, t54793: F, t54801: F, t54811: F, t554: F, t559: F, t56689: F, t56729: F, t56778: F, t56826: F, t56866: F, t56904: F, t56952: F, t56996: F, t57030: F, t57084: F, t57133: F, t57305: F, t57351: F, t57400: F, t57447: F, t1834: F, t5286: F, t12240: F, t1352: F, t16036: F, t16037: F, t16041: F, t16047: F, t16048: F, t16055: F, t16419: F, t19654: F, t19661: F, t19735: F, t19736: F, t19739: F, t19743: F, t19810: F, t3793: F, t5334: F, t5344: F, t16046: F, t1814: F, t1824: F, t5318: F, t1351: F, t16033: F, t16049: F, t16052: F, t16125: F, t19660: F, t19740: F, t19763: F, t5230: F, t5250: F, t5335: F, t5343: F, t5345: F, t54963: F, t56666: F, t57147: F, t12250: F, t12267: F, t1336: F, t16044: F, t16206: F, t19668: F, t19732: F, t19745: F, t19748: F, t19752: F, t20018: F, t3777: F, t3901: F, t3909: F, t54976: F, t6378: F, t6448: F, t1372: F, t6387: F, t6414: F, t12259: F, t1380: F, t16060: F, t16065: F, t16068: F, t16416: F, t1825: F, t19674: F, t19761: F, t5234: F, t5333: F, t5336: F, t5339: F, t5341: F, t55039: F, t57354: F, t6420: F, t3787: F, t6434: F, t1338: F, t20009: F, t1381: F, t16133: F, t16414: F, t1838: F, t19657: F, t19815: F, t3898: F, t3902: F, t5348: F, t5351: F, t53909: F, t544: F, t553: F, t56923: F, t1332: F, t16127: F, t16132: F, t16423: F, t19658: F, t19813: F, t20010: F, t3773: F, t3856: F, t3905: F, t3907: F, t40486: F, t5287: F, t6388: F, t6415: F, t6456: F, t6458: F, t19731: F, t562: F, t1383: F, t16136: F, t16429: F, t19805: F, t20014: F, t3897: F, t5349: F, t564: F, t56914: F, t6454: F, t16123: F, t16433: F, t1840: F, t19733: F, t19744: F, t19756: F, t57300: F, t6451: F, t12021: F, t12030: F, t1375: F, t1378: F, t16030: F, t16413: F, t16437: F, t16439: F, t16453: F, t16471: F, t1807: F, t1843: F, t19648: F, t20051: F, t20060: F, t3758: F, t3882: F, t3888: F, t3889: F, t3911: F, t5215: F, t5321: F, t5326: F, t5354: F, t539: F, t55134: F, t568: F, t6439: F, t6440: F, t6460: F, t1390: F, t16497: F, t193: F, t3918: F, t39595: F, t39615: F, t5187: F, t533: F, t56411: F, t56412: F, t56416: F, t56417: F, t56457: F, t56605: F, t56649: F, t57203: F, t57204: F, t57205: F) -> F {
        let (t57465, t57481) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726::<F>(t16398, t19890, t12283, t19972, t225, t56570, t16150, t16155, t16233, t16308, t16387, t16394, t19855, t19871, t19876, t19956, t3803, t3851, t3858, t40335, t40443, t40449, t5240, t5248, t54764, t54785, t54787, t54793, t54801, t54811, t554, t559);
        let t57485 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2727::<F>(t56689, t56729, t56778, t56826, t56866, t56904, t56952, t56996, t57030, t57084, t57133, t57305, t57351, t57400, t57447, t57481);
        let (t57499, t57526) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2728::<F>(t1834, t5286, t12240, t1352, t16036, t16037, t16041, t16047, t16048, t16055, t16419, t19654, t19661, t19735, t19736, t19739, t19743, t19810, t3793, t3851, t40335, t5334, t5344);
        let (t57545, t57564) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2729::<F>(t16046, t1814, t1824, t5318, t1351, t19735, t12240, t16033, t16047, t16048, t16049, t16052, t16055, t16125, t19654, t19660, t19740, t19743, t19763, t19810, t5230, t5250, t5334, t5335, t5343, t5345, t54963, t56666, t57147, t57499);
        let t57597 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2730::<F>(t12250, t5286, t12240, t12267, t1336, t1351, t16033, t16044, t16047, t16055, t16206, t19660, t19668, t19732, t19739, t19745, t19748, t19752, t19810, t20018, t3777, t3851, t3901, t3909, t5334, t5335, t5344, t54976, t6378, t6448);
        let (t57607, t57618, t57631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2731::<F>(t1372, t6387, t6414, t12259, t1336, t1352, t1380, t16033, t16060, t16065, t16068, t16416, t1825, t19654, t19674, t19761, t19810, t3777, t5230, t5234, t5250, t5333, t5334, t5336, t5339, t5341, t5344, t55039, t57354, t6420);
        let t57667 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2732::<F>(t1352, t5286, t3787, t6434, t1338, t20009, t1336, t1381, t16133, t16206, t16414, t1814, t1838, t19657, t19815, t3793, t3851, t3898, t3902, t5230, t5234, t5335, t5344, t5348, t5351, t53909, t544, t553, t56923, t57485);
        let t57692 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2733::<F>(t12259, t12267, t1332, t1336, t16127, t16132, t16423, t19657, t19658, t19813, t19815, t20010, t3773, t3777, t3856, t3905, t3907, t40486, t5234, t5287, t6388, t6415, t6456, t6458);
        let (t57704, t57725) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2734::<F>(t19731, t562, t12267, t1336, t1352, t1383, t16033, t16036, t16060, t16136, t16429, t19739, t19805, t20014, t3856, t3897, t5234, t5250, t5287, t5334, t5344, t5349, t564, t56914, t57465, t57545, t57618, t6454);
        let t57760 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2735::<F>(t12267, t1336, t1352, t1380, t16047, t16048, t16123, t16433, t1840, t19660, t19733, t19743, t19744, t19756, t3777, t3793, t3856, t5234, t5250, t5334, t5344, t57300, t57607, t57704, t6451);
        let t57795 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2736::<F>(t12021, t12030, t1375, t1378, t16030, t16413, t16437, t16439, t16453, t16471, t1807, t1843, t19648, t20051, t20060, t3758, t3882, t3888, t3889, t3911, t5215, t5321, t5326, t5354, t539, t55134, t568, t57485, t57526, t57564, t57597, t57631, t57667, t57692, t57725, t57760, t6439, t6440, t6460);
        let t57801 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2737::<F>(t1390, t16497, t193, t3918, t39595, t39615, t5187, t533, t56411, t56412, t56416, t56417, t56457, t56605, t56649, t57203, t57204, t57205, t57795);
    t57801
}
