//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2495;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta683<F: Float>(t41189: F, t4134: F, t118: F, t12971: F, t2576: F, t794: F, t13025: F, t9546: F, t13017: F, t2563: F, t1489: F, t41083: F, t2559: F, t4126: F, t4130: F, t12997: F, t13000: F, t2566: F, t67: F, t792: F, t9558: F, t12984: F, t2379: F, t686: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46772, t46780, t46782, t46788, t46790) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2495::<F>(t41189, t4134, t118, t12971, t2576, t794, t13025, t9546, t13017, t2563, t1489, t41083);
        let (t46793, t46796, t46799, t46802) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2496::<F>(t2559, t4126, t4130, t12997, t13000, t2566, t67, t792, t9558, t12984, t2379, t686);
    (t46772, t46780, t46782, t46788, t46790, t46793, t46796, t46799, t46802)
}
