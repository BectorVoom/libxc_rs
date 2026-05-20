//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta56 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk346;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk347;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta56<F: Float>(t5: F, t1406: F, t1437: F, t605: F, t86: F, t112: F, t1408: F, t109: F, t95: F, t50: F, t103: F, t100: F, t104: F, t92: F, t656: F, t64: F, t654: F, tau1: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t1441, t1442) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk346::<F>(t5, t1406, t1437, t605, t86, t112);
        let t1444 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk347::<F>(t1408);
        let (t1447, t1449, t1450, t1453, t1454, t1458) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk348::<F>(t109, t1444, t95, t50, t103, t100, t104, t92, t656, t64, t654, tau1);
    (t1441, t1442, t1444, t1447, t1449, t1450, t1453, t1454, t1458)
}
