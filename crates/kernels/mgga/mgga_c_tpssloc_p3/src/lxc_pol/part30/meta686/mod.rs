//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta686 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2167;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2168;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2169;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2170;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2171;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2172;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2173;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2174;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2175;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta686<F: Float>(t26322: F, t7708: F, t91202: F, t20004: F, t26309: F, t19945: F, t19981: F, t22833: F, t19994: F, t221: F, t26284: F, t19631: F, t1998: F, t236: F, t6926: F, t1361: F, t22690: F, t6330: F, t80840: F, t22792: F, t6347: F, t80900: F, t80915: F, t91387: F, t93757: F, t97394: F, t97398: F, t97400: F, t97402: F, t97404: F, t97407: F, t26318: F, t91351: F, t19844: F, t6916: F, t22804: F, t28077: F, t22779: F, t28067: F, t19924: F, t26288: F, t19919: F, t91194: F, t91198: F, t20000: F, t91361: F, t28060: F, t80940: F, t80957: F, t80971: F, t91400: F, t91403: F, t91404: F, t93760: F, t97233: F, t97268: F, t97309: F, t97349: F, t97376: F, t97392: F, t19661: F, t1992: F, t22897: F, t19736: F, t22892: F, t22893: F, t28138: F, t1336: F, t1352: F, t16060: F, t19810: F, t26404: F, t26442: F, t26456: F, t26458: F, t28152: F, t3777: F, t5234: F, t5287: F, t5344: F, t544: F, t553: F, t7745: F, t91065: F, t91077: F, t93795: F, t93796: F, t97172: F, t97181: F, t97189: F, t97200: F, t28116: F, t81228: F, t81326: F, t6897: F, t7700: F, t90544: F, t214: F, t6434: F, t1985: F, t6907: F, t22633: F, t26215: F, t90566: F, t80722: F, t80744: F, t81264: F, t90605: F, t90609: F, t90646: F, t93438: F, t93445: F, t22635: F, t26354: F, t5353: F, t26338: F, t22751: F, t28213: F, t28210: F, t28233: F, t6883: F, t1323: F, t16439: F, t19804: F, t2006: F, t22656: F, t28107: F, t28187: F, t3882: F, t568: F, t6361: F, t6461: F, t6955: F, t7750: F, t81284: F, t90702: F, t90708: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t97410, t97412, t97414, t97416, t97419, t97423) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2167::<F>(t26322, t7708, t91202, t20004, t26309, t19945, t19981, t22833, t19994, t221, t26284, t19631, t1998, t236, t6926);
        let t97433 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2168::<F>(t1361, t22690, t6330, t80840, t22792, t6347, t80900, t80915, t91387, t93757, t97394, t97398, t97400, t97402, t97404, t97407, t97410, t97412, t97414, t97416, t97419, t97423);
        let (t97435, t97437, t97439, t97444, t97447) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2169::<F>(t26318, t7708, t91351, t19844, t6916, t22804, t28077, t22779, t28067, t1361, t19924, t26288);
        let (t97450, t97453, t97456, t97459, t97461, t97463) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2170::<F>(t1361, t19994, t26288, t19919, t221, t91194, t19924, t26284, t91198, t20000, t91361, t22779, t28060);
        let t97465 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2171::<F>(t80940, t80957, t80971, t91400, t91403, t91404, t93760, t97435, t97437, t97439, t97444, t97447, t97450, t97453, t97456, t97459, t97461, t97463);
        let (t97468, t97488, t97491) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2172::<F>(t97233, t97268, t97309, t97349, t97376, t97392, t97433, t97465, t19661, t1992, t22897, t19736);
        let t97496 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2173::<F>(t22892, t22893, t28138, t1336, t1352, t16060, t19810, t26404, t26442, t26456, t26458, t28152, t3777, t5234, t5287, t5344, t544, t553, t7745, t91065, t91077, t93795, t93796, t97172, t97181, t97189, t97200, t97468, t97488, t97491);
        let (t97503, t97509, t97511, t97513, t97516) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2174::<F>(t28116, t81228, t81326, t6897, t7700, t90544, t214, t6434, t1985, t6907, t22633, t26215, t90566);
        let (t97519, t97524) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2175::<F>(t80722, t80744, t81264, t90605, t90609, t90646, t93438, t93445, t97509, t97513, t97516, t1992, t22635, t26354, t5353);
        let (t97527, t97529, t97552) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2176::<F>(t22633, t26338, t90566, t22751, t28213, t28210, t28233, t6883, t1323, t16439, t19804, t2006, t22656, t28107, t28187, t3882, t568, t6361, t6461, t6955, t7750, t81284, t90702, t90708);
    (t97468, t97496, t97503, t97511, t97519, t97524, t97527, t97529, t97552)
}
