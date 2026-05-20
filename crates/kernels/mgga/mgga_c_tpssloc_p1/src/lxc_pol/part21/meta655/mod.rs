//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2453;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta655<F: Float>(t11045: F, t42332: F, t42340: F, t42341: F, t43288: F, t23508: F, t43292: F, t10163: F, t386: F, t68: F, t3215: F, t3399: F, t3402: F, t3639: F, t2394: F, t3244: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43562, t43576, t43577, t43604, t43637, t43688) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2453::<F>(t11045, t42332, t42340, t42341, t43288, t23508, t43292, t10163, t386, t68, t3215, t3399);
        let (t43689, t43692, t43706, t43748) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2454::<F>(t43688, t3402, t3639, t2394, t3244);
    (t43562, t43576, t43577, t43604, t43637, t43689, t43692, t43706, t43748)
}
