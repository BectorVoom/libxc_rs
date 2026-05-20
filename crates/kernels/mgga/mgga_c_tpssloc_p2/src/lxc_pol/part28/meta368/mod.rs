//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1384;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1385;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1386;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1387;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1388;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1389;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta368<F: Float>(t13985: F, t4593: F, t4582: F, t3132: F, t3069: F, t4669: F, t10231: F, t4338: F, t973: F, t13542: F, t977: F, t10388: F, t10424: F, t10480: F, t10876: F, t10898: F, t10949: F, t13959: F, t13963: F, t13966: F, t13972: F, t13977: F, t13982: F, t1618: F, t3073: F, t3109: F, t3130: F, t4596: F, t4652: F, t13546: F, t13555: F, t2979: F, t13528: F, t13532: F, t10214: F, t13537: F, t13969: F, t4595: F, t1616: F, t2780: F, t3071: F, t2771: F, t10408: F, t1539: F, t3121: F, t3048: F, t4571: F, t10390: F, t10891: F, t10904: F, t10937: F, t10957: F, t1622: F, t3070: F, t3098: F, t4575: F, t4600: F, t4644: F, t4630: F, t4650: F, t884: F, t10436: F, t10441: F, t10449: F, t10455: F, t10460: F, t10490: F, t10496: F, t10504: F, t10511: F, t10517: F, t10863: F, t10866: F, t10871: F, t4636: F, t3108: F, t4640: F, t1611: F, t3047: F, t3103: F, t4641: F, t1040: F, t4616: F, t1044: F, t13611: F, t248: F, t1023: F, t13975: F, t3041: F, t1031: F, t1612: F, t3082: F, t1025: F, t1041: F, t1046: F, t10873: F, t10883: F, t10952: F, t10965: F, t3039: F, t3117: F, t378: F, t4585: F, t4590: F, t2776: F, t4584: F, t4589: F, t12652: F, t4583: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t14004 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1384::<F>(t13985, t4593, t4582, t3132, t3069, t4669, t10231, t4338, t973, t13542, t977, t10388, t10424, t10480, t10876, t10898, t10949, t13959, t13963, t13966, t13972, t13977, t13982, t1618, t3073, t3109, t3130, t4596, t4652);
        let (t14006, t14009, t14012, t14015, t14018, t14027, t14032) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1385::<F>(t13546, t977, t13555, t2979, t13528, t13532, t10214, t13537, t13969, t4595, t3130, t1616, t2780);
        let t14050 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1386::<F>(t14032, t3071, t1616, t2771, t10408, t1539, t3121, t3048, t4571, t10390, t10891, t10904, t10937, t10957, t14006, t14009, t14012, t14015, t14018, t14027, t1622, t3070, t3098, t4575, t4596, t4600, t4644, t973);
        let t14074 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1387::<F>(t3109, t4630, t4650, t884, t3071, t10436, t10441, t10449, t10455, t10460, t10490, t10496, t10504, t10511, t10517, t10863, t10866, t10871, t1618, t1622, t3048, t3070, t4636);
        let (t14077, t14080, t14084, t14085, t14093, t14098) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1388::<F>(t3108, t4640, t1611, t3047, t3103, t4641, t1040, t4616, t1044, t13611, t248, t1023, t13975);
        let t14120 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1389::<F>(t14098, t4582, t3121, t4593, t3041, t1031, t4616, t1612, t3082, t1025, t1041, t1046, t10873, t10883, t10952, t10965, t14077, t14080, t14084, t14085, t14093, t1622, t3039, t3048, t3117, t378, t4585, t4590, t4600, t4636);
        let (t14122, t14126, t14130, t14136, t14139, t14142) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1390::<F>(t1539, t3132, t3071, t3041, t1616, t2776, t13969, t4584, t1041, t4589, t12652, t4583);
    (t14004, t14050, t14074, t14120, t14122, t14126, t14130, t14136, t14139, t14142)
}
