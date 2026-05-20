//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2497;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta684<F: Float>(t133: F, t1484: F, t41214: F, t6600: F, t12998: F, t46766: F, t686: F, t776: F, t12984: F, t2553: F, t12990: F, t13012: F, t12994: F, t213: F, t221: F, t13004: F, t782: F, t13007: F, t131: F, t205: F, t41160: F, t116: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46806, t46819, t46828, t46830) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2497::<F>(t133, t1484, t41214, t6600, t12998, t46766, t686, t776, t12984, t2553, t12990, t13012);
        let (t46836, t46838, t46843, t46844, t46847, t46853) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2498::<F>(t12994, t13012, t213, t221, t13004, t782, t13007, t131, t205, t41160, t116, t212);
    (t46806, t46819, t46828, t46830, t46836, t46838, t46843, t46844, t46847, t46853)
}
