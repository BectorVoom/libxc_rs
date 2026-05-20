//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2015;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta505<F: Float>(t604: F, t9226: F, t2233: F, t2239: F, t601: F, t9238: F, t85: F, t24: F, t10276: F, t73: F, t11152: F, t76: F, t41: F, t42: F, t53: F, t54: F, t9576: F, t111: F, t9346: F, t2405: F, t2420: F, t702: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39046, t39049, t39054, t39063, t39096, t39114) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2015::<F>(t604, t9226, t2233, t2239, t601, t9238, t85, t24, t10276, t73, t11152, t76);
        let (t39159, t39168, t39210, t39235, t39246, t39249) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2016::<F>(t41, t42, t53, t54, t9576, t111, t9346, t2405, t2420, t702);
    (t39046, t39049, t39054, t39063, t39096, t39114, t39159, t39168, t39210, t39235, t39246, t39249)
}
