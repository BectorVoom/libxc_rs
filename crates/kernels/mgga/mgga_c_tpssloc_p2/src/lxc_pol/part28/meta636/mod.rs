//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta636 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2020;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2021;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2022;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2023;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2024;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2025;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2026;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2027;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2028;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2029;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2030;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta636<F: Float>(t91214: F, t80761: F, t80767: F, t80769: F, t80776: F, t91183: F, t91185: F, t91187: F, t91189: F, t91192: F, t91196: F, t91200: F, t91204: F, t91206: F, t91210: F, t91212: F, t91216: F, t91218: F, t91225: F, t80780: F, t80784: F, t80789: F, t80792: F, t80794: F, t80796: F, t80801: F, t80807: F, t80814: F, t80821: F, t80828: F, t84514: F, t91229: F, t91233: F, t91237: F, t91241: F, t91256: F, t91281: F, t91283: F, t91286: F, t91290: F, t91300: F, t80837: F, t80843: F, t80857: F, t80859: F, t84520: F, t91261: F, t91263: F, t91268: F, t91272: F, t91276: F, t91279: F, t91294: F, t91298: F, t91303: F, t91305: F, t91310: F, t91312: F, t91327: F, t91344: F, t80867: F, t80870: F, t80872: F, t91317: F, t91319: F, t91321: F, t91323: F, t91330: F, t91333: F, t91336: F, t91340: F, t91346: F, t91356: F, t91358: F, t91364: F, t91386: F, t80889: F, t80915: F, t84533: F, t84536: F, t91354: F, t91362: F, t91366: F, t91370: F, t91374: F, t91378: F, t91381: F, t91384: F, t91389: F, t91391: F, t91394: F, t91398: F, t91402: F, t91404: F, t80920: F, t80922: F, t80940: F, t80943: F, t80959: F, t80989: F, t80992: F, t80998: F, t81007: F, t84555: F, t84558: F, t91400: F, t91413: F, t91416: F, t93642: F, t93661: F, t16060: F, t27086: F, t3777: F, t544: F, t553: F, t7209: F, t81127: F, t81140: F, t81149: F, t81160: F, t81184: F, t84595: F, t84597: F, t91002: F, t91008: F, t91014: F, t91025: F, t91036: F, t93615: F, t93618: F, t91064: F, t91076: F, t91078: F, t91081: F, t3787: F, t7918: F, t1336: F, t1814: F, t24116: F, t24121: F, t3793: F, t5230: F, t5287: F, t7211: F, t81187: F, t81189: F, t81197: F, t81216: F, t81218: F, t81230: F, t91048: F, t91052: F, t91074: F, t91091: F, t12020: F, t7213: F, t90723: F, t12444: F, t1375: F, t1385: F, t16453: F, t1807: F, t2092: F, t24063: F, t26990: F, t27114: F, t3887: F, t55093: F, t568: F, t7194: F, t7937: F, t81307: F, t81311: F, t90665: F, t90728: F, t90737: F, t90741: F, t91486: F, t12030: F, t16452: F, t1843: F, t24139: F, t26224: F, t26989: F, t27068: F, t3889: F, t5215: F, t55150: F, t81365: F, t81375: F, t84700: F, t91478: F, t91482: F, t91531: F, t91548: F, t12033: F, t16022: F, t27115: F, t3752: F, t3882: F, t7199: F, t7214: F, t81393: F, t81395: F, t84705: F, t91505: F, t12545: F, t1266: F, t12724: F, t12728: F, t12823: F, t1323: F, t1378: F, t1386: F, t1390: F, t16030: F, t16437: F, t16439: F, t16460: F, t16470: F, t16471: F, t1842: F, t19577: F, t1983: F, t2075: F, t22574: F, t22607: F, t2320: F, t23951: F, t24092: F, t24138: F, t24147: F, t26161: F, t26558: F, t26875: F, t26967: F, t26969: F, t27009: F, t27051: F, t27132: F, t27145: F, t27180: F, t32193: F, t3652: F, t3758: F, t3912: F, t4034: F, t510: F, t5210: F, t5321: F, t5325: F, t533: F, t5353: F, t53866: F, t539: F, t54825: F, t55069: F, t56404: F, t6876: F, t6879: F, t7042: F, t7191: F, t7685: F, t7787: F, t7806: F, t7890: F, t7904: F, t7925: F, t7943: F, t80683: F, t80722: F, t80725: F, t80728: F, t81284: F, t81328: F, t81350: F, t81379: F, t84433: F, t84659: F, t90442: F, t90509: F, t90519: F, t90556: F, t90560: F, t90568: F, t90571: F, t90602: F, t90612: F, t90615: F, t91449: F, t91455: F, t91469: F, t91870: F, t93286: F, t93332: F, t93333: F, t93335: F, t93337: F, t93338: F, t93341: F, t93344: F, t93363: F, t93368: F, t93399: F, t93404: F, t93407: F, t93431: F, t93465: F, t93467: F, t93492: F, t93519: F, t93546: F, t93567: F, t93587: F, t93612: F) -> F {
        let t93681 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2020::<F>(t91214, t80761, t80767, t80769, t80776, t91183, t91185, t91187, t91189, t91192, t91196, t91200, t91204, t91206, t91210, t91212, t91216, t91218);
        let t93699 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2021::<F>(t91225, t80780, t80784, t80789, t80792, t80794, t80796, t80801, t80807, t80814, t80821, t80828, t84514, t91229, t91233, t91237, t91241, t91256);
        let t93719 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2022::<F>(t91281, t91283, t91286, t91290, t91300, t80837, t80843, t80857, t80859, t84520, t91261, t91263, t91268, t91272, t91276, t91279, t91294, t91298);
        let t93738 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2023::<F>(t91303, t91305, t91310, t91312, t91327, t91344, t80867, t80870, t80872, t91317, t91319, t91321, t91323, t91330, t91333, t91336, t91340, t91346);
        let t93756 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2024::<F>(t91356, t91358, t91364, t91386, t80889, t80915, t84533, t84536, t91354, t91362, t91366, t91370, t91374, t91378, t91381, t91384, t91389, t91391);
        let t93773 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2025::<F>(t91394, t91398, t91402, t91404, t80920, t80922, t80940, t80943, t80959, t80989, t80992, t80998, t81007, t84555, t84558, t91400, t91413, t91416);
        let (t93776, t93784) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2026::<F>(t93642, t93661, t93681, t93699, t93719, t93738, t93756, t93773, t16060, t27086, t3777, t544, t553, t7209, t81127, t81140, t81149, t81160, t81184, t84595, t84597, t91002, t91008, t91014, t91025, t91036, t93615, t93618);
        let t93809 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2027::<F>(t91064, t91076, t91078, t91081, t3787, t7918, t1336, t1814, t24116, t24121, t3793, t5230, t5287, t7211, t81187, t81189, t81197, t81216, t81218, t81230, t91048, t91052, t91074, t91091);
        let (t93818, t93824, t93847) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2028::<F>(t12020, t7213, t90723, t12444, t1375, t1385, t16453, t1807, t2092, t24063, t26990, t27114, t3887, t55093, t568, t7194, t7937, t81307, t81311, t90665, t90728, t90737, t90741);
        let t93879 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2029::<F>(t91486, t12030, t16452, t1843, t2092, t24139, t26224, t26989, t27068, t3889, t5215, t55150, t7937, t81365, t81375, t84700, t91478, t91482);
        let t93914 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2030::<F>(t91531, t91548, t12033, t16022, t26990, t27115, t3752, t3882, t568, t7199, t7214, t7918, t7937, t81393, t81395, t84705, t91505);
        let t93930 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2031::<F>(t12030, t12444, t12545, t1266, t12724, t12728, t12823, t1323, t1375, t1378, t1386, t1390, t16030, t16437, t16439, t16460, t16470, t16471, t1842, t1843, t19577, t1983, t2075, t2092, t22574, t22607, t2320, t23951, t24092, t24138, t24139, t24147, t26161, t26224, t26558, t26875, t26967, t26969, t26989, t27009, t27051, t27132, t27145, t27180, t32193, t3652, t3758, t3882, t3887, t3889, t3912, t4034, t510, t5210, t5215, t5321, t5325, t533, t5353, t53866, t539, t54825, t55069, t56404, t568, t6876, t6879, t7042, t7191, t7194, t7199, t7213, t7214, t7685, t7787, t7806, t7890, t7904, t7925, t7943, t80683, t80722, t80725, t80728, t81284, t81328, t81350, t81379, t84433, t84659, t90442, t90509, t90519, t90556, t90560, t90568, t90571, t90602, t90612, t90615, t91449, t91455, t91469, t91870, t93286, t93332, t93333, t93335, t93337, t93338, t93341, t93344, t93363, t93368, t93399, t93404, t93407, t93431, t93465, t93467, t93492, t93519, t93546, t93567, t93587, t93612, t93776, t93784, t93809, t93818, t93824, t93847, t93879, t93914);
    t93930
}
