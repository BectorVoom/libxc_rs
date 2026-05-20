//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta71 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk516;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk517;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk518;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk519;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk520;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta71<F: Float>(t1426: F, t33: F, t1409: F, t634: F, t638: F, t72: F, t1411: F, t66: F, t80: F, t5: F, t1406: F, t605: F, t86: F, t112: F, t1408: F, t95: F, t50: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1427, t1430, t1431, t1433, t1434) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk516::<F>(t1426, t33, t1409, t634, t638, t72);
        let t1437 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk517::<F>(t1411, t1427, t1434, t66, t80);
        let (t1441, t1442) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk518::<F>(t5, t1406, t1437, t605, t86, t112);
        let t1444 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk519::<F>(t1408);
        let (t1445, t1447) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk520::<F>(t1444, t95, t50, tau1);
        let t1449 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk521::<F>(t1444);
    (t1427, t1430, t1431, t1433, t1434, t1437, t1441, t1442, t1444, t1445, t1447, t1449)
}
