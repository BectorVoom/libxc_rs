//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta702 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2273;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2274;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2275;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2276;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2277;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2278;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2279;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2280;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2281;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2282;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2283;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta702<F: Float>(t23384: F, t28681: F, t1054: F, t5943: F, t1921: F, t5914: F, t6688: F, t225: F, t28505: F, t28496: F, t1066: F, t17582: F, t18165: F, t23346: F, t25406: F, t25732: F, t25757: F, t25758: F, t25826: F, t28697: F, t28713: F, t3026: F, t4557: F, t6687: F, t6691: F, t6704: F, t6705: F, t82436: F, t986: F, t28488: F, t10164: F, t14545: F, t14555: F, t1599: F, t17575: F, t17588: F, t23365: F, t23588: F, t25801: F, t25810: F, t28485: F, t28495: F, t3169: F, t387: F, t4540: F, t4664: F, t5838: F, t6776: F, t7600: F, t7624: F, t7625: F, t88731: F, t88753: F, t28557: F, t381: F, t3173: F, t5919: F, t28702: F, t82431: F, t1052: F, t1409: F, t1626: F, t1634: F, t17686: F, t23327: F, t23329: F, t23330: F, t23336: F, t23369: F, t254: F, t25429: F, t25731: F, t25759: F, t28475: F, t28499: F, t3174: F, t3966: F, t4693: F, t5944: F, t6680: F, t88035: F, t88758: F, t28510: F, t28565: F, t1065: F, t14552: F, t1635: F, t17635: F, t25423: F, t25784: F, t28470: F, t4542: F, t5398: F, t5920: F, t6816: F, t83281: F, t88145: F, t884: F, t10165: F, t17691: F, t23581: F, t25430: F, t25743: F, t25755: F, t28515: F, t4665: F, t6815: F, t7553: F, t88022: F, t88023: F, t88812: F, t88845: F, t88868: F, t88932: F, t28516: F, t25749: F, t7560: F, t28594: F, t17583: F, t18047: F, t18061: F, t1920: F, t1956: F, t25420: F, t345: F, t4660: F, t5844: F, t61621: F, t6699: F, t6771: F, t88882: F, t89620: F, t28519: F, t25453: F, t25778: F, t28593: F, t28679: F, t388: F, t82411: F, t83344: F, t88889: F, t88915: F, t990: F, t99099: F, t17667: F, t23537: F, t1622: F, t17925: F, t17962: F, t23529: F, t5861: F, t5875: F, t5880: F, t6755: F, t82848: F, t82851: F, t82956: F, t83043: F, t83061: F, t83215: F, t88249: F, t88584: F, t25577: F, t4630: F, t25580: F, t4571: F, t17906: F, t6765: F, t17884: F, t17655: F, t23541: F, t1618: F, t17972: F, t23433: F, t4575: F, t5869: F, t5900: F, t82875: F, t88251: F, t88513: F, t88591: F, t17632: F, t17637: F, t17643: F, t17688: F, t17718: F, t17976: F, t17980: F, t4585: F, t4590: F, t82885: F, t83065: F, t88281: F, t18029: F, t6754: F, t1025: F, t17693: F, t17697: F, t17734: F, t23544: F, t4636: F, t4652: F, t82914: F, t88277: F, t88305: F, t88307: F, t88388: F, t17673: F, t17984: F, t25589: F, t4596: F, t4600: F, t7578: F, t83054: F, t83058: F, t88320: F, t88321: F, t88324: F, t88335: F, t88336: F, t88339: F, t88594: F, t88600: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99209, t99238) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2273::<F>(t23384, t28681, t1054, t5943, t1921, t5914, t6688, t225, t28505, t28496, t1066, t17582, t18165, t23346, t25406, t25732, t25757, t25758, t25826, t28697, t28713, t3026, t4557, t6687, t6691, t6704, t6705, t82436, t986);
        let t99271 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2274::<F>(t225, t28488, t10164, t1066, t14545, t14555, t1599, t17575, t17588, t1921, t23365, t23588, t25757, t25801, t25810, t28485, t28495, t3169, t387, t4540, t4664, t5838, t6687, t6776, t7600, t7624, t7625, t88731, t88753);
        let (t99296, t99313) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2275::<F>(t28557, t381, t3173, t5919, t1921, t28702, t82431, t1052, t1409, t1626, t1634, t17686, t23327, t23329, t23330, t23336, t23369, t254, t25429, t25731, t25759, t28475, t28499, t28713, t3169, t3174, t3966, t4693, t5944, t6680, t6687, t6691, t88035, t88758, t986);
        let t99353 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2276::<F>(t23384, t28510, t28565, t381, t1065, t14552, t1635, t17588, t17635, t23327, t23329, t23330, t23346, t23369, t25423, t25784, t28470, t28697, t3169, t4542, t5398, t5920, t6687, t6691, t6816, t7600, t83281, t88145, t884, t99209, t99296);
        let t99390 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2277::<F>(t10165, t1052, t1599, t17575, t17635, t17686, t17691, t23327, t23329, t23336, t23581, t25429, t25430, t25743, t25755, t28515, t4557, t4665, t5919, t6687, t6815, t6816, t7553, t88022, t88023, t88812, t88845, t88868, t88932);
        let t99422 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2278::<F>(t23384, t28470, t28516, t25749, t7560, t225, t28594, t1066, t1635, t17583, t18047, t18061, t1920, t1956, t23346, t25420, t25757, t25758, t345, t387, t4660, t5844, t61621, t6687, t6699, t6771, t88882, t89620, t986);
        let t99450 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2279::<F>(t23384, t28519, t1052, t23329, t23346, t25429, t25453, t25778, t28510, t28593, t28679, t3026, t3174, t388, t4660, t4665, t4693, t5943, t6815, t7624, t82411, t83344, t88889, t88915, t990, t99099);
        let t99492 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2280::<F>(t17667, t23537, t1622, t17925, t17962, t23529, t5861, t5875, t5880, t6755, t82848, t82851, t82956, t83043, t83061, t83215, t88249, t88584);
        let t99514 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2281::<F>(t25577, t4630, t25580, t4571, t17906, t6765, t17884, t17655, t23541, t1618, t17972, t23433, t23529, t4575, t5869, t5900, t82875, t88251, t88513, t88591);
        let t99535 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2282::<F>(t17632, t17637, t17643, t17688, t17718, t17976, t17980, t23541, t25580, t4585, t4590, t6765, t82885, t83065, t88281);
        let t99556 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2283::<F>(t18029, t6754, t1025, t1618, t1622, t17693, t17697, t17734, t23537, t23544, t25577, t25580, t4636, t4652, t5900, t6765, t82914, t88277, t88305, t88307, t88388);
        let t99571 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2284::<F>(t17673, t17984, t25589, t4596, t4600, t7578, t83054, t83058, t88320, t88321, t88324, t88335, t88336, t88339, t88594, t88600);
    (t99238, t99271, t99313, t99353, t99390, t99422, t99450, t99492, t99514, t99535, t99556, t99571)
}
