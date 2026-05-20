//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1556;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1557;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta308<F: Float>(t3314: F, t422: F, t11191: F, t11275: F, t1146: F, t3399: F, t3402: F, t448: F, t11129: F, t1164: F, t3411: F, t3415: F, t445: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11277, t11278, t11280, t11282) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1556::<F>(t3314, t422, t11191, t11275, t1146, t3399);
        let t11285 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1557::<F>(t3402, t448);
        let (t11286, t11288, t11290, t11292) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1558::<F>(t11129, t11282, t11285, t1164, t3411, t3415, t3399, t445);
    (t11277, t11278, t11280, t11282, t11285, t11286, t11288, t11290, t11292)
}
