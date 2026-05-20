//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk975;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta243<F: Float>(t11129: F, t11282: F, t11285: F, t1164: F, t3411: F, t3415: F, t3399: F, t445: F, t3403: F, t1143: F, t3375: F, t1156: F, t1124: F, t3331: F, t1136: F, t3333: F, t1137: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11286, t11288, t11290, t11292) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk975::<F>(t11129, t11282, t11285, t1164, t3411, t3415, t3399, t445);
        let (t11294, t11296, t11297, t11300, t11303, t11306, t11307) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk976::<F>(t11129, t11292, t3403, t1164, t1143, t3375, t1156, t1124, t3331, t1136, t3333, t1137);
    (t11286, t11288, t11290, t11292, t11294, t11296, t11297, t11300, t11303, t11306, t11307)
}
