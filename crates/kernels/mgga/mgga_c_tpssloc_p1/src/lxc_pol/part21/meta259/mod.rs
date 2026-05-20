//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1500;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta259<F: Float>(t21: F, t587: F, t14: F, t598: F, t2230: F, t594: F, t2229: F, t3: F, t19: F, t2233: F, t604: F, t2239: F, t601: F, t83: F, t84: F, t85: F) -> (F, F, F, F, F, F, F, F) {
        let (t9216, t9218) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1500::<F>(t21, t587, t14, t598);
        let (t9220, t9223, t9225, t9228, t9231, t9238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1501::<F>(t2230, t594, t2229, t3, t19, t2233, t604, t2239, t601, t83, t84, t85);
    (t9216, t9218, t9220, t9223, t9225, t9228, t9231, t9238)
}
