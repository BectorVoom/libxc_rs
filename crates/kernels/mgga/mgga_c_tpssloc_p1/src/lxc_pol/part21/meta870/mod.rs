//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta870 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3194;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3195;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3196;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3197;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3198;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta870<F: Float>(t15572: F, t15740: F, t11697: F, t18382: F, t3577: F, t3575: F, t62053: F, t3624: F, t1229: F, t1734: F, t375: F, t3610: F, t27524: F, t607: F, t1215: F, t6224: F, t1227: F, t13969: F, t18954: F, t11709: F, t15617: F, t15702: F, t15708: F, t15709: F, t15750: F, t18236: F, t18397: F, t18948: F, t19002: F, t3247: F, t3508: F, t3578: F, t45112: F, t45119: F, t45134: F, t45162: F, t5005: F, t53220: F, t53246: F, t19067: F, t1222: F, t18297: F, t18982: F, t18947: F, t3506: F, t11719: F, t18302: F, t11546: F, t1174: F, t11814: F, t15625: F, t15761: F, t18300: F, t3440: F, t3507: F, t44725: F, t44863: F, t44906: F, t45030: F, t45178: F, t4582: F, t53267: F, t6221: F, t63378: F, t63386: F, t63394: F, t18225: F, t3431: F, t18221: F, t15522: F, t4889: F, t11668: F, t11678: F, t1177: F, t15686: F, t3248: F, t3252: F, t3494: F, t52893: F, t53270: F, t53272: F, t53274: F, t53287: F, t53291: F, t5979: F, t6225: F, t63368: F, t63410: F, t64990: F, t3545: F, t6109: F, t19071: F, t3515: F, t11728: F, t18306: F, t11738: F, t19076: F, t11692: F, t1196: F, t15239: F, t15453: F, t15507: F, t15531: F, t15667: F, t1735: F, t45224: F, t4954: F, t4977: F, t52615: F, t53360: F, t55677: F, t61910: F, t6230: F, t63402: F, t66310: F, t974: F, t18940: F, t486: F, t15753: F, t18375: F, t3536: F, t11734: F, t1216: F, t15594: F, t15620: F, t15637: F, t19062: F, t3243: F, t4978: F, t4989: F, t53378: F, t53387: F, t53389: F, t53397: F, t53404: F, t53410: F, t6219: F) -> (F, F, F, F, F, F, F, F) {
        let (t66360, t66363, t66372, t66374, t66378) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3194::<F>(t15572, t15740, t11697, t18382, t3577, t3575, t62053, t3624, t1229, t1734, t375, t3610);
        let (t66380, t66400) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3195::<F>(t27524, t607, t1215, t6224, t1227, t13969, t18954, t11709, t15617, t15702, t15708, t15709, t15740, t15750, t18236, t18397, t18948, t19002, t3247, t3508, t3577, t3578, t45112, t45119, t45134, t45162, t5005, t53220, t53246, t66360, t66363, t66372, t66374, t66378);
        let t66442 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3196::<F>(t1227, t13969, t19067, t1222, t18297, t18982, t18947, t3506, t11719, t18302, t11546, t1174, t11814, t15625, t15761, t18300, t3440, t3507, t44725, t44863, t44906, t45030, t45178, t4582, t5005, t53267, t6221, t63378, t63386, t63394);
        let t66480 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3197::<F>(t1174, t18225, t3431, t18221, t15522, t4889, t11668, t11678, t1177, t15686, t3248, t3252, t3440, t3494, t3577, t3578, t52893, t53270, t53272, t53274, t53287, t53291, t5979, t6225, t63368, t63410, t64990);
        let t66528 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3198::<F>(t3545, t6109, t13969, t19071, t3515, t11728, t18306, t11738, t19076, t11692, t1174, t1177, t1196, t1227, t15239, t15453, t15507, t15531, t15667, t1735, t3248, t3252, t3506, t3508, t3578, t45224, t4582, t4889, t4954, t4977, t52615, t53360, t55677, t61910, t6230, t63402, t66310, t974);
        let t66564 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3199::<F>(t18940, t486, t15753, t4889, t18375, t3536, t11668, t11728, t11734, t1216, t15507, t15594, t15620, t15637, t18300, t19062, t3243, t3506, t3515, t3577, t4582, t4978, t4989, t53378, t53387, t53389, t53397, t53404, t53410, t6219);
    (t66372, t66378, t66380, t66400, t66442, t66480, t66528, t66564)
}
