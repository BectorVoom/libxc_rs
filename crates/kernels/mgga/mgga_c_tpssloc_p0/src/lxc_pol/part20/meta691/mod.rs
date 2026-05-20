//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta691 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2623;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2624;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2625;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2626;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2627;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2628;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2629;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2630;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2631;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2632;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2633;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta691<F: Float>(t11734: F, t15548: F, t1174: F, t14749: F, t3431: F, t1222: F, t15723: F, t11738: F, t13969: F, t15534: F, t3514: F, t53371: F, t1213: F, t15525: F, t248: F, t3570: F, t11813: F, t5018: F, t15749: F, t3577: F, t45124: F, t1214: F, t1218: F, t15531: F, t15553: F, t3494: F, t3515: F, t3518: F, t4582: F, t475: F, t52458: F, t11835: F, t4889: F, t1725: F, t2402: F, t11665: F, t11668: F, t11692: F, t11845: F, t11850: F, t1227: F, t14730: F, t14748: F, t15654: F, t15708: F, t15710: F, t3578: F, t45250: F, t4723: F, t48554: F, t52532: F, t52538: F, t53144: F, t3506: F, t4979: F, t49850: F, t11754: F, t11825: F, t4993: F, t15486: F, t3490: F, t11727: F, t52835: F, t11678: F, t11697: F, t15662: F, t11731: F, t11770: F, t14735: F, t15438: F, t15750: F, t52911: F, t53366: F, t15709: F, t1226: F, t15764: F, t11832: F, t1706: F, t15608: F, t11838: F, t11841: F, t11761: F, t1232: F, t14725: F, t45128: F, t45256: F, t45260: F, t45262: F, t3566: F, t5023: F, t15734: F, t11789: F, t4733: F, t11814: F, t15498: F, t3527: F, t3531: F, t45264: F, t45266: F, t45271: F, t45283: F, t45296: F, t5014: F, t11159: F, t11163: F, t11168: F, t11172: F, t11546: F, t11655: F, t11662: F, t11670: F, t11674: F, t11709: F, t11719: F, t11728: F, t11729: F, t11853: F, t11858: F, t1215: F, t1216: F, t15474: F, t15495: F, t15503: F, t15507: F, t15555: F, t15564: F, t15569: F, t15594: F, t15612: F, t15620: F, t15625: F, t15650: F, t15659: F, t15700: F, t15701: F, t15702: F, t15704: F, t15740: F, t1653: F, t1735: F, t1743: F, t2244: F, t2250: F, t3243: F, t3248: F, t3447: F, t3496: F, t3509: F, t3511: F, t3516: F, t3587: F, t44803: F, t44847: F, t44904: F, t44929: F, t44953: F, t45002: F, t45114: F, t45119: F, t45134: F, t45251: F, t45971: F, t45993: F, t4728: F, t48497: F, t488: F, t4972: F, t4980: F, t5005: F, t5012: F, t5024: F, t50910: F, t50915: F, t50992: F, t52165: F, t52548: F, t52554: F, t52568: F, t52606: F, t52639: F, t52668: F, t52674: F, t52680: F, t52682: F, t52684: F, t52687: F, t52696: F, t52737: F, t52769: F, t52797: F, t52801: F, t52810: F, t52813: F, t52817: F, t52853: F, t52886: F, t52893: F, t52897: F, t52928: F, t52932: F, t52935: F, t52942: F, t52953: F, t52989: F, t53013: F, t53037: F, t53064: F, t53067: F, t53106: F, t53129: F, t53149: F, t53167: F, t53176: F, t53185: F, t53187: F, t53236: F, t53258: F, t53276: F, t53287: F, t53291: F, t53298: F, t53345: F, t53377: F, t11638: F, t11868: F, t11877: F, t11881: F, t1235: F, t1244: F, t1246: F, t14985: F, t14989: F, t15000: F, t15239: F, t1734: F, t1755: F, t3590: F, t3604: F, t3610: F, t3612: F, t3624: F, t3625: F, t470: F, t493: F, t5011: F, t5068: F, t5072: F, t5073: F, t5079: F, t52500: F, t11712: F, t11913: F, t491: F, t11887: F, t52834: F, t11616: F, t11640: F, t11890: F, t11907: F, t11914: F, t15022: F, t15023: F, t15240: F, t15241: F, t15248: F, t15429: F, t1758: F, t44691: F, t45323: F, t5064: F, t5075: F, t5080: F, t11880: F, t11871: F, t11897: F, t11904: F, t14997: F, t15032: F, t15430: F, t15777: F, t1932: F, t3493: F, t3621: F, t45329: F, t5052: F, t5083: F, t5084: F, t52480: F, t52709: F) -> (F, F, F, F, F, F, F) {
        let (t53378, t53387, t53389, t53397, t53399) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2623::<F>(t11734, t15548, t1174, t14749, t3431, t1222, t15723, t11738, t13969, t15534, t3514, t53371);
        let t53412 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2624::<F>(t1213, t15525, t248, t3570, t11813, t5018, t15749, t3577, t45124, t11734, t1214, t1218, t15531, t15553, t3494, t3515, t3518, t4582, t475, t52458, t53378, t53387, t53389, t53397, t53399);
        let t53446 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2625::<F>(t11835, t4889, t1174, t1725, t2402, t11665, t11668, t11692, t11845, t11850, t1227, t14730, t14748, t15654, t15708, t15710, t3577, t3578, t45250, t4582, t4723, t48554, t52532, t52538, t53144);
        let (t53453, t53456, t53468, t53470, t53472, t53476) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2626::<F>(t3506, t4979, t49850, t11754, t4889, t11825, t4993, t15486, t3490, t11727, t52835, t11678, t11697, t15662);
        let t53478 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2627::<F>(t11665, t11668, t11678, t11731, t11770, t14735, t15438, t15708, t15750, t3577, t4723, t52911, t53366, t53453, t53456, t53468, t53470, t53472, t53476);
        let (t53481, t53487, t53490, t53494, t53496, t53498) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2628::<F>(t11697, t15709, t3577, t1226, t15764, t11832, t1706, t11665, t15608, t11838, t4889, t11841);
        let t53503 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2629::<F>(t11761, t1232, t14725, t3577, t45128, t45256, t45260, t45262, t4889, t52538, t53481, t53487, t53490, t53494, t53496, t53498);
        let t53524 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2630::<F>(t3566, t5023, t15734, t3490, t11789, t1227, t248, t4733, t11814, t1232, t15498, t3527, t3531, t45264, t45266, t45271, t45283, t45296, t5014);
        let t53529 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2631::<F>(t11159, t11163, t11168, t11172, t11546, t11655, t11662, t11665, t11668, t11670, t11674, t11678, t11692, t11709, t11719, t11728, t11729, t1174, t11770, t11853, t11858, t1215, t1216, t1218, t1227, t15474, t15495, t15503, t15507, t15553, t15555, t15564, t15569, t15594, t15612, t15620, t15625, t15650, t15659, t15700, t15701, t15702, t15704, t15740, t1653, t1735, t1743, t2244, t2250, t3243, t3248, t3447, t3490, t3494, t3496, t3506, t3509, t3511, t3516, t3518, t3527, t3577, t3578, t3587, t44803, t44847, t44904, t44929, t44953, t45002, t45114, t45119, t45134, t45251, t4582, t45971, t45993, t4723, t4728, t4733, t48497, t488, t4972, t4980, t5005, t5012, t5024, t50910, t50915, t50992, t52165, t52532, t52538, t52548, t52554, t52568, t52606, t52639, t52668, t52674, t52680, t52682, t52684, t52687, t52696, t52737, t52769, t52797, t52801, t52810, t52813, t52817, t52853, t52886, t52893, t52897, t52928, t52932, t52935, t52942, t52953, t52989, t53013, t53037, t53064, t53067, t53106, t53129, t53149, t53167, t53176, t53185, t53187, t53236, t53258, t53276, t53287, t53291, t53298, t53345, t53377, t53412, t53446, t53478, t53503, t53524);
        let t53538 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2632::<F>(t11638, t11868, t11877, t11881, t1235, t1244, t1246, t14985, t14989, t15000, t15239, t1734, t1755, t3590, t3604, t3610, t3612, t3624, t3625, t470, t493, t5011, t5068, t5072, t5073, t5079, t52500, t53529);
        let (t53545, t53590) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2633::<F>(t11712, t11913, t491, t11887, t52834, t11616, t11640, t11890, t11907, t11914, t15022, t15023, t15240, t15241, t15248, t15429, t1758, t3604, t3624, t44691, t45323, t5064, t5072, t5075, t5079, t5080);
        let (t53592, t53613, t53650) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2634::<F>(t11913, t52834, t11880, t11712, t11887, t491, t11638, t11871, t11877, t11897, t11904, t1244, t1246, t14997, t15022, t15032, t15430, t15777, t1755, t1932, t3493, t3604, t3610, t3621, t3624, t45329, t475, t5052, t5064, t5083, t5084, t52480, t52709);
    (t53529, t53538, t53545, t53590, t53592, t53613, t53650)
}
