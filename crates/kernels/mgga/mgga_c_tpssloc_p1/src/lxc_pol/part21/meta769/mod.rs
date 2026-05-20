//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2665;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2666;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2667;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2668;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta769<F: Float>(t12550: F, t12557: F, t12725: F, t12734: F, t12813: F, t1459: F, t1774: F, t19451: F, t19456: F, t20109: F, t2314: F, t2323: F, t26114: F, t4028: F, t4073: F, t4077: F, t45632: F, t5460: F, t5494: F, t55934: F, t55969: F, t574: F, t652: F, t7458: F, t9348: F, t12823: F, t12841: F, t1442: F, t15857: F, t20143: F, t2320: F, t3652: F, t4034: F, t4037: F, t4072: F, t510: F, t5107: F, t5118: F, t5361: F, t5457: F, t55946: F, t55962: F, t55967: F, t6287: F, t1266: F, t12724: F, t12728: F, t12835: F, t19289: F, t19450: F, t19461: F, t19534: F, t20100: F, t20127: F, t2363: F, t2364: F, t4026: F, t5493: F, t55410: F, t55943: F, t671: F, t672: F, t89: F, t53777: F, t53779: F, t53782: F, t53787: F, t19681: F, t2528: F, t172: F, t19572: F, t763: F, t2535: F, t40611: F, t6324: F, t12477: F, t3698: F, t3918: F, t39249: F, t39256: F, t5160: F, t6347: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t55998 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2665::<F>(t12550, t12557, t12725, t12734, t12813, t1459, t1774, t19451, t19456, t20109, t2314, t2323, t26114, t4028, t4073, t4077, t45632, t5460, t5494, t55934, t55969, t574, t652, t7458, t9348);
        let t56034 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2666::<F>(t12725, t12734, t12823, t12841, t1442, t1459, t15857, t19456, t20109, t20143, t2314, t2320, t3652, t4028, t4034, t4037, t4072, t510, t5107, t5118, t5361, t5457, t5460, t5494, t55946, t55962, t55967, t6287, t652);
        let t56075 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2667::<F>(t1266, t12724, t12728, t12835, t1774, t19289, t19450, t19451, t19461, t19534, t20100, t20127, t2314, t2363, t2364, t3652, t4026, t4028, t4034, t510, t5107, t5493, t55410, t55943, t6287, t652, t671, t672, t7458, t89);
        let (t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56106) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2668::<F>(t53777, t53779, t53782, t53787, t19681, t2528, t172, t19572, t763, t2535, t40611, t6324);
        let t56110 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2669::<F>(t12477, t3698, t3918, t39249, t39256, t5160, t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56106, t6347);
    (t55998, t56034, t56075, t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56110)
}
