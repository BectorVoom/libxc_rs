//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1319;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1320;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1321;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1322;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1323;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1324;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1325;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta217<F: Float>(t1365: F, t1799: F, t1307: F, t1347: F, t5187: F, t1345: F, t1348: F, t1819: F, t1821: F, t5272: F, t5278: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t1352: F, t5248: F, t5249: F, t120: F, t3805: F, t1831: F, t3866: F, t3870: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5279, t5280, t5283, t5286) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1319::<F>(t1365, t1799, t1307, t1347, t5187, t1345, t1348, t1819, t1821, t5272, t5278, t546, t548);
        let t5287 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1320::<F>(t5286, t550);
        let t5289 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1321::<F>(t1343, t5287, t820);
        let t5293 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1322::<F>(t1352, t5248, t5249);
        let t5301 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1323::<F>(t120, t1799);
        let t5303 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1324::<F>(t1352, t3805, t5301);
        let (t5306, t5308) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1325::<F>(t1831, t3866, t1307, t1799);
        let t5310 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1326::<F>(t3870, t5308, t820);
    (t5279, t5280, t5283, t5286, t5287, t5289, t5293, t5301, t5303, t5306, t5308, t5310)
}
