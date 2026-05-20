//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk226;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk227;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk228;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk229;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk230;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk231;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta31<F: Float>(t19: F, t598: F, t83: F, t85: F, t24: F, t583: F, t61: F, t59: F, t40: F, t73: F, t52: F, t76: F, t111: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t600, t604, t605, t625) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk226::<F>(t19, t598, t83, t85, t24, t583, t61);
        let t626 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk227::<F>(t59, t625);
        let (t627, t632) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk228::<F>(t626, t40);
        let t634 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk229::<F>(t632, t73);
        let t636 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk230::<F>(t52);
        let t638 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk231::<F>(t636, t76);
        let t652 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk232::<F>(t111, t89);
    (t600, t604, t605, t625, t626, t627, t632, t634, t636, t638, t652)
}
