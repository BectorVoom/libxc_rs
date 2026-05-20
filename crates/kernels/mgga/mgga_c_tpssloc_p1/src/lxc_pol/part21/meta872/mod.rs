//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta872 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3212;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3213;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3214;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3215;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3216;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3217;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3218;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3219;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta872<F: Float>(t63731: F, t63733: F, t63735: F, t63737: F, t63739: F, t63741: F, t63743: F, t63745: F, t63747: F, t63752: F, t63754: F, t63757: F, t63759: F, t63763: F, t19270: F, t3633: F, t4700: F, t63765: F, t63767: F, t63769: F, t63771: F, t63829: F, t64100: F, t64253: F, t64259: F, t64433: F, t65290: F, t65293: F, t65296: F, t65299: F, t5091: F, t11947: F, t6270: F, t193: F, t336: F, t3637: F, t3640: F, t64436: F, t64441: F, t65301: F, t65305: F, t65307: F, t65309: F, t65312: F, t65314: F, t65319: F, t65321: F, t65324: F, t65326: F, t28: F, t265: F, t504: F, t59618: F, t64473: F, t64510: F, t64534: F, t64545: F, t66885: F, t1081: F, t1260: F, t12606: F, t13493: F, t1409: F, t15844: F, t1649: F, t16558: F, t17133: F, t1768: F, t18196: F, t19276: F, t2250: F, t2756: F, t3231: F, t3644: F, t3966: F, t47676: F, t506: F, t5099: F, t52: F, t5398: F, t55677: F, t5669: F, t59627: F, t59629: F, t59631: F, t5966: F, t607: F, t6279: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t113: F, t12545: F, t1271: F, t12816: F, t1393: F, t1458: F, t15857: F, t16503: F, t1778: F, t1849: F, t19289: F, t19537: F, t20098: F, t20136: F, t2312: F, t2314: F, t3652: F, t3660: F, t3929: F, t4028: F, t4034: F, t510: F, t513: F, t5450: F, t55568: F, t55927: F, t56110: F, t56124: F, t56148: F, t56161: F, t56174: F, t56192: F, t56212: F, t56294: F, t56364: F, t56370: F, t56389: F, t56408: F, t57801: F, t57810: F, t57815: F, t57822: F, t6287: F, t6295: F, t63261: F, t6468: F, t650: F, t652: F, t55998: F, t56034: F, t56075: F, t1395: F, t671: F, t112: F, t20148: F, t12524: F, t12813: F, t1401: F, t16521: F, t16524: F, t16538: F, t16541: F, t19534: F, t20162: F, t20173: F, t20176: F, t2363: F, t3938: F, t3941: F, t4072: F, t5456: F, t55571: F, t577: F, t1851: F, t5381: F, t580: F, t20186: F, t576: F, t6483: F, t1404: F, t6470: F, t1858: F, t5363: F, t16507: F, t3: F, t5364: F, t55368: F, t55374: F, t55376: F, t55378: F, t1396: F, t1398: F, t16546: F, t1852: F, t20149: F, t3932: F, t3946: F, t45584: F, t45588: F, t55417: F, t6471: F) -> F {
        let t66886 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3212::<F>(t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757, t63759, t63763);
        let t66891 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3213::<F>(t19270, t3633, t4700, t63765, t63767, t63769, t63771, t63829, t64100, t64253, t64259, t64433, t65290, t65293, t65296, t65299);
        let t66901 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3214::<F>(t5091, t11947, t6270, t193, t336, t3637, t3640, t4700, t64436, t64441, t65301, t65305, t65307, t65309, t65312, t65314, t65319, t65321, t65324, t65326);
        let t66921 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3215::<F>(t28, t265, t504, t59618, t64473, t64510, t64534, t64545, t66885, t66886, t66891, t66901, t1081, t1260, t12606, t13493, t1409, t15844, t1649, t16558, t17133, t1768, t18196, t19276, t2250, t2756, t3231, t3644, t3966, t47676, t506, t5099, t52, t5398, t55677, t5669, t59627, t59629, t59631, t5966, t607, t6279, t873, dens_threshold, rho1, zeta_threshold);
        let t66935 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3216::<F>(t113, t12545, t1271, t12816, t1393, t1458, t15857, t16503, t1778, t1849, t19289, t19537, t20098, t20136, t2312, t2314, t3652, t3660, t3929, t4028, t4034, t510, t513, t5450, t55568, t55927, t56110, t56124, t56148, t56161, t56174, t56192, t56212, t56294, t56364, t56370, t56389, t56408, t57801, t57810, t57815, t57822, t6287, t6295, t63261, t6468, t650, t652, t66921);
        let (t66937, t66961) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3217::<F>(t55998, t56034, t56075, t66935, t1395, t671, t112, t20148, t12524, t12813, t1401, t1458, t16521, t16524, t16538, t16541, t19534, t20162, t20173, t20176, t2363, t3938, t3941, t4072, t5456, t55568, t55571, t577);
        let (t66964, t66967, t66976, t66987, t66989, t66991) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3218::<F>(t1851, t5381, t20148, t580, t20186, t576, t1395, t6483, t1404, t6470, t1858, t5363);
        let t66993 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3219::<F>(t16507, t1858, t3, t5364, t5381, t55368, t55374, t55376, t55378, t580, t66937, t66976, t66987, t66989, t66991);
        let tv4rho42 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3220::<F>(t1396, t1398, t1404, t16546, t1852, t20149, t20186, t3932, t3946, t45584, t45588, t55417, t6471, t6483, t66961, t66964, t66967, t66993);
    tv4rho42
}
