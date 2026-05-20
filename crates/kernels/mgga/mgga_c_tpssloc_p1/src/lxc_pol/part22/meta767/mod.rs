//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta767 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2592;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2594;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2595;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2596;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2597;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2598;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2599;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2600;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta767<F: Float>(t1227: F, t21776: F, t248: F, t3521: F, t18392: F, t5005: F, t15737: F, t18356: F, t19040: F, t5024: F, t11738: F, t22299: F, t3570: F, t11728: F, t22312: F, t1174: F, t1177: F, t15495: F, t6221: F, t65552: F, t65554: F, t65558: F, t65567: F, t71189: F, t71201: F, t19033: F, t4993: F, t19046: F, t5018: F, t5023: F, t6169: F, t11546: F, t1218: F, t1230: F, t1232: F, t15498: F, t1737: F, t1748: F, t19026: F, t19087: F, t22214: F, t22218: F, t3490: F, t4889: F, t5014: F, t5030: F, t6211: F, t66147: F, t66150: F, t71148: F, t71158: F, t18321: F, t5040: F, t19002: F, t19005: F, t19047: F, t19051: F, t52628: F, t65581: F, t65598: F, t65600: F, t65605: F, t65607: F, t65613: F, t71168: F, t71177: F, t1009: F, t22113: F, t1011: F, t1212: F, t18375: F, t5002: F, t18943: F, t19080: F, t65617: F, t65619: F, t65628: F, t65632: F, t65637: F, t65647: F, t65649: F, t65651: F, t66159: F, t1730: F, t19032: F, t1017: F, t1207: F, t1210: F, t22173: F, t372: F, t471: F, t479: F, t15507: F, t19095: F, t65660: F, t65662: F, t65664: F, t65668: F, t65670: F, t65672: F, t65674: F, t65676: F, t65681: F, t22298: F, t486: F, t11668: F, t11678: F, t11692: F, t15659: F, t1735: F, t18232: F, t19000: F, t3577: F, t3578: F, t45037: F, t45114: F, t45197: F, t4582: F, t4724: F, t4729: F, t4974: F, t4978: F, t4984: F, t6225: F, t6230: F, t65464: F, t65474: F, t65545: F, t65689: F, t65691: F, t72146: F, t13969: F, t22270: F, t3506: F, t15591: F, t18301: F, t18594: F, t18955: F, t18959: F, t22280: F, t45030: F, t45162: F, t4733: F, t5012: F, t52600: F, t52601: F, t52610: F, t5975: F, t70330: F, t22257: F, t21769: F, t22157: F, t45124: F, t11709: F, t1216: F, t18303: F, t18307: F, t22246: F, t22271: F, t3536: F, t5019: F, t52810: F, t53238: F, t53472: F, t5971: F, t6227: F, t66533: F, t11697: F, t22287: F, t15569: F, t18371: F, t1090: F, t18364: F, t18948: F, t22244: F, t4950: F, t4954: F, t4989: F, t52680: F, t53083: F, t53336: F, t65803: F, t66622: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t72273, t72285, t72287, t72289, t72293) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2592::<F>(t1227, t21776, t248, t3521, t18392, t5005, t15737, t18356, t19040, t5024, t11738, t22299, t3570);
        let t72299 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593::<F>(t11728, t22312, t248, t3570, t1174, t1177, t15495, t6221, t65552, t65554, t65558, t65567, t71189, t71201, t72273, t72285, t72287, t72289, t72293);
        let t72333 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2594::<F>(t19033, t4993, t19046, t5018, t5023, t6169, t11546, t1174, t1218, t1227, t1230, t1232, t15498, t1737, t1748, t19026, t19087, t22214, t22218, t248, t3490, t4889, t5014, t5030, t6211, t66147, t66150, t71148, t71158);
        let t72357 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2595::<F>(t18321, t5040, t1174, t1177, t1748, t19002, t19005, t19047, t19051, t4889, t5014, t5030, t52628, t65581, t65598, t65600, t65605, t65607, t65613, t71168, t71177);
        let (t72361, t72380) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2596::<F>(t1009, t22113, t1011, t1212, t18375, t5002, t1218, t1737, t18943, t19080, t5014, t65617, t65619, t65628, t65632, t65637, t65647, t65649, t65651, t66159);
        let t72405 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2597::<F>(t1730, t19032, t1017, t1207, t1210, t22173, t372, t471, t479, t15507, t19095, t1218, t1232, t65660, t65662, t65664, t65668, t65670, t65672, t65674, t65676, t65681);
        let (t72445, t72452) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2598::<F>(t22298, t486, t11668, t11678, t11692, t15659, t1735, t18232, t19000, t19033, t3577, t3578, t45037, t45114, t45197, t4582, t4724, t4729, t4974, t4978, t4984, t6225, t6230, t65464, t65474, t65545, t65689, t65691, t72146);
        let t72484 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2599::<F>(t13969, t22270, t3506, t11678, t1227, t15591, t18301, t18594, t18955, t18959, t19051, t22280, t3577, t3578, t45030, t45162, t4582, t4733, t4974, t5005, t5012, t5024, t52600, t52601, t52610, t5975, t6221, t6225, t70330, t72445);
        let t72522 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2600::<F>(t1227, t13969, t22257, t21769, t248, t3521, t22157, t3577, t45124, t11668, t11709, t1216, t15659, t18303, t18307, t18943, t18959, t21776, t22246, t22271, t3506, t3536, t3578, t4582, t5005, t5012, t5019, t52810, t53238, t53472, t5971, t6227, t66533);
        let t72552 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2601::<F>(t11697, t22287, t3577, t15569, t18371, t1090, t1216, t15737, t18303, t18307, t18364, t18948, t19051, t21769, t22244, t3578, t4950, t4954, t4989, t52680, t53083, t53336, t65803, t66622);
    (t72299, t72333, t72357, t72361, t72380, t72405, t72445, t72452, t72484, t72522, t72552)
}
