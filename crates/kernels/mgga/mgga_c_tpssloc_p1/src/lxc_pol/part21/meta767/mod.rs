//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta767 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2645;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2646;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2647;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2648;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2649;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2650;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2651;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta767<F: Float>(t112: F, t16506: F, t111: F, t5363: F, t1851: F, t3946: F, t1858: F, t3931: F, t1395: F, t5381: F, t1404: F, t6470: F, t2319: F, t4072: F, t12521: F, t12524: F, t12813: F, t1458: F, t16535: F, t19534: F, t20181: F, t2363: F, t3941: F, t45560: F, t5371: F, t5376: F, t5456: F, t5493: F, t671: F, t19530: F, t626: F, t12774: F, t12795: F, t12802: F, t1447: F, t16: F, t19488: F, t19489: F, t19492: F, t19499: F, t19503: F, t19504: F, t19517: F, t2219: F, t2248: F, t2336: F, t2341: F, t2351: F, t2355: F, t30171: F, t30307: F, t45697: F, t45707: F, t45751: F, t45762: F, t5469: F, t5472: F, t5475: F, t657: F, t659: F, t92: F, t2349: F, t100: F, t12792: F, t12796: F, t12799: F, t12805: F, t19493: F, t19498: F, t19513: F, t19521: F, t19525: F, t21: F, t2342: F, t2350: F, t2354: F, t4049: F, t4059: F, t45460: F, t45496: F, t45717: F, t5396: F, t5468: F, t5480: F, t5484: F, t584: F, t662: F, t9: F, t9384: F, t9398: F, t4067: F, t2331: F, t45421: F, t45422: F, t45424: F, t45426: F, t45656: F, t45658: F, t45660: F, t45662: F, t45688: F, t45690: F, t64: F, t656: F, t2281: F, t5489: F, t5465: F, t19474: F, t19483: F, t19477: F, t12808: F, t19473: F, t19482: F, t19529: F, t2332: F, t2358: F, t26129: F, t29903: F, t4043: F, t45435: F, t45676: F, t5464: F, t5488: F, t666: F, t9365: F, t109: F, t576: F, t12649: F, t12652: F, t12653: F, t12656: F, t12661: F, t12708: F, t1410: F, t1426: F, t1434: F, t19343: F, t19346: F, t19349: F, t19441: F, t2304: F, t3961: F, t3962: F, t3967: F, t3997: F, t4018: F, t5403: F, t609: F, t642: F, t80: F, t1409: F, t628: F, t67: F, t2250: F, t5398: F, t16558: F, t607: F, t12606: F, t12620: F, t12623: F, t12662: F, t12665: F, t1411: F, t17635: F, t1864: F, t19322: F, t19323: F, t19363: F, t19404: F, t2251: F, t3966: F, t3968: F, t3971: F, t5427: F, t608: F, t65: F, t6509: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55341, t55353, t55368, t55374, t55376, t55378, t55388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2645::<F>(t112, t16506, t111, t5363, t1851, t3946, t1858, t3931, t1395, t5381, t1404, t6470);
        let (t55410, t55417) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2646::<F>(t1851, t2319, t4072, t12521, t12524, t12813, t1458, t16535, t19534, t20181, t2363, t3941, t45560, t5371, t5376, t5456, t5493, t55341, t55353, t55388, t671);
        let (t55420, t55457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2647::<F>(t19530, t626, t12774, t12795, t12802, t1447, t16, t19488, t19489, t19492, t19499, t19503, t19504, t19517, t2219, t2248, t2336, t2341, t2351, t2355, t30171, t30307, t45697, t45707, t45751, t45762, t5469, t5472, t5475, t657, t659, t92);
        let t55512 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2648::<F>(t1447, t2349, t100, t12792, t12796, t12799, t12805, t19493, t19498, t19513, t19521, t19525, t21, t2248, t2341, t2342, t2350, t2354, t4049, t4059, t45460, t45496, t45717, t5396, t5468, t5480, t5484, t584, t662, t9, t92, t9384, t9398);
        let t55530 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2649::<F>(t4067, t2331, t45421, t45422, t45424, t45426, t45656, t45658, t45660, t45662, t45688, t45690, t55420, t55457, t55512, t64, t656);
        let t55566 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2650::<F>(t2281, t5489, t5465, t19474, t626, t19483, t19477, t12808, t19473, t19482, t19529, t2331, t2332, t2358, t26129, t29903, t4043, t4067, t45435, t45676, t5464, t5488, t64, t666, t9365);
        let (t55568, t55571, t55631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2651::<F>(t109, t55530, t55566, t2363, t576, t12649, t12652, t12653, t12656, t12661, t12708, t1410, t1426, t1434, t19343, t19346, t19349, t19441, t2304, t3961, t3962, t3967, t3997, t4018, t5403, t609, t642, t80);
        let (t55662, t55666, t55673) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2652::<F>(t1409, t628, t67, t2250, t5398, t16558, t607, t12606, t12620, t12623, t12662, t12665, t1411, t1434, t17635, t1864, t19322, t19323, t19363, t19404, t2251, t3966, t3968, t3971, t4018, t5427, t608, t642, t65, t6509, t80);
    (t55368, t55374, t55376, t55378, t55410, t55417, t55568, t55571, t55631, t55662, t55666, t55673)
}
