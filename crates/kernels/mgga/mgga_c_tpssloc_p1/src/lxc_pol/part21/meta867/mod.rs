//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta867 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3164;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3165;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3166;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3167;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3168;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3169;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3170;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3171;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3172;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3173;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3174;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta867<F: Float>(t3508: F, t6218: F, t1215: F, t11721: F, t6224: F, t15594: F, t4993: F, t11692: F, t11697: F, t18396: F, t18400: F, t3577: F, t1653: F, t3507: F, t11678: F, t19001: F, t11825: F, t14726: F, t15659: F, t15661: F, t15702: F, t1735: F, t18395: F, t19083: F, t19101: F, t3490: F, t3493: F, t3578: F, t3587: F, t45114: F, t45128: F, t45197: F, t52704: F, t53149: F, t6207: F, t11818: F, t1213: F, t248: F, t6219: F, t3036: F, t6163: F, t3500: F, t3503: F, t1210: F, t15734: F, t5005: F, t19047: F, t3572: F, t3506: F, t6225: F, t1174: F, t1214: F, t1227: F, t1230: F, t15672: F, t15761: F, t1737: F, t19026: F, t19051: F, t3440: F, t3496: F, t3511: F, t3515: F, t3518: F, t475: F, t4889: F, t5024: F, t52568: F, t6211: F, t63311: F, t63353: F, t65264: F, t11539: F, t18211: F, t3540: F, t6170: F, t19015: F, t45124: F, t6158: F, t15730: F, t5002: F, t1226: F, t18573: F, t11546: F, t1232: F, t14744: F, t14753: F, t15569: F, t15710: F, t15764: F, t1743: F, t3447: F, t3566: F, t45119: F, t488: F, t52696: F, t52995: F, t53187: F, t55716: F, t6164: F, t63372: F, t18392: F, t18241: F, t3521: F, t19040: F, t6230: F, t15578: F, t11789: F, t5979: F, t3523: F, t19080: F, t1177: F, t15581: F, t15584: F, t15587: F, t6203: F, t63406: F, t65330: F, t11709: F, t18356: F, t18975: F, t6165: F, t19033: F, t11734: F, t19095: F, t15486: F, t1222: F, t18574: F, t15527: F, t1748: F, t3527: F, t3531: F, t5019: F, t53487: F, t63390: F, t5975: F, t18321: F, t3548: F, t15437: F, t15502: F, t15506: F, t4965: F, t5023: F, t15498: F, t44811: F, t4974: F, t52575: F, t52580: F, t52583: F, t52586: F, t52599: F, t11668: F, t14706: F, t15470: F, t15474: F, t15560: F, t15564: F, t15615: F, t15681: F, t15740: F, t3516: F, t4582: F, t4972: F, t5030: F, t50992: F, t51002: F, t52609: F, t52619: F, t52766: F, t52879: F, t55662: F, t5971: F, t61855: F, t61910: F, t62044: F) -> (F, F, F, F, F, F, F) {
        let (t65464, t65469, t65474, t65479, t65482, t65485) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3164::<F>(t3508, t6218, t1215, t11721, t6224, t15594, t4993, t11692, t11697, t18396, t18400, t3577);
        let t65518 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3165::<F>(t1653, t3507, t11678, t11697, t19001, t11692, t11825, t14726, t15659, t15661, t15702, t1735, t18395, t19083, t19101, t3490, t3493, t3577, t3578, t3587, t45114, t45128, t45197, t52704, t53149, t6207, t65464, t65469, t65474, t65479, t65482, t65485);
        let (t65528, t65541, t65545, t65552, t65554) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3166::<F>(t11818, t1213, t248, t6219, t3036, t6163, t3500, t3503, t1210, t15734, t5005, t19047, t3572);
        let t65565 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3167::<F>(t11818, t248, t3506, t6225, t1174, t11825, t1214, t1227, t1230, t15672, t15761, t1737, t19026, t19051, t3440, t3496, t3511, t3515, t3518, t3587, t475, t4889, t5024, t52568, t6211, t63311, t63353, t65264, t65528, t65541, t65545, t65552, t65554);
        let (t65567, t65581, t65598, t65600, t65605, t65607) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3168::<F>(t11539, t1174, t18211, t3540, t6170, t19015, t3577, t45124, t6158, t15730, t5002, t1226, t18573);
        let t65610 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3169::<F>(t11546, t1174, t1232, t14744, t14753, t15569, t15710, t15764, t1735, t1743, t18395, t3447, t3566, t3577, t3578, t45119, t488, t52696, t52995, t53187, t55716, t6164, t63372, t65567, t65581, t65598, t65600, t65605, t65607);
        let (t65613, t65617, t65619, t65628, t65632) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3170::<F>(t18392, t3490, t1227, t18241, t248, t3521, t19040, t15734, t5024, t11818, t3515, t6230);
        let t65653 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3171::<F>(t15578, t4889, t11789, t1227, t248, t5979, t19051, t3523, t19080, t3572, t1174, t1177, t11825, t1213, t1214, t15581, t15584, t15587, t475, t6203, t63406, t65330, t65613, t65617, t65619, t65628, t65632);
        let (t65660, t65662, t65664, t65668, t65670, t65672, t65674) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3172::<F>(t11709, t18356, t18975, t3490, t3540, t6165, t19083, t3523, t19026, t3572, t19033, t11734, t19095);
        let t65685 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3173::<F>(t15486, t5005, t1222, t18574, t1174, t15527, t1748, t19033, t3440, t3527, t3531, t3587, t5019, t53487, t63390, t65660, t65662, t65664, t65668, t65670, t65672, t65674);
        let t65716 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3174::<F>(t11789, t1227, t248, t5975, t18321, t3548, t15437, t15502, t15506, t4965, t5023, t1232, t15498, t15594, t19083, t3511, t3518, t3527, t3531, t44811, t4974, t52575, t52580, t52583, t52586, t52599);
        let t65764 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3175::<F>(t11668, t11692, t1214, t1227, t14706, t15470, t15474, t15560, t15564, t15594, t15615, t15681, t15740, t1735, t248, t3506, t3508, t3516, t3577, t3578, t4582, t4889, t4972, t5030, t50992, t51002, t52609, t52619, t52766, t52879, t55662, t5971, t61855, t61910, t62044, t65264);
    (t65518, t65565, t65610, t65653, t65685, t65716, t65764)
}
