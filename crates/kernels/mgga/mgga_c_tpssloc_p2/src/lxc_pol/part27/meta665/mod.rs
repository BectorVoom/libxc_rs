//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2335;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2336;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2337;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta665<F: Float>(t1354: F, t91278: F, t1827: F, t80991: F, t22765: F, t5289: F, t22764: F, t5234: F, t26298: F, t80958: F, t1307: F, t1339: F, t22827: F, t5287: F, t54068: F, t550: F, t22779: F, t26319: F, t80837: F, t80843: F, t80848: F, t80857: F, t80859: F, t91261: F, t91263: F, t91268: F, t91272: F, t91276: F, t1358: F, t26248: F, t3862: F, t7715: F, t22705: F, t22852: F, t236: F, t5286: F, t26245: F, t80791: F, t80867: F, t22788: F, t5310: F, t16150: F, t6952: F, t16155: F, t26271: F, t80836: F, t1361: F, t22690: F, t22792: F, t5187: F, t16148: F, t26288: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91279, t91282, t91284, t91287, t91290, t91294) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2335::<F>(t1354, t91278, t1827, t80991, t22765, t5289, t22764, t5234, t26298, t80958, t1307, t1339, t22827, t5287);
        let t91302 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2336::<F>(t1339, t22827, t54068, t550, t22779, t26319, t80837, t80843, t80848, t80857, t80859, t91261, t91263, t91268, t91272, t91276, t91279, t91282, t91284, t91287, t91290, t91294);
        let (t91304, t91305, t91311, t91312, t91314) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2337::<F>(t1358, t26248, t3862, t7715, t22705, t22852, t236, t5286, t550, t26245, t80791, t80867);
        let (t91317, t91319, t91321, t91323, t91328, t91330) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2338::<F>(t22788, t5310, t16150, t6952, t16155, t26271, t80836, t1361, t22690, t22792, t5187, t16148, t26288);
    (t91302, t91304, t91305, t91311, t91312, t91314, t91317, t91319, t91321, t91323, t91328, t91330)
}
