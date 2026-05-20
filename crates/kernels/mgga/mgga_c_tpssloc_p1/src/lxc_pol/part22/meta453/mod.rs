//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1816;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1817;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta453<F: Float>(t3919: F, t6330: F, t12116: F, t12118: F, t12123: F, t12130: F, t12133: F, t12141: F, t15976: F, t16171: F, t19689: F, t19690: F, t19691: F, t19693: F, t19694: F, t19695: F, t19696: F, t19697: F, t19698: F, t5126: F, t9853: F, t9859: F, t19595: F, t20075: F, t20092: F, t19534: F, t510: F, t1458: F, t5107: F, t113: F, t12725: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t19289: F, t19537: F, t2314: F, t4026: F, t4028: F, t4034: F, t4073: F, t4077: F, t5118: F, t513: F, t5361: F, t5460: F, t574: F, t652: F, t7458: F, t6287: F, t671: F, t4072: F, t1266: F, t5493: F, t1271: F, t1393: F, t19450: F, t19451: F, t19456: F, t19461: F, t4037: F, t5450: F, t5457: F, t5494: F, t6295: F, t6468: F, t650: F, t672: F) -> (F, F, F, F, F, F, F, F) {
        let t20096 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1816::<F>(t3919, t6330, t12116, t12118, t12123, t12130, t12133, t12141, t15976, t16171, t19689, t19690, t19691, t19693, t19694, t19695, t19696, t19697, t19698, t5126, t9853, t9859);
        let (t20098, t20100, t20109, t20118) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1817::<F>(t19595, t20075, t20092, t20096, t19534, t510, t1458, t5107, t113, t12725, t1442, t1459, t1774, t1778, t1849, t19289, t19537, t2314, t4026, t4028, t4034, t4073, t4077, t5118, t513, t5361, t5460, t574, t652, t7458);
        let (t20127, t20136, t20143, t20147) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1818::<F>(t6287, t671, t1774, t4072, t1266, t5493, t1271, t1393, t1459, t19450, t19451, t19456, t19461, t2314, t4028, t4034, t4037, t510, t5450, t5457, t5494, t6295, t6468, t650, t652, t672);
    (t20098, t20100, t20109, t20118, t20127, t20136, t20143, t20147)
}
