//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta869 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3182;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3183;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3184;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3185;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3186;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3187;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3188;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3190;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3191;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3192;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta869<F: Float>(t15495: F, t4997: F, t15492: F, t5019: F, t15591: F, t5002: F, t1174: F, t18237: F, t3431: F, t6187: F, t698: F, t1227: F, t13969: F, t18341: F, t18345: F, t1177: F, t18943: F, t3536: F, t3555: F, t52872: F, t52875: F, t55723: F, t63294: F, t63298: F, t63302: F, t974: F, t18589: F, t15743: F, t5005: F, t6177: F, t11709: F, t15455: F, t15459: F, t15463: F, t15525: F, t15535: F, t15569: F, t15612: F, t15631: F, t15650: F, t1653: F, t18321: F, t19058: F, t3552: F, t3557: F, t3560: F, t3577: F, t3578: F, t5024: F, t52906: F, t53083: F, t53087: F, t11692: F, t11697: F, t18964: F, t18583: F, t11678: F, t18367: F, t18593: F, t15640: F, t15737: F, t11668: F, t15478: F, t15659: F, t18395: F, t18946: F, t19000: F, t45114: F, t45128: F, t4723: F, t52893: F, t52897: F, t52908: F, t52917: F, t52926: F, t52932: F, t53176: F, t65014: F, t65452: F, t15503: F, t19025: F, t3535: F, t1202: F, t19032: F, t15498: F, t4993: F, t15486: F, t1090: F, t1218: F, t1232: F, t15654: F, t15708: F, t18205: F, t18941: F, t3243: F, t3447: F, t3494: F, t4582: F, t4729: F, t4987: F, t5012: F, t52935: F, t52942: F, t53249: F, t55716: F, t5971: F, t61798: F, t61910: F, t6225: F, t15590: F, t5018: F, t15507: F, t15548: F, t15438: F, t15531: F, t15555: F, t15622: F, t15627: F, t18307: F, t18346: F, t3490: F, t44858: F, t44953: F, t4980: F, t52810: F, t52836: F, t52952: F, t52973: F, t52975: F, t52987: F, t53336: F, t11719: F, t11728: F, t11738: F, t15545: F, t15620: F, t15625: F, t15656: F, t18303: F, t19056: F, t3248: F, t3506: F, t3509: F, t3516: F, t44896: F, t44968: F, t44972: F, t44976: F, t52991: F, t52993: F, t52999: F, t53001: F, t6219: F, t19057: F, t11546: F, t11665: F, t15434: F, t18360: F, t18584: F, t44996: F, t45002: F, t4889: F, t4984: F, t52601: F, t52813: F, t53023: F, t53026: F, t53033: F, t53238: F, t61855: F, t6192: F, t6230: F, t63415: F, t15608: F, t15689: F, t135: F, t18996: F, t18969: F, t3440: F, t45197: F, t52704: F, t53064: F, t53067: F, t53079: F, t53093: F, t53096: F, t53099: F, t53102: F, t63315: F, t1089: F, t5011: F, t607: F, t1215: F, t14749: F, t15661: F, t15663: F, t15700: F, t15701: F, t15704: F, t1735: F, t18401: F, t18959: F, t3966: F, t45020: F, t4733: F, t4972: F, t52628: F, t52903: F, t53114: F, t53116: F, t53118: F, t55666: F, t18363: F, t45124: F, t18359: F, t15740: F, t18368: F, t3562: F, t45044: F, t45049: F, t45162: F, t53135: F, t53142: F, t53155: F, t53158: F, t53161: F, t53185: F, t53472: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t65992, t65994, t65996, t65998, t66001, t66015, t66024) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3182::<F>(t15495, t4997, t15492, t5019, t15591, t5002, t1174, t18237, t3431, t6187, t698, t1227, t13969, t18341);
        let t66029 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3183::<F>(t1227, t13969, t18345, t1174, t1177, t18943, t3536, t3555, t52872, t52875, t55723, t63294, t63298, t63302, t65992, t65994, t65996, t65998, t66001, t66015, t66024, t974);
        let t66067 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3184::<F>(t1227, t13969, t18589, t15743, t5005, t1174, t6177, t698, t11709, t15455, t15459, t15463, t15525, t15535, t15569, t15612, t15631, t15650, t1653, t18321, t19058, t3552, t3557, t3560, t3577, t3578, t5024, t52906, t53083, t53087, t55723, t974);
        let (t66073, t66076, t66079, t66084, t66092) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3185::<F>(t11692, t11697, t18964, t18583, t3577, t11678, t18367, t1227, t13969, t18593, t15640, t15737);
        let t66111 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3186::<F>(t11668, t11678, t11692, t15478, t15569, t15659, t18395, t18946, t19000, t3577, t3578, t45114, t45128, t4723, t52893, t52897, t52908, t52917, t52926, t52932, t53176, t65014, t65452, t66073, t66076, t66079, t66084, t66092);
        let t66157 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3187::<F>(t15503, t15640, t19025, t3535, t1202, t19032, t15498, t4993, t15486, t5024, t1090, t11668, t11678, t1218, t1227, t1232, t15654, t15708, t18205, t18941, t3243, t3447, t3494, t3577, t3578, t45128, t4582, t4729, t4987, t5012, t52935, t52942, t53249, t55716, t5971, t61798, t61910, t6225);
        let t66185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3188::<F>(t15590, t5018, t15507, t15548, t1218, t15438, t15503, t15531, t15535, t15555, t15622, t15627, t18307, t18346, t3490, t44858, t44953, t4980, t52810, t52836, t52952, t52973, t52975, t52987, t53336);
        let t66219 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189::<F>(t11719, t11728, t11738, t15545, t15620, t15625, t15656, t18303, t19056, t3248, t3506, t3509, t3516, t3577, t3578, t44896, t44968, t44972, t44976, t4582, t5024, t52991, t52993, t52999, t53001, t6219);
        let t66254 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3190::<F>(t13969, t19057, t3506, t11546, t11665, t11668, t11692, t1174, t1227, t15434, t15622, t15627, t15737, t18360, t18584, t3243, t44996, t45002, t4582, t4889, t4984, t52601, t52813, t53023, t53026, t53033, t53238, t61855, t6192, t6230, t63415);
        let t66282 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3191::<F>(t15438, t15548, t15569, t15608, t15689, t4889, t1174, t135, t18996, t11665, t15650, t18969, t3440, t45197, t5005, t52704, t52897, t53064, t53067, t53079, t53093, t53096, t53099, t53102, t53176, t63315);
        let (t66310, t66326) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3192::<F>(t1089, t5011, t607, t15743, t5024, t11665, t11678, t11692, t1215, t1227, t14749, t15659, t15661, t15663, t15700, t15701, t15704, t1735, t18401, t18959, t3490, t3577, t3578, t3966, t45020, t45114, t4582, t4733, t4972, t52628, t52903, t53114, t53116, t53118, t55666, t6225);
        let t66353 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193::<F>(t18363, t3577, t45124, t11697, t18359, t15459, t15463, t15478, t15631, t15740, t18321, t18368, t3562, t45044, t45049, t45162, t53135, t53142, t53155, t53158, t53161, t53185, t53472);
    (t66029, t66067, t66111, t66157, t66185, t66219, t66254, t66282, t66310, t66326, t66353)
}
