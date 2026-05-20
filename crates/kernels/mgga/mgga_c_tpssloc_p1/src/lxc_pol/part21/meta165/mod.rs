//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta165 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1071;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1072;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1073;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1074;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1075;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1076;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1077;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1078;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta165<F: Float>(t1378: F, t3911: F, t1375: F, t1386: F, t3753: F, t3755: F, t3758: F, t3880: F, t3882: F, t3889: F, t568: F, t193: F, t532: F, t1388: F, t1390: F, t1297: F, t1307: F, t2408: F, t2417: F, t3683: F, t3686: F, t3688: F, t3690: F, t3693: F, t3695: F, t3697: F, t3698: F, t3701: F, t3719: F, t3813: F, t533: F, t531: F, t571: F, t2423: F, t2426: F, t2486: F, t3734: F, t3816: F, t3819: F, t3821: F, t3823: F, t3825: F, t3828: F, t3830: F, t3832: F, t3834: F, t3836: F, t113: F, t1266: F, t1271: F, t1393: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t3652: F, t3660: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F, t3: F, t112: F, t1395: F, t111: F, t576: F, t1401: F, t2319: F, t2363: F, t577: F, t671: F, t2218: F, t2221: F, t2225: F, t2232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3912, t3914) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1071::<F>(t1378, t3911, t1375, t1386, t3753, t3755, t3758, t3880, t3882, t3889, t568);
        let t3918 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1072::<F>(t193, t532);
        let t3919 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1073::<F>(t1388, t1390);
        let t3923 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1074::<F>(t1297, t1307, t1390, t193, t2408, t2417, t3683, t3686, t3688, t3690, t3693, t3695, t3697, t3698, t3701, t3719, t3813, t3914, t3918, t3919, t533);
        let (t3924, t3928) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1075::<F>(t531, t571, t193, t2423, t2426, t2486, t3734, t3816, t3819, t3821, t3823, t3825, t3828, t3830, t3832, t3834, t3836);
        let (t3929, t3931) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1076::<F>(t3923, t3928, t113, t1266, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3652, t3660, t510, t513, t574, t650, t652, t672);
        let (t3932, t3938) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1077::<F>(t3, t3931, t112, t1395);
        let t3941 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1078::<F>(t111, t576);
        let (t3946, t3951) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1079::<F>(t1401, t2319, t2363, t3931, t3938, t3941, t577, t671, t2218, t2221, t2225, t2232);
    (t3912, t3914, t3918, t3919, t3924, t3929, t3931, t3932, t3938, t3941, t3946, t3951)
}
