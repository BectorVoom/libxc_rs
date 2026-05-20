//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta705 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2308;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2309;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2310;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2311;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2312;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2313;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2314;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2315;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2316;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2317;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2318;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2319;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta705<F: Float>(t5932: F, t6743: F, t28653: F, t82822: F, t1014: F, t1058: F, t1060: F, t11046: F, t14608: F, t1625: F, t17959: F, t18093: F, t1945: F, t23478: F, t23601: F, t23602: F, t23633: F, t25492: F, t25516: F, t25554: F, t25558: F, t25712: F, t28596: F, t28601: F, t28641: F, t3186: F, t4673: F, t6687: F, t82717: F, t89175: F, t89224: F, t5936: F, t1022: F, t5392: F, t6800: F, t23518: F, t5928: F, t17843: F, t1949: F, t23346: F, t23604: F, t28602: F, t28610: F, t3180: F, t5844: F, t6805: F, t83239: F, t83240: F, t83245: F, t884: F, t89256: F, t89292: F, t89294: F, t89296: F, t23384: F, t28657: F, t1615: F, t18107: F, t23327: F, t23613: F, t25429: F, t25510: F, t25549: F, t25705: F, t25713: F, t2770: F, t2775: F, t28609: F, t28614: F, t3200: F, t3961: F, t7619: F, t883: F, t89309: F, t89310: F, t89327: F, t99180: F, t1003: F, t17187: F, t18086: F, t23635: F, t25500: F, t28634: F, t28660: F, t353: F, t383: F, t4542: F, t4669: F, t5398: F, t6784: F, t6785: F, t6811: F, t7614: F, t82668: F, t83233: F, t89329: F, t99859: F, t1920: F, t28630: F, t968: F, t5872: F, t6768: F, t83244: F, t89242: F, t18047: F, t1948: F, t28663: F, t3201: F, t345: F, t4649: F, t5838: F, t7593: F, t89360: F, t89362: F, t89366: F, t89369: F, t89505: F, t986: F, t17686: F, t17691: F, t18138: F, t23670: F, t25721: F, t28593: F, t28618: F, t28622: F, t28637: F, t28652: F, t3966: F, t82625: F, t82799: F, t88022: F, t89071: F, t89176: F, t28671: F, t82736: F, t14651: F, t1599: F, t25479: F, t25535: F, t3188: F, t7620: F, t82809: F, t89243: F, t89421: F, t89429: F, t89431: F, t89445: F, t89501: F, t28557: F, t11034: F, t1409: F, t1539: F, t23685: F, t25497: F, t28642: F, t28674: F, t4684: F, t5681: F, t5866: F, t6797: F, t6801: F, t82830: F, t89235: F, t89449: F, t362: F, t5914: F, t14618: F, t25568: F, t25708: F, t28605: F, t28631: F, t5685: F, t5903: F, t6680: F, t6813: F, t7603: F, t89532: F, t89546: F, t99921: F, t100025: F, t100068: F, t100103: F, t100147: F, t100176: F, t100195: F, t1052: F, t1055: F, t1603: F, t17875: F, t18070: F, t18074: F, t18166: F, t23581: F, t25755: F, t25757: F, t28499: F, t28679: F, t3169: F, t388: F, t4694: F, t5848: F, t6699: F, t6771: F, t6816: F, t83459: F, t88851: F, t89662: F, t89672: F, t99983: F, t28719: F, t3216: F, t1068: F, t1070: F, t1637: F, t18169: F, t193: F, t23738: F, t23742: F, t25840: F, t25845: F, t336: F, t4696: F, t4700: F, t5946: F, t5950: F, t6822: F, t83472: F, t83479: F, t89698: F, t89702: F, t99104: F, t99143: F, t99172: F, t99202: F, t99238: F, t99271: F, t99313: F, t99353: F, t99390: F, t99422: F, t99450: F, t99866: F, t99894: F, t99930: F, t99959: F, t25365: F, t57911: F, t10143: F, t1484: F, t25374: F, t16596: F, t16944: F, t16949: F, t1877: F, t1915: F, t202: F, t22959: F, t23290: F, t23295: F, t25013: F, t2522: F, t25354: F, t25358: F, t28248: F, t4255: F, t4314: F, t5544: F, t6666: F, t6670: F, t67128: F, t7541: F, t82312: F, t870: F, t97999: F, t98003: F, t98007: F, t98011: F, t99042: F) -> (F, F) {
        let t100225 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2308::<F>(t5932, t6743, t28653, t82822, t1014, t1058, t1060, t11046, t14608, t1625, t17959, t18093, t1945, t23478, t23601, t23602, t23633, t25492, t25516, t25554, t25558, t25712, t28596, t28601, t28641, t3186, t4673, t6687, t82717, t89175, t89224);
        let (t100236, t100253) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2309::<F>(t5936, t6743, t1022, t5392, t6800, t23518, t5928, t17843, t1949, t23346, t23604, t23633, t25554, t28602, t28610, t3180, t5844, t6687, t6805, t83239, t83240, t83245, t884, t89256, t89292, t89294, t89296);
        let t100287 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2310::<F>(t23384, t28657, t1058, t1060, t1615, t1625, t18107, t23327, t23346, t23613, t23633, t25429, t25510, t25549, t25705, t25713, t2770, t2775, t28609, t28614, t3200, t3961, t6687, t6743, t7619, t883, t89309, t89310, t89327, t99180);
        let t100314 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2311::<F>(t100236, t1003, t1022, t17187, t18086, t23346, t23633, t23635, t25500, t28634, t28653, t28660, t353, t383, t4542, t4669, t5398, t6687, t6784, t6785, t6800, t6811, t7614, t82668, t83233, t89329, t99859);
        let (t100326, t100334, t100341) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2312::<F>(t1920, t28630, t968, t5872, t6768, t83244, t89242, t1058, t1060, t18047, t1948, t23346, t28663, t3200, t3201, t345, t4649, t5838, t6687, t6805, t7593, t89360, t89362, t89366, t89369, t89505, t986);
        let t100377 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2313::<F>(t1022, t1058, t1060, t1615, t17686, t17691, t18138, t23346, t23613, t23633, t23635, t23670, t25429, t25510, t25721, t28593, t28618, t28622, t28637, t28652, t3186, t3966, t6800, t7619, t82625, t82799, t88022, t89071, t89176);
        let t100396 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2314::<F>(t23384, t28618, t28671, t82736, t100326, t100334, t14651, t1599, t25479, t25535, t3186, t3188, t6687, t7620, t82809, t89243, t89421, t89429, t89431, t89445, t89501);
        let t100430 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2315::<F>(t23384, t28610, t28557, t6743, t1058, t1060, t11034, t1409, t1539, t23633, t23635, t23685, t25497, t28601, t28642, t28674, t3180, t3200, t4649, t4669, t4684, t5681, t5866, t6687, t6768, t6784, t6797, t6800, t6801, t82830, t89235, t89449);
        let t100459 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2316::<F>(t23384, t28660, t28614, t362, t5914, t14618, t23327, t23670, t23685, t25568, t25708, t25713, t28605, t28631, t4669, t5685, t5903, t6680, t6687, t6784, t6813, t7603, t884, t89532, t89546, t99921);
        let t100489 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2317::<F>(t100025, t100068, t100103, t100147, t100176, t100195, t100225, t100253, t100287, t100314, t100341, t100377, t100396, t100430, t100459, t1052, t1055, t1603, t17875, t18070, t18074, t18166, t1945, t23581, t25705, t25755, t25757, t28499, t28679, t3169, t388, t4694, t5838, t5848, t6687, t6699, t6768, t6771, t6816, t83459, t88851, t89662, t89672, t99983);
        let t100528 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2318::<F>(t28719, t3216, t100489, t1068, t1070, t1637, t18169, t193, t23738, t23742, t25840, t25845, t336, t4696, t4700, t5946, t5950, t6822, t83472, t83479, t89698, t89702, t99104, t99143, t99172, t99202, t99238, t99271, t99313, t99353, t99390, t99422, t99450, t99866, t99894, t99930, t99959);
        let t100578 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2319::<F>(t25365, t57911, t10143, t1484, t25374, t16596, t16944, t16949, t1877, t1915, t193, t202, t22959, t23290, t23295, t25013, t2522, t25354, t25358, t28248, t4255, t4314, t5544, t6666, t6670, t67128, t7541, t82312, t870, t97999, t98003, t98007, t98011, t99042);
    (t100528, t100578)
}
