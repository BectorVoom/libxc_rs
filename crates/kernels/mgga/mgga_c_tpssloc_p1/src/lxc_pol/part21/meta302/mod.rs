//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta302 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1641;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1642;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1643;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1644;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta302<F: Float>(t407: F, t11135: F, t410: F, t417: F, t1097: F, t3311: F, t409: F, t3314: F, t422: F, t1146: F, t3399: F, t3402: F, t448: F, t445: F, t1143: F, t3375: F, t1124: F, t3331: F, t440: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11243, t11247, t11265, t11274, t11275) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1641::<F>(t407, t11135, t410, t417, t1097, t3311, t409);
        let (t11277, t11282) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1642::<F>(t3314, t422, t1146, t3399);
        let t11285 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1643::<F>(t3402, t448);
        let t11292 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1644::<F>(t3399, t445);
        let (t11297, t11303, t11310) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1645::<F>(t1143, t3375, t1124, t3331, t11282, t440);
    (t11243, t11247, t11265, t11274, t11275, t11277, t11282, t11285, t11292, t11297, t11303, t11310)
}
