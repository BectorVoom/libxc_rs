//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta856 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3096;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3097;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3098;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3099;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3100;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3101;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3102;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3103;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3104;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3105;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3106;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta856<F: Float>(t43748: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63891: F, t63893: F, t63896: F, t63899: F, t63903: F, t63906: F, t63909: F, t63911: F, t63914: F, t50846: F, t50848: F, t50853: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F, t63997: F, t64003: F, t64006: F, t64009: F, t43855: F, t43859: F, t43861: F, t43863: F, t44249: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t50952: F, t50954: F, t43780: F, t43782: F, t43816: F, t44275: F, t50968: F, t50970: F, t50972: F, t50978: F, t51039: F, t51041: F, t64028: F, t64031: F, t64033: F, t64042: F, t64045: F, t51043: F, t51051: F, t51053: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F, t43959: F, t6024: F, t1128: F, t18668: F, t3263: F, t5983: F, t3266: F, t1129: F, t1137: F, t1138: F, t11410: F, t15118: F, t15141: F, t1683: F, t18840: F, t18894: F, t3327: F, t3352: F, t3360: F, t44211: F, t4797: F, t4820: F, t51594: F, t6037: F, t6053: F, t63763: F, t63765: F, t63767: F, t63769: F, t63771: F, t63829: F, t64100: F, t64103: F, t64132: F, t64148: F, t4819: F, t3331: F, t6031: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63841: F, t63843: F, t63845: F, t63323: F, t63327: F, t63330: F, t63848: F, t63853: F, t63856: F, t63858: F, t63860: F, t63862: F, t63865: F, t63867: F, t63870: F, t63873: F, t63876: F, t63879: F, t43777: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t64165 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3096::<F>(t43748, t63332, t63334, t63336, t63886, t63888, t63891, t63893, t63896, t63899, t63903, t63906, t63909, t63911, t63914);
        let t64181 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3097::<F>(t50846, t50848, t50853, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939, t63997, t64003, t64006, t64009);
        let t64197 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3098::<F>(t43855, t43859, t43861, t43863, t44249, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954);
        let t64212 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3099::<F>(t43780, t43782, t43816, t44275, t50968, t50970, t50972, t50978, t51039, t51041, t64028, t64031, t64033, t64042, t64045);
        let t64229 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3100::<F>(t51043, t51051, t51053, t63355, t63359, t63361, t63365, t63370, t63374, t63380, t63382, t63384, t63388, t63392, t63396);
        let t64245 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3101::<F>(t63398, t63400, t63404, t63408, t63412, t63417, t63422, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let (t64253, t64259, t64260) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3102::<F>(t43959, t6024, t1128, t18668, t3263, t5983, t3266, t1129, t1137, t1138, t11410, t15118, t15141, t1683, t18840, t18894, t3327, t3352, t3360, t44211, t4797, t4820, t51594, t6037, t6053, t63763, t63765, t63767, t63769, t63771, t63829, t64100, t64103, t64132, t64148, t64165, t64181, t64197, t64212, t64229, t64245);
        let (t64261, t64292, t64309) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3103::<F>(t4819, t3331, t6031, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63841, t63843, t63845);
        let t64325 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3104::<F>(t63323, t63327, t63330, t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867, t63870, t63873, t63876, t63879);
        let t64342 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3105::<F>(t43748, t63332, t63334, t63336, t63886, t63888, t63891, t63893, t63896, t63899, t63903, t63906, t63909, t63911, t63914);
        let t64358 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3106::<F>(t50846, t50848, t50853, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939, t63997, t64003, t64006, t64009);
        let t64374 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3107::<F>(t43777, t43855, t43859, t43861, t43863, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954);
    (t64253, t64259, t64260, t64261, t64292, t64309, t64325, t64342, t64358, t64374)
}
