//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta392 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1481;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1482;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1483;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1484;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1485;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1486;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1487;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1488;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1489;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta392<F: Float>(t11814: F, t3572: F, t11825: F, t3523: F, t11820: F, t3536: F, t11778: F, t121: F, t11148: F, t1227: F, t248: F, t11728: F, t11729: F, t3570: F, t1229: F, t204: F, t1090: F, t11692: F, t1174: F, t1177: F, t11779: F, t11781: F, t1213: F, t1214: F, t1216: F, t3490: F, t3515: F, t3527: F, t3578: F, t3585: F, t43719: F, t43752: F, t43792: F, t43796: F, t44668: F, t44798: F, t45250: F, t45251: F, t45256: F, t45260: F, t475: F, t44878: F, t44943: F, t44999: F, t45066: F, t45133: F, t45186: F, t45246: F, t3609: F, t44927: F, t3623: F, t11880: F, t44690: F, t11913: F, t11881: F, t11883: F, t11884: F, t11897: F, t11916: F, t1244: F, t1246: F, t3604: F, t3610: F, t3612: F, t3613: F, t3626: F, t44669: F, t44673: F, t44700: F, t44710: F, t44785: F, t44786: F, t470: F, t491: F, t493: F, t11598: F, t11608: F, t11613: F, t11616: F, t11620: F, t11621: F, t11625: F, t11639: F, t11640: F, t11868: F, t11869: F, t11872: F, t11877: F, t11888: F, t11889: F, t11890: F, t11904: F, t11907: F, t11910: F, t11914: F, t11918: F, t11919: F, t11925: F, t11928: F, t11935: F, t1201: F, t1215: F, t1235: F, t1238: F, t1241: F, t1249: F, t1251: F, t1252: F, t15429: F, t3481: F, t3487: F, t3493: F, t3565: F, t3590: F, t3593: F, t3598: F, t3600: F, t3620: F, t3621: F, t3624: F, t3625: F, t3628: F, t3631: F, t44412: F, t44657: F, t44662: F, t44691: F, t44698: F, t44701: F, t44748: F, t44753: F, t44754: F, t44774: F, t494: F, t498: F, t5079: F, t11931: F, t225: F, t11604: F, t496: F, t68: F, t3599: F, t11601: F, t11599: F, t11606: F, t1190: F, t3630: F, t466: F, t1256: F, t193: F, t336: F, t3640: F, t44161: F, t44164: F, t44167: F, t44358: F, t44375: F, t44377: F, t44378: F, t44384: F, t44388: F, t44392: F, t44396: F, t44400: F, t28: F, t265: F, t504: F, t41606: F, t43920: F, t43990: F, t44373: F, t10150: F, t1081: F, t11122: F, t11957: F, t1260: F, t2250: F, t2756: F, t3231: F, t3644: F, t39110: F, t39448: F, t506: F, t52: F, t607: F, t873: F, t9258: F, dens_threshold: F, rho1: F, zeta_threshold: F, t113: F, t12492: F, t12507: F, t1266: F, t1271: F, t12734: F, t1393: F, t2314: F, t2320: F, t2364: F, t3652: F, t3660: F, t39223: F, t39231: F, t39235: F, t3929: F, t39332: F, t39385: F, t39480: F, t39524: F, t39586: F, t39626: F, t39847: F, t40615: F, t43657: F, t510: F, t513: F, t672: F, t89: F, t9347: F, t9351: F, t9419: F, t2311: F, t2319: F, t107: F, t9576: F, t2585: F, t667: F, t2281: F, t2333: F, t2359: F, t626: F, t9367: F, t9371: F) -> (F, F, F, F, F, F, F, F) {
        let (t45262, t45264, t45266, t45271, t45283) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1481::<F>(t11814, t3572, t11825, t3523, t11820, t3536, t11778, t121, t11148, t1227, t248, t11728, t11729, t3570);
        let t45311 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1482::<F>(t1229, t204, t1090, t1227, t248, t11692, t1174, t1177, t11779, t11781, t11825, t1213, t1214, t1216, t3490, t3515, t3527, t3578, t3585, t43719, t43752, t43792, t43796, t44668, t44798, t45250, t45251, t45256, t45260, t45262, t45264, t45266, t45271, t45283, t475);
        let (t45314, t45320, t45323, t45326) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1483::<F>(t44878, t44943, t44999, t45066, t45133, t45186, t45246, t45311, t3609, t44927, t3623, t11880, t44690);
        let t45332 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1484::<F>(t11913, t44690, t11881, t11883, t11884, t11897, t11916, t1244, t1246, t3604, t3610, t3612, t3613, t3626, t44669, t44673, t44700, t44710, t44785, t44786, t44798, t45314, t45320, t45323, t45326, t470, t491, t493);
        let t45344 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1485::<F>(t11598, t11608, t11613, t11616, t11620, t11621, t11625, t11639, t11640, t11868, t11869, t11872, t11877, t11888, t11889, t11890, t11904, t11907, t11910, t11914, t11918, t11919, t11925, t11928, t11935, t1201, t1215, t1235, t1238, t1241, t1244, t1246, t1249, t1251, t1252, t15429, t3481, t3487, t3493, t3565, t3590, t3593, t3598, t3600, t3604, t3620, t3621, t3624, t3625, t3628, t3631, t44412, t44657, t44662, t44669, t44673, t44691, t44698, t44700, t44701, t44748, t44753, t44754, t44774, t45332, t491, t494, t498, t5079);
        let t45382 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1486::<F>(t11931, t225, t11604, t496, t68, t3599, t11601, t11599, t11606, t11608, t11613, t11868, t1190, t11919, t11925, t11928, t11935, t1238, t1252, t3487, t3593, t3600, t3630, t3631, t45314, t466, t498);
        let t45387 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1487::<F>(t1256, t193, t336, t3640, t44161, t44164, t44167, t44358, t44375, t44377, t44378, t44384, t44388, t44392, t44396, t44400, t45344, t45382);
        let t45402 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1488::<F>(t28, t265, t504, t41606, t43920, t43990, t44373, t45387, t10150, t1081, t11122, t11957, t1260, t2250, t2756, t3231, t3644, t39110, t39448, t506, t52, t607, t873, t9258, dens_threshold, rho1, zeta_threshold);
        let t45405 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1489::<F>(t113, t12492, t12507, t1266, t1271, t12734, t1393, t2314, t2320, t2364, t3652, t3660, t39223, t39231, t39235, t3929, t39332, t39385, t39480, t39524, t39586, t39626, t39847, t40615, t43657, t45402, t510, t513, t672, t89, t9347, t9351, t9419);
        let (t45408, t45421, t45422, t45424, t45426, t45428, t45430) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1490::<F>(t2311, t2319, t107, t9576, t2585, t667, t2281, t2333, t2359, t626, t9367, t9371);
    (t45405, t45408, t45421, t45422, t45424, t45426, t45428, t45430)
}
