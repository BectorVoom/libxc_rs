//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta738 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2422;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2423;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2424;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2425;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2426;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2427;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2428;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2429;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2430;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2431;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2432;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta738<F: Float>(t17517: F, t49226: F, t21347: F, t942: F, t10765: F, t14266: F, t1569: F, t17428: F, t21259: F, t4434: F, t49427: F, t5743: F, t5759: F, t59962: F, t68762: F, t68764: F, t68767: F, t68769: F, t68771: F, t68773: F, t68775: F, t68883: F, t68885: F, t952: F, t48103: F, t49304: F, t49306: F, t49317: F, t49322: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t41684: F, t41863: F, t68460: F, t68464: F, t68468: F, t68472: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68500: F, t68502: F, t68504: F, t68506: F, t68509: F, t68511: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F, t49378: F, t59657: F, t60163: F, t60168: F, t60173: F, t68536: F, t68541: F, t68545: F, t68549: F, t68552: F, t68556: F, t68563: F, t49379: F, t60192: F, t60194: F, t60202: F, t68571: F, t68577: F, t68580: F, t68583: F, t68586: F, t68589: F, t68592: F, t42212: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t60204: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F, t42213: F, t47787: F, t59700: F, t59702: F, t59704: F, t60274: F, t68619: F, t68626: F, t68628: F, t68630: F, t68633: F, t68635: F, t59759: F, t59761: F, t60308: F, t60310: F, t60312: F, t68638: F, t68640: F, t68643: F, t68646: F, t68649: F, t68695: F, t68697: F, t10756: F, t10825: F, t14332: F, t14369: F, t1581: F, t17350: F, t17355: F, t21115: F, t21195: F, t21198: F, t21247: F, t2856: F, t41984: F, t42149: F, t4411: F, t4472: F, t48789: F, t49096: F, t5762: F, t5775: F, t5790: F, t5791: F, t60338: F, t68758: F, t68926: F, t68995: F, t924: F, t932: F, t950: F, t21360: F, t923: F, t10828: F, t14263: F, t14337: F, t1568: F, t17443: F, t17446: F, t17451: F, t17499: F, t17547: F, t21089: F, t21207: F, t21242: F, t21306: F, t2886: F, t2930: F, t41826: F, t42111: F, t42113: F, t4433: F, t4471: F, t49099: F, t60775: F, t69003: F, t69005: F, t933: F, t10740: F, t10747: F, t10771: F, t1580: F, t17297: F, t17349: F, t17454: F, t17493: F, t17544: F, t21309: F, t21312: F, t21321: F, t2861: F, t2905: F, t4438: F, t48783: F, t49263: F, t49422: F, t5758: F, t59895: F, t69011: F, t69018: F, t4359: F, t60357: F, t4400: F, t59959: F, t13727: F, t13520: F, t17521: F, t17524: F, t17528: F, t49274: F, t21238: F, t2932: F, t17496: F, t17500: F, t21239: F, t4454: F, t4476: F, t49104: F, t5794: F, t60343: F, t60424: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69036, t69050) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2422::<F>(t17517, t49226, t21347, t942, t10765, t14266, t1569, t17428, t21259, t4434, t49427, t5743, t5759, t59962, t68762, t68764, t68767, t68769, t68771, t68773, t68775, t68883, t68885, t952);
        let (t69066, t69079) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2423::<F>(t48103, t49304, t49306, t49317, t49322, t68442, t68444, t68446, t68448, t68452, t68454, t41684, t41863, t68460, t68464, t68468, t68472, t68479, t68483, t68486, t68489, t68492, t68494);
        let t69093 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2424::<F>(t68498, t68500, t68502, t68504, t68506, t68509, t68511, t68515, t68518, t68523, t68527, t68530);
        let t69105 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2425::<F>(t49378, t59657, t60163, t60168, t60173, t68536, t68541, t68545, t68549, t68552, t68556, t68563);
        let (t69118, t69130) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2426::<F>(t49379, t60192, t60194, t60202, t68571, t68577, t68580, t68583, t68586, t68589, t68592, t42212, t59663, t59665, t59680, t59688, t59694, t60204, t68596, t68599, t68602, t68605, t68608);
        let t69143 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2427::<F>(t42213, t47787, t59700, t59702, t59704, t60274, t68619, t68626, t68628, t68630, t68633, t68635);
        let t69156 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2428::<F>(t59759, t59761, t60308, t60310, t60312, t68638, t68640, t68643, t68646, t68649, t68695, t68697);
        let t69180 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2429::<F>(t10756, t10825, t14332, t14369, t1581, t17350, t17355, t21115, t21195, t21198, t21247, t2856, t41984, t42149, t4411, t4472, t48789, t49096, t5762, t5775, t5790, t5791, t60338, t68758, t68926, t68995, t69066, t69079, t69093, t69105, t69118, t69130, t69143, t69156, t924, t932, t950);
        let t69218 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2430::<F>(t21360, t923, t10756, t10765, t10828, t14263, t14337, t1568, t17443, t17446, t17451, t17499, t17547, t21089, t21207, t21242, t21247, t21306, t2886, t2930, t41826, t42111, t42113, t4433, t4471, t49099, t5775, t60775, t69003, t69005, t933, t950);
        let t69249 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2431::<F>(t10740, t10747, t10771, t10825, t1569, t1580, t1581, t17297, t17349, t17454, t17493, t17544, t21309, t21312, t21321, t2861, t2905, t2930, t4434, t4438, t4472, t48783, t49263, t49422, t5758, t5790, t59895, t69011, t69018, t69036);
        let (t69253, t69255, t69257, t69259, t69261, t69263, t69276) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2432::<F>(t4359, t60357, t4400, t59959, t13727, t17517, t13520, t17521, t17524, t17528, t49274, t21238, t2932);
        let t69286 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2433::<F>(t10828, t14263, t14337, t17454, t17493, t17496, t17500, t21239, t2905, t2930, t4454, t4471, t4476, t49104, t5794, t60343, t60424, t69253, t69255, t69257, t69259, t69261, t69263, t69276, t950);
    (t69036, t69050, t69180, t69218, t69249, t69253, t69255, t69257, t69259, t69261, t69263, t69286)
}
