//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta305<F: Float>(t39033: F, t587: F, t598: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F, t604: F, t9226: F, t2233: F, t2239: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39034, t39035, t39036, t39037, t39038, t39040, t39043, t39046, t39049) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1092::<F>(t39033, t587, t598, t14, t2230, t594, t9223, t22811, t19, t604, t9226, t2233, t2239);
    (t39034, t39035, t39036, t39037, t39038, t39040, t39043, t39046, t39049)
}
