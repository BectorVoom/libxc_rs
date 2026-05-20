//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta12 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk92;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk93;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk94;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk95;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk96;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk97;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk98;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk99;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk100;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta12<F: Float>(t136: F, t119: F, t210: F, t205: F, t207: F, t154: F, t131: F, t206: F, t209: F, t191: F, t144: F, t186: F, t189: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t212 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk92::<F>(t136);
        let t213 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk93::<F>(t212);
        let t214 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk94::<F>(t119, t213);
        let t215 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk95::<F>(t210, t214);
        let t218 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk96::<F>(t205, t207, t215);
        let (t219, t220, t221) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk97::<F>(t154, t205, t131, t206, t119, t209);
        let t222 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk98::<F>(t220, t221);
        let t225 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk99::<F>(t191);
        let t226 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk100::<F>(t218, t225);
        let t228 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk101::<F>(t144, t186, t189, t225);
    (t212, t213, t214, t215, t218, t219, t221, t222, t225, t226, t228)
}
