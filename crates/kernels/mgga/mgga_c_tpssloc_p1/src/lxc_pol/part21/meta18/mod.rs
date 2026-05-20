//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta18 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk138;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk139;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk140;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk141;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk142;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk143;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk144;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk145;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta18<F: Float>(t335: F, t131: F, t39: F, t271: F, t60: F, t285: F, t221: F, t225: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t337 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk138::<F>(t335);
        let t338 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk139::<F>(t131, t337);
        let t339 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk140::<F>(t338, t39);
        let t340 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk141::<F>(t271);
        let (t341, t343) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk142::<F>(t340, t60, t285);
        let t344 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk143::<F>(t343);
        let (t346, t349) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk144::<F>(t341, t344, t221, t339);
        let (t350, t353) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk145::<F>(t221, t341, t225, t349);
        let t354 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk146::<F>(t353, t68);
    (t337, t338, t339, t340, t341, t343, t344, t346, t349, t350, t353, t354)
}
