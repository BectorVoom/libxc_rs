//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta801 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta801<F: Float>(t46376: F, t16710: F, t2663: F, t41255: F, t41259: F, t46433: F, t46435: F, t46437: F, t46439: F, t16717: F, t47176: F, t157: F, t46387: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t58983, t58985, t58986, t58987, t58988, t58989, t58990, t58991, t58993, t58994) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2788::<F>(t46376, t16710, t2663, t41255, t41259, t46433, t46435, t46437, t46439, t16717, t47176, t157, t46387);
    (t58983, t58985, t58986, t58987, t58988, t58989, t58990, t58991, t58993, t58994)
}
