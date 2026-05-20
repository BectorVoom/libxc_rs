//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta543 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2033;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2034;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2035;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2036;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2037;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta543<F: Float>(t1284: F, t17: F, t9861: F, t1287: F, t9212: F, t1285: F, t9218: F, t118: F, t142: F, t39283: F, t2223: F, t3824: F, t2475: F, t2461: F, t2478: F, t159: F, t172: F, t2454: F, t268: F, t39249: F, t39256: F, t39300: F, t39309: F, t39312: F, t39316: F, t39320: F, t39377: F, t39378: F, t39381: F, t39535: F, t676: F, t724: F, t732: F, t739: F, t740: F, t746: F, t747: F, t781: F, t9493: F, t9720: F, t9738: F, t9740: F, t9752: F, t9762: F, t9763: F, t9781: F, t9828: F, t204: F, t2368: F, t2459: F, t2462: F, t2471: F, t2472: F, t2476: F, t2480: F, t2490: F, t2494: F, t2495: F, t2505: F, t2509: F, t2513: F, t39373: F, t39389: F, t39397: F, t39400: F, t39408: F, t39411: F, t9489: F, t9729: F, t9734: F, t9739: F, t9755: F, t9759: F, t9766: F, t9803: F, t9810: F, t9814: F, t168: F, t2458: F, t2479: F, t2504: F, t2510: F, t2512: F, t39273: F, t39275: F, t39278: F, t39281: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t690: F, t725: F, t730: F, t731: F, t9730: F, t9733: F, t9758: F, t9892: F, t9905: F, t181: F, t2369: F, t2460: F, t2477: F, t39263: F, t39529: F, t39549: F, t39563: F, t39585: F, t39590: F, t39593: F, t745: F, t9711: F, t9751: F, t9843: F, t521: F, t2225: F, t3826: F, t12129: F, t592: F, t9216: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39620, t39634, t39636, t39655, t39658) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2033::<F>(t1284, t17, t9861, t1287, t9212, t1285, t9218, t118, t142, t39283);
        let (t39659, t39664, t39706) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2034::<F>(t2223, t3824, t2475, t2461, t2478, t159, t172, t2454, t268, t39249, t39256, t39300, t39309, t39312, t39316, t39320, t39377, t39378, t39381, t39535, t676, t724, t732, t739, t740, t746, t747, t781, t9493, t9720, t9738, t9740, t9752, t9762, t9763, t9781, t9828);
        let t39749 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2035::<F>(t204, t2368, t2459, t2462, t2471, t2472, t2476, t2480, t2490, t2494, t2495, t2505, t2509, t2513, t268, t39373, t39389, t39397, t39400, t39408, t39411, t676, t746, t9489, t9729, t9734, t9739, t9755, t9759, t9766, t9803, t9810, t9814);
        let t39803 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2036::<F>(t118, t159, t168, t2458, t2459, t2461, t2471, t2472, t2475, t2476, t2479, t2495, t2504, t2510, t2512, t39273, t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39378, t39389, t39463, t39468, t39472, t39476, t39483, t39664, t690, t725, t730, t731, t9730, t9733, t9739, t9758, t9892, t9905);
        let t39840 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2037::<F>(t2471, t118, t181, t2369, t2460, t2462, t2477, t2479, t2494, t2510, t2512, t39263, t39283, t39529, t39549, t39563, t39585, t39590, t39593, t39658, t39664, t730, t731, t745, t747, t9711, t9730, t9751, t9752, t9758, t9762, t9843);
        let (t39842, t39844, t39845, t39851, t39855) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2038::<F>(t39706, t39749, t39803, t39840, t17, t521, t2225, t3826, t12129, t592, t1287, t9216);
    (t39620, t39634, t39636, t39655, t39658, t39659, t39842, t39844, t39845, t39851, t39855)
}
