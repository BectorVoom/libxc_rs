//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1049;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1050;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1051;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta161<F: Float>(t2223: F, t522: F, t2516: F, t521: F, t17: F, t1284: F, t750: F, t1285: F, t592: F, t1287: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3823, t3824) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1049::<F>(t2223, t522, t2516, t521);
        let t3825 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1050::<F>(t17, t3824);
        let t3826 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1051::<F>(t1284, t750);
        let (t3827, t3828, t3829, t3830, t3832) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1052::<F>(t17, t3826, t1285, t592, t1287);
    (t3823, t3824, t3825, t3826, t3827, t3828, t3829, t3830, t3832)
}
