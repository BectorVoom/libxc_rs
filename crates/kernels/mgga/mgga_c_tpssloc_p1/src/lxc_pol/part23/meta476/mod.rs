//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1427;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta476<F: Float>(t449: F, t78211: F, t78223: F, t300: F, t14850: F, t21724: F, t1118: F, t11190: F, t78129: F, t6020: F, t3264: F, t3313: F, t3315: F, t78118: F, t78120: F, t78122: F, t78125: F, t78128: F, t78132: F, t78196: F, t78199: F) -> (F, F, F, F, F, F, F) {
        let (t78225, t78227, t78229, t78232, t78236, t78239) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1427::<F>(t449, t78211, t78223, t300, t14850, t21724, t1118, t11190, t78129, t6020, t3264, t3313, t3315);
        let t78240 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1428::<F>(t78118, t78120, t78122, t78125, t78128, t78132, t78196, t78199, t78227, t78229, t78232, t78236, t78239);
    (t78225, t78227, t78229, t78232, t78236, t78239, t78240)
}
