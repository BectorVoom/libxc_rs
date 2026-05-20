//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta249 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1453;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1454;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1455;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1456;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1457;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1458;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta249<F: Float>(t248: F, t3585: F, t5971: F, t1230: F, t5979: F, t5975: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6092: F, t6094: F, t6096: F, t6100: F, t6104: F, t6108: F, t475: F, t1214: F, t1734: F, t3508: F, t1213: F, t1227: F, t1737: F, t1748: F, t3506: F, t3515: F, t3542: F, t3547: F, t467: F, t5005: F, t5019: F, t5024: F, t5036: F, t5041: F, t6109: F, t6197: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6203, t6207, t6211, t6218) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1453::<F>(t248, t3585, t5971, t1230, t5979, t5975, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108);
        let t6219 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1454::<F>(t475, t6218);
        let (t6221, t6224) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1455::<F>(t1214, t248, t6219, t1734);
        let t6225 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1456::<F>(t3508, t6224);
        let (t6227, t6230) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1457::<F>(t1214, t248, t6225, t475, t6224);
        let (t6232, t6237) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1458::<F>(t1214, t248, t6230, t1213, t1227, t1737, t1748, t3506, t3515, t3542, t3547, t467, t5005, t5019, t5024, t5036, t5041, t6109, t6203, t6207, t6211, t6221, t6227);
        let t6238 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1459::<F>(t6197, t6237);
    (t6203, t6207, t6211, t6218, t6219, t6221, t6224, t6225, t6227, t6230, t6232, t6238)
}
