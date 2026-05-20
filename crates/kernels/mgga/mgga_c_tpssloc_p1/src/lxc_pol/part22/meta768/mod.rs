//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta768 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2602;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2603;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2604;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2606;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2607;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2608;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta768<F: Float>(t19051: F, t4993: F, t11784: F, t1227: F, t21762: F, t248: F, t11721: F, t6218: F, t11668: F, t11692: F, t15503: F, t15700: F, t1735: F, t18241: F, t19058: F, t3515: F, t3577: F, t3578: F, t45114: F, t45197: F, t4582: F, t4972: F, t4980: F, t52548: F, t52732: F, t52897: F, t5392: F, t65464: F, t65819: F, t65881: F, t65963: F, t66533: F, t70321: F, t1174: F, t135: F, t22128: F, t22132: F, t11665: F, t1216: F, t15438: F, t15507: F, t18590: F, t18955: F, t19062: F, t19072: F, t21758: F, t22158: F, t45128: F, t4984: F, t5005: F, t5024: F, t50992: F, t52759: F, t65914: F, t65920: F, t65966: F, t70330: F, t18356: F, t18975: F, t21749: F, t3431: F, t11738: F, t15569: F, t15740: F, t18225: F, t18300: F, t18321: F, t18387: F, t18969: F, t19068: F, t4950: F, t4954: F, t4969: F, t5012: F, t65541: F, t65815: F, t65935: F, t22011: F, t18375: F, t5019: F, t18946: F, t19033: F, t19056: F, t19083: F, t22208: F, t3490: F, t3506: F, t44836: F, t4989: F, t5030: F, t65884: F, t65952: F, t65992: F, t65994: F, t65996: F, t65998: F, t72445: F, t14725: F, t17635: F, t18329: F, t4889: F, t18324: F, t22136: F, t18346: F, t18580: F, t19019: F, t3440: F, t52873: F, t52893: F, t66001: F, t66015: F, t66024: F, t66027: F, t71138: F, t18371: F, t1222: F, t22175: F, t1090: F, t11728: F, t18383: F, t22312: F, t66052: F, t66054: F, t66057: F, t66073: F, t66076: F, t66079: F, t66084: F, t66092: F, t1734: F, t1089: F, t11678: F, t1215: F, t15659: F, t15701: F, t15702: F, t18237: F, t18368: F, t18395: F, t18397: F, t18401: F, t4729: F, t5046: F, t52879: F, t52903: F, t5398: F, t5979: F, t607: F, t6219: F, t65469: F, t66120: F, t70458: F, t18231: F, t3961: F, t22169: F, t11539: F, t21745: F, t11546: F, t15654: F, t18342: F, t22284: F, t45134: F, t4733: F, t4987: F, t5033: F, t52919: F, t6230: F, t70316: F, t70339: F, t71133: F, t71197: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t72577, t72593) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2602::<F>(t19051, t4993, t11784, t1227, t21762, t248, t11721, t6218, t11668, t11692, t15503, t15700, t1735, t18241, t19058, t3515, t3577, t3578, t45114, t45197, t4582, t4972, t4980, t52548, t52732, t52897, t5392, t65464, t65819, t65881, t65963, t66533, t70321);
        let t72622 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2603::<F>(t1174, t135, t22128, t22132, t11665, t1216, t1227, t15438, t15507, t18590, t18955, t19062, t19072, t21758, t22158, t3577, t45128, t4582, t4984, t5005, t5024, t50992, t52759, t65914, t65920, t65966, t70330);
        let t72654 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2604::<F>(t15503, t18356, t18975, t5024, t1174, t21749, t3431, t11738, t15569, t15740, t1735, t18225, t18300, t18321, t18387, t18969, t19068, t3577, t3578, t4582, t4950, t4954, t4969, t4980, t5012, t65541, t65815, t65935);
        let t72683 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605::<F>(t1174, t135, t22011, t18375, t5019, t1216, t18946, t19033, t19056, t19083, t22208, t3490, t3506, t44836, t4582, t4950, t4954, t4989, t5030, t65884, t65952, t65992, t65994, t65996, t65998, t72445);
        let (t72688, t72712) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2606::<F>(t14725, t17635, t18329, t4889, t18324, t1174, t135, t22136, t18346, t18580, t19019, t3440, t45128, t5024, t52873, t52893, t66001, t66015, t66024, t66027, t71138);
        let t72735 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2607::<F>(t15740, t18371, t1222, t22175, t1090, t11728, t15569, t18300, t18383, t18946, t22312, t3578, t45114, t4582, t66052, t66054, t66057, t66073, t66076, t66079, t66084, t66092);
        let (t72767, t72783) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2608::<F>(t1734, t6218, t1089, t11678, t11692, t1215, t1227, t15569, t15659, t15700, t15701, t15702, t1735, t18237, t18321, t18368, t18395, t18397, t18401, t3577, t3578, t4582, t4729, t4972, t5046, t52879, t52903, t5398, t5979, t607, t6219, t65464, t65469, t66120, t70458);
        let (t72788, t72823) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2609::<F>(t18231, t3961, t1222, t22169, t11539, t1174, t21745, t11546, t11692, t1227, t15654, t18321, t18342, t19083, t22284, t3440, t3578, t45134, t4582, t4733, t4987, t4989, t5005, t5033, t52893, t52919, t6230, t70316, t70330, t70339, t71133, t71197);
    (t72577, t72593, t72622, t72654, t72683, t72688, t72712, t72735, t72767, t72783, t72788, t72823)
}
