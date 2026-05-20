//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta483 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1464;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1465;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1466;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1467;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1468;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1469;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1470;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta483<F: Float>(t300: F, t78874: F, t78914: F, t78944: F, t79002: F, t78335: F, t78338: F, t78344: F, t78355: F, t78357: F, t78359: F, t78361: F, t78364: F, t78367: F, t78370: F, t78373: F, t78791: F, t78792: F, t78794: F, t6224: F, t11721: F, t1213: F, t1214: F, t15503: F, t19083: F, t22246: F, t22271: F, t22309: F, t248: F, t45030: F, t475: F, t488: F, t5002: F, t53336: F, t6164: F, t6169: F, t6211: F, t65628: F, t65632: F, t65647: F, t65664: F, t65689: F, t72403: F, t1227: F, t1230: F, t15569: F, t1653: F, t19026: F, t19051: F, t22214: F, t22218: F, t22288: F, t22307: F, t3578: F, t44828: F, t45197: F, t5005: F, t6207: F, t6221: F, t6227: F, t65541: F, t65703: F, t72470: F, t72495: F, t72501: F, t77961: F, t77969: F, t11668: F, t11678: F, t11692: F, t15740: F, t19080: F, t22158: F, t22312: F, t45114: F, t52680: F, t5971: F, t5975: F, t6225: F, t6230: F, t65819: F, t72512: F, t72530: F, t72542: F, t72556: F, t72560: F, t1735: F, t1737: F, t1748: F, t21762: F, t21769: F, t3577: F, t467: F, t5979: F, t6219: F, t65935: F, t72304: F, t72307: F, t72597: F, t72600: F, t72632: F, t72634: F, t72648: F, t78506: F, t11719: F, t11728: F, t11738: F, t15438: F, t15659: F, t15737: F, t1743: F, t19056: F, t22115: F, t22275: F, t22314: F, t3506: F, t3515: F, t3585: F, t4582: F, t53472: F, t65474: F, t66015: F, t72669: F, t72673: F, t73028: F, t77965: F, t1174: F, t22149: F, t22154: F, t22301: F, t3440: F, t3508: F, t45037: F, t4889: F, t5024: F, t52836: F, t66057: F, t72703: F, t72705: F, t72708: F, t72727: F, t72733: F, t72798: F, t77981: F, t78031: F, t22162: F, t22185: F, t22284: F, t22299: F, t45119: F, t45192: F, t52903: F, t53079: F, t53099: F, t6192: F, t6232: F, t65545: F, t65815: F, t72815: F, t72849: F, t72857: F, t72864: F, t75836: F, t974: F, t19033: F, t44836: F, t52766: F, t6203: F, t65963: F, t65966: F, t72363: F, t72936: F, t72959: F, t77973: F, t77977: F, t78757: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t79005, t79006) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1463::<F>(t300, t78874, t78914, t78944, t79002, t78335, t78338, t78344, t78355, t78357, t78359, t78361, t78364, t78367, t78370, t78373);
        let (t79008, t79018, t79024) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1464::<F>(t78791, t78792, t78794, t79006, t6224, t11721, t1213, t1214, t15503, t19083, t22246, t22271, t22309, t248, t45030, t475, t488, t5002, t53336, t6164, t6169, t6211, t65628, t65632, t65647, t65664, t65689, t72403);
        let t79056 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1465::<F>(t1227, t1230, t15569, t1653, t19026, t19051, t22214, t22218, t22288, t22307, t248, t3578, t44828, t45197, t5005, t6207, t6211, t6221, t6227, t65541, t65703, t72470, t72495, t72501, t77961, t77969);
        let t79087 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1466::<F>(t11668, t11678, t11692, t15569, t15740, t1653, t19080, t22158, t22312, t3578, t45114, t52680, t5971, t5975, t6221, t6225, t6230, t65819, t72512, t72530, t72542, t72556, t72560);
        let t79120 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1467::<F>(t11668, t11692, t1735, t1737, t1748, t21762, t21769, t3577, t3578, t467, t5971, t5979, t6219, t6230, t65935, t72304, t72307, t72597, t72600, t72632, t72634, t72648, t78506);
        let t79160 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1468::<F>(t11719, t11728, t11738, t1227, t15438, t15659, t15737, t1735, t1743, t19056, t22115, t22271, t22275, t22314, t248, t3506, t3515, t3585, t4582, t488, t53472, t6225, t6230, t65474, t66015, t72669, t72673, t73028, t77965);
        let t79188 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1469::<F>(t1174, t1214, t1227, t1230, t15740, t22149, t22154, t22218, t22301, t248, t3440, t3508, t45037, t4889, t5024, t52836, t66057, t72703, t72705, t72708, t72727, t72733, t72798, t77981, t78031, t79018);
        let t79214 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1470::<F>(t1174, t15740, t1653, t22162, t22185, t22284, t22299, t3578, t45119, t45192, t5005, t52903, t53079, t53099, t6192, t6232, t65545, t65815, t72815, t72849, t72857, t72864, t75836, t974);
        let t79251 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1471::<F>(t1214, t1227, t1230, t1737, t19033, t19051, t19083, t22214, t22284, t248, t3515, t3585, t44836, t475, t5024, t52766, t6203, t6207, t6227, t6232, t65963, t65966, t72363, t72936, t72959, t77973, t77977, t78757, t79018);
    (t79005, t79008, t79018, t79024, t79056, t79087, t79120, t79160, t79188, t79214, t79251)
}
