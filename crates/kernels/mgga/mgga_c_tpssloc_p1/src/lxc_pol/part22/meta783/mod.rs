//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta783 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2679;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2680;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2681;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2682;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2683;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2685;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2686;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2687;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2688;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2689;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta783<F: Float>(t1799: F, t5286: F, t16224: F, t16305: F, t1825: F, t19919: F, t19924: F, t3803: F, t40006: F, t40060: F, t54063: F, t57007: F, t57009: F, t57011: F, t57019: F, t57022: F, t57041: F, t57057: F, t57071: F, t57073: F, t1315: F, t210: F, t214: F, t40343: F, t40347: F, t40350: F, t54631: F, t54633: F, t54638: F, t54639: F, t54644: F, t56465: F, t56469: F, t74355: F, t118: F, t20416: F, t3739: F, t794: F, t16094: F, t16095: F, t6347: F, t686: F, t213: F, t1307: F, t16084: F, t16101: F, t19631: F, t19781: F, t20356: F, t221: F, t40351: F, t5187: F, t5195: F, t5196: F, t54728: F, t56482: F, t56484: F, t56491: F, t56493: F, t20582: F, t40021: F, t40412: F, t20576: F, t3726: F, t40372: F, t40401: F, t40402: F, t40407: F, t46838: F, t56501: F, t56505: F, t56514: F, t74389: F, t16081: F, t20586: F, t40422: F, t54663: F, t54668: F, t54676: F, t54702: F, t54725: F, t56535: F, t56537: F, t56539: F, t56548: F, t56550: F, t225: F, t16311: F, t19876: F, t19890: F, t19966: F, t40124: F, t40145: F, t5246: F, t54534: F, t554: F, t559: F, t57127: F, t57143: F, t57145: F, t57158: F, t57160: F, t57170: F, t6414: F, t1352: F, t16306: F, t20448: F, t20563: F, t54556: F, t54582: F, t54612: F, t57308: F, t57310: F, t57324: F, t57383: F, t57392: F, t57396: F, t57398: F, t57407: F, t57409: F, t16233: F, t16394: F, t19886: F, t19894: F, t19981: F, t40449: F, t54013: F, t54014: F, t54786: F, t54793: F, t54812: F, t56812: F, t57091: F, t57437: F, t57450: F, t57457: F, t6394: F, t74415: F, t74133: F, t74181: F, t74216: F, t74253: F, t74286: F, t74316: F, t74386: F, t74428: F, t74569: F, t74610: F, t74632: F, t74655: F, t20602: F, t20420: F, t1323: F, t1375: F, t1385: F, t1386: F, t16030: F, t16439: F, t1807: F, t1843: F, t20009: F, t20023: F, t20025: F, t20601: F, t20661: F, t20662: F, t26224: F, t3882: F, t3887: F, t5215: F, t539: F, t55118: F, t56596: F, t56607: F, t568: F, t6440: F, t6461: F, t16022: F, t20026: F, t20029: F, t20051: F, t20608: F, t20613: F, t3758: F, t40591: F, t5318: F, t5321: F, t5353: F, t5354: F, t56422: F, t6361: F, t6460: F, t20672: F, t1372: F, t16460: F, t20044: F, t20060: F, t20594: F, t20609: F, t5210: F, t5326: F, t562: F, t56434: F, t56580: F, t6434: F) -> (F, F, F, F, F) {
        let (t74677, t74682) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2679::<F>(t1799, t5286, t16224, t16305, t1825, t19919, t19924, t3803, t40006, t40060, t54063, t57007, t57009, t57011, t57019, t57022, t57041, t57057, t57071, t57073);
        let t74699 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2680::<F>(t1315, t210, t214, t40343, t40347, t40350, t54631, t54633, t54638, t54639, t54644, t56465, t56469, t74355);
        let t74735 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2681::<F>(t118, t20416, t3739, t794, t16094, t16095, t6347, t686, t213, t1307, t16084, t16101, t19631, t19781, t20356, t221, t40351, t5187, t5195, t5196, t54728, t56482, t56484, t56491, t56493);
        let t74754 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2682::<F>(t20582, t40021, t118, t20356, t40412, t794, t20576, t3726, t16101, t40372, t40401, t40402, t40407, t46838, t56501, t56505, t56514, t74389);
        let t74765 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2683::<F>(t16081, t20586, t40422, t54663, t54668, t54676, t54702, t54725, t56535, t56537, t56539, t56548, t56550);
        let (t74767, t74768, t74786) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684::<F>(t74699, t74735, t74754, t74765, t225, t1307, t16305, t16311, t19876, t19890, t19966, t40124, t40145, t5246, t54534, t554, t559, t57127, t57143, t57145, t57158, t57160, t57170, t6414, t74677);
        let t74806 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2685::<F>(t1352, t16224, t16306, t20448, t20563, t3803, t54556, t54582, t54612, t57308, t57310, t57324, t57383, t57392, t57396, t57398, t57407, t57409);
        let t74833 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2686::<F>(t1352, t16233, t16305, t16394, t19886, t19894, t19981, t3803, t40449, t54013, t54014, t54786, t54793, t54812, t56812, t57091, t57437, t57450, t57457, t6394, t74415);
        let t74837 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2687::<F>(t74133, t74181, t74216, t74253, t74286, t74316, t74386, t74428, t74569, t74610, t74632, t74655, t74682, t74786, t74806, t74833);
        let t74868 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2688::<F>(t20602, t225, t20420, t1323, t1375, t1385, t1386, t16030, t16439, t1807, t1843, t20009, t20023, t20025, t20601, t20661, t20662, t26224, t3882, t3887, t5215, t539, t55118, t56596, t56607, t568, t6440, t6461, t74837);
        let t74899 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2689::<F>(t1375, t1385, t16022, t16030, t16439, t1843, t20023, t20026, t20029, t20051, t20608, t20613, t20662, t3758, t3887, t40591, t5215, t5318, t5321, t5353, t5354, t56422, t568, t6361, t6440, t6460, t6461);
        let t74929 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2690::<F>(t20672, t225, t1372, t1386, t16022, t16460, t1843, t20044, t20060, t20594, t20609, t20613, t3758, t3882, t5210, t5326, t562, t56434, t56580, t568, t6434, t6440, t6461, t74767);
    (t74768, t74837, t74868, t74899, t74929)
}
