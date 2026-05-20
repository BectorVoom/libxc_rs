//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta785 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2703;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2704;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2705;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2706;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2707;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2708;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2709;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2710;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2711;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2712;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2713;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta785<F: Float>(t1410: F, t1434: F, t19335: F, t19338: F, t19343: F, t19346: F, t19349: F, t19404: F, t20227: F, t3961: F, t3967: F, t4018: F, t5400: F, t5403: F, t5427: F, t642: F, t80: F, t12680: F, t1420: F, t16558: F, t19368: F, t19390: F, t19391: F, t19394: F, t19398: F, t20217: F, t20234: F, t2267: F, t39: F, t39159: F, t3966: F, t3981: F, t3991: F, t45970: F, t45974: F, t51: F, t5398: F, t5416: F, t607: F, t68513: F, t12698: F, t19401: F, t20235: F, t20238: F, t20241: F, t20246: F, t2274: F, t39168: F, t39210: F, t3990: F, t3994: F, t43: F, t55: F, t615: F, t621: F, t67060: F, t12595: F, t12598: F, t12609: F, t12612: F, t17635: F, t19420: F, t19430: F, t2291: F, t2298: F, t39096: F, t39114: F, t4007: F, t4012: F, t634: F, t638: F, t1427: F, t19326: F, t19405: F, t19441: F, t20210: F, t20265: F, t20285: F, t33: F, t3997: F, t3998: F, t5392: F, t5393: F, t5428: F, t5442: F, t629: F, t66: F, t72: F, t1437: F, t19445: F, t20201: F, t20204: F, t20288: F, t2235: F, t2240: F, t39054: F, t39063: F, t3953: F, t4021: F, t5389: F, t5445: F, t605: F, t645: F, t75356: F, t75392: F, t9231: F, t9239: F, t5: F, t12568: F, t12571: F, t19299: F, t19310: F, t19313: F, t19318: F, t39043: F, t3958: F, t45844: F, t46085: F, t46086: F, t46087: F, t46088: F, t46089: F, t46090: F, t46104: F, t55880: F, t55921: F, t75284: F, t86: F, t112: F, t5449: F, t671: F, t20305: F, t626: F, t20308: F, t1453: F, t5488: F, t20343: F, t1444: F, t5396: F, t16: F, t39031: F, t12774: F, t19503: F, t2: F, t20311: F, t20312: F, t20315: F, t20318: F, t20319: F, t20322: F, t2219: F, t2341: F, t4049: F, t4060: F, t45496: F, t45697: F, t5468: F, t5475: F, t584: F, t657: F, t659: F, t663: F, t92: F, t95: F, t1449: F, t5484: F, t100: F, t103: F, t12795: F, t1447: F, t19514: F, t19518: F, t19522: F, t19525: F, t19526: F, t20331: F, t20338: F, t2349: F, t4059: F, t4064: F, t45460: F, t45707: F, t5480: F, t55491: F, t662: F, t12757: F, t19473: F, t19529: F, t20304: F, t20342: F, t2331: F, t29903: F, t4043: F, t4067: F, t45435: F, t55420: F, t64: F, t656: F, t666: F, t109: F, t45421: F, t45422: F, t45656: F, t45659: F, t45689: F, t55531: F, t55537: F, t55546: F, t55559: F, t55561: F, t1268: F, t12725: F, t1458: F, t19451: F, t19456: F, t19534: F, t20347: F, t2314: F, t26114: F, t26117: F, t28002: F, t4028: F, t4072: F, t5113: F, t5493: F, t55943: F, t67001: F, t75275: F, t7676: F) -> (F, F, F, F) {
        let t75419 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2703::<F>(t1410, t1434, t19335, t19338, t19343, t19346, t19349, t19404, t20227, t3961, t3967, t4018, t5400, t5403, t5427, t642, t80);
        let t75461 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2704::<F>(t12680, t1420, t16558, t19368, t19390, t19391, t19394, t19398, t20217, t20234, t2267, t39, t39159, t3966, t3981, t3991, t45970, t45974, t51, t5398, t5416, t607, t68513);
        let t75494 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2705::<F>(t12698, t1420, t16558, t19401, t20217, t20234, t20235, t20238, t20241, t20246, t2274, t39, t39168, t39210, t3990, t3994, t43, t51, t5398, t5416, t55, t607, t615, t621, t67060);
        let t75543 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2706::<F>(t12595, t12598, t12609, t12612, t16558, t17635, t19420, t19430, t20217, t20234, t2291, t2298, t39096, t39114, t3966, t4007, t4012, t5398, t607, t634, t638, t67060);
        let t75547 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2707::<F>(t1427, t1434, t19326, t19405, t19441, t20210, t20265, t20285, t33, t3997, t3998, t4018, t5392, t5393, t5428, t5442, t629, t642, t66, t72, t75461, t75494, t75543, t80);
        let t75552 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2708::<F>(t1437, t19445, t20201, t20204, t20288, t2235, t2240, t39054, t39063, t3953, t4021, t5389, t5445, t605, t645, t75356, t75392, t75419, t75547, t9231, t9239);
        let t75554 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2709::<F>(t5, t12568, t12571, t1437, t19299, t19310, t19313, t19318, t39043, t3958, t4021, t45844, t46085, t46086, t46087, t46088, t46089, t46090, t46104, t5389, t5445, t55880, t55921, t645, t75284, t75552, t86);
        let (t75555, t75560, t75592, t75601, t75603, t75613, t75631) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2710::<F>(t112, t75554, t5449, t671, t20305, t626, t20308, t1453, t5488, t20343, t1444, t5396);
        let (t75649, t75657) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2711::<F>(t16, t39031, t12774, t19503, t2, t20311, t20312, t20315, t20318, t20319, t20322, t2219, t2341, t4049, t4060, t45496, t45697, t5396, t5468, t5475, t584, t657, t659, t663, t75631, t92, t95);
        let t75694 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2712::<F>(t1449, t5484, t100, t103, t12795, t1447, t19514, t19518, t19522, t19525, t19526, t2, t20331, t20338, t2219, t2349, t4059, t4064, t45460, t45707, t5475, t5480, t55491, t584, t662, t75649);
        let t75699 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2713::<F>(t12757, t19473, t19529, t20304, t20342, t2331, t29903, t4043, t4067, t45435, t5488, t55420, t64, t656, t666, t75592, t75601, t75603, t75613, t75657, t75694);
        let (t75701, t75704) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2714::<F>(t109, t45421, t45422, t45656, t45659, t45689, t55531, t55537, t55546, t55559, t55561, t75699, t1268, t12725, t1458, t19451, t19456, t19534, t20347, t2314, t26114, t26117, t28002, t4028, t4072, t5113, t5493, t55943, t67001, t671, t75275, t75555, t75560, t7676);
    (t75555, t75560, t75701, t75704)
}
