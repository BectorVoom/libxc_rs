//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta793 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta793<F: Float>(t40738: F, t40745: F, t46283: F, t46285: F, t13133: F, t4202: F, t5597: F, t9912: F, t40754: F, t40761: F, t46291: F, t40741: F, t40743: F, t40748: F, t40760: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t58025, t58026, t58027, t58028, t58030, t58032, t58033, t58034, t58035, t58036) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2754::<F>(t40738, t40745, t46283, t46285, t13133, t4202, t5597, t9912, t40754, t40761, t46291, t40741, t40743, t40748, t40760);
    (t58025, t58026, t58027, t58028, t58030, t58032, t58033, t58034, t58035, t58036)
}
