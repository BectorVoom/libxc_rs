//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta861 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3122;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3123;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3124;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3125;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3126;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3127;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta861<F: Float>(t19262: F, t3640: F, t1164: F, t3400: F, t3403: F, t63283: F, t1156: F, t3375: F, t18276: F, t3411: F, t11126: F, t6106: F, t18287: F, t225: F, t11925: F, t11928: F, t1235: F, t1252: F, t14980: F, t15771: F, t15789: F, t15790: F, t15797: F, t15803: F, t1720: F, t1761: F, t18571: F, t19209: F, t19249: F, t27784: F, t3590: F, t3593: F, t3600: F, t4945: F, t498: F, t5055: F, t5089: F, t53677: F, t53703: F, t6150: F, t6244: F, t6268: F, t15419: F, t18215: F, t3447: F, t18469: F, t44525: F, t18206: F, t52133: F, t15324: F, t15327: F, t15376: F, t15379: F, t15391: F, t44529: F, t44558: F, t4900: F, t63386: F, t63394: F, t4899: F, t6138: F, t6144: F, t11571: F, t15313: F, t15320: F, t15396: F, t4904: F, t4919: F, t51948: F, t51961: F, t51970: F, t51980: F, t51988: F, t51991: F, t51995: F, t52040: F, t15420: F, t18211: F, t11575: F, t11579: F, t11584: F, t15268: F, t15321: F, t18409: F, t18416: F, t18420: F, t4908: F, t51975: F, t52013: F, t63298: F, t63302: F, t15402: F, t18225: F, t11589: F, t18427: F, t18221: F, t15399: F, t15403: F, t11593: F, t15314: F, t15332: F, t15335: F, t15395: F, t15415: F, t63415: F, t15339: F, t18232: F, t15317: F, t52019: F, t52022: F, t52038: F, t52050: F, t52053: F, t52057: F, t52061: F, t52064: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t64548, t64558, t64562, t64564, t64566) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3122::<F>(t19262, t3640, t1164, t3400, t3403, t63283, t1156, t3375, t18276, t3411, t11126, t6106);
        let t64602 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3123::<F>(t18287, t225, t11925, t11928, t1235, t1252, t14980, t15771, t15789, t15790, t15797, t15803, t1720, t1761, t18571, t19209, t19249, t27784, t3590, t3593, t3600, t4945, t498, t5055, t5089, t53677, t53703, t6150, t6244, t6268);
        let t64634 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3124::<F>(t15419, t18215, t3447, t18469, t44525, t18206, t52133, t15324, t15327, t15376, t15379, t15391, t44529, t44558, t4900, t63386, t63394);
        let t64660 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3125::<F>(t4899, t6138, t6144, t11571, t15313, t15320, t15376, t15396, t3447, t4904, t4919, t51948, t51961, t51970, t51980, t51988, t51991, t51995, t52040);
        let t64694 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3126::<F>(t15376, t15420, t15419, t18211, t3447, t11575, t11579, t11584, t15268, t15321, t18409, t18416, t18420, t4908, t51975, t52013, t63298, t63302);
        let t64725 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3127::<F>(t15402, t18225, t3447, t11589, t18427, t18221, t15376, t15399, t15403, t18409, t11593, t15314, t15332, t15335, t15395, t15415, t63415);
        let t64746 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3128::<F>(t15339, t15376, t15419, t18232, t3447, t11593, t15317, t18427, t52019, t52022, t52038, t52050, t52053, t52057, t52061, t52064);
    (t64548, t64558, t64562, t64564, t64566, t64602, t64634, t64660, t64694, t64725, t64746)
}
