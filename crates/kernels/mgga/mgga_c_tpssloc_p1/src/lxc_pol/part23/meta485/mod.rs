//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta485 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1484;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1485;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1486;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1487;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1488;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta485<F: Float>(t1410: F, t1434: F, t1864: F, t19322: F, t20207: F, t20217: F, t20222: F, t20227: F, t20264: F, t20265: F, t33: F, t5398: F, t5399: F, t5400: F, t5427: F, t5442: F, t65: F, t7445: F, t75361: F, t75847: F, t79692: F, t80: F, t12571: F, t1437: F, t19299: F, t20201: F, t20204: F, t20288: F, t2240: F, t39030: F, t39032: F, t39034: F, t39036: F, t39038: F, t39040: F, t39043: F, t39063: F, t3953: F, t45844: F, t5389: F, t5445: F, t55921: F, t605: F, t75284: F, t79579: F, t79585: F, t79637: F, t86: F, t9239: F, t5: F, t112: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t19451: F, t20347: F, t20698: F, t20702: F, t20717: F, t22425: F, t28002: F, t4028: F, t510: F, t5450: F, t5457: F, t5494: F, t6287: F, t652: F, t67001: F, t7458: F, t77944: F, t79553: F, t5464: F, t5488: F, t5468: F, t5396: F, t5480: F, t5484: F, t75910: F, t100: F, t103: F, t104: F, t1447: F, t1450: F, t19488: F, t19513: F, t20245: F, t20318: F, t20322: F, t20332: F, t20335: F, t20338: F, t20339: F, t2341: F, t2349: F, t4049: F, t4059: F, t45460: F, t45496: F, t5475: F, t5481: F, t5485: F, t92: F, t95: F, tau1: F, t19473: F, t20342: F, t2331: F, t4043: F, t45421: F, t45435: F, t45656: F, t55531: F, t55537: F, t64: F, t656: F, t75592: F, t75601: F, t75613: F, t109: F, t5493: F, t5449: F, t5456: F, t53777: F, t53779: F, t56099: F, t56104: F, t73967: F, t53798: F, t1799: F, t19596: F, t20067: F, t20675: F, t28830: F, t3918: F, t39249: F, t39256: F, t39261: F, t5160: F, t5161: F, t6347: F, t74068: F, t75240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t79707 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1484::<F>(t1410, t1434, t1864, t19322, t20207, t20217, t20222, t20227, t20264, t20265, t33, t5398, t5399, t5400, t5427, t5442, t65, t7445, t75361, t75847, t79692, t80);
        let t79711 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1485::<F>(t12571, t1437, t19299, t20201, t20204, t20288, t2240, t39030, t39032, t39034, t39036, t39038, t39040, t39043, t39063, t3953, t45844, t5389, t5445, t55921, t605, t75284, t79579, t79585, t79637, t79707, t86, t9239);
        let (t79713, t79729) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1486::<F>(t5, t79711, t112, t113, t1442, t1459, t1774, t1778, t19451, t20347, t20698, t20702, t20717, t22425, t28002, t4028, t510, t5450, t5457, t5494, t6287, t652, t67001, t7458, t77944, t79553);
        let (t79748, t79755, t79812) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1487::<F>(t5464, t5488, t5468, t5396, t5480, t5484, t75910, t100, t103, t104, t1447, t1450, t19488, t19513, t20245, t20318, t20322, t20332, t20335, t20338, t20339, t2341, t2349, t4049, t4059, t45460, t45496, t5475, t5481, t5485, t92, t95, tau1);
        let t79816 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1488::<F>(t19473, t20342, t2331, t4043, t45421, t45435, t45656, t5488, t55531, t55537, t64, t656, t75592, t75601, t75613, t79748, t79755, t79812);
        let (t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1489::<F>(t109, t79816, t5493, t5449, t5456, t53777, t53779, t56099, t56104, t73967, t53798, t1799, t19596, t20067, t20675, t28830, t3918, t39249, t39256, t39261, t5160, t5161, t6347, t74068, t75240);
    (t79713, t79729, t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855)
}
