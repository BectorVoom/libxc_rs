//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk366;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk367;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk368;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk369;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk370;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk371;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk372;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta52<F: Float>(t1036: F, t370: F, t368: F, t372: F, t364: F, t354: F, t270: F, t283: F, t61: F, t248: F, t884: F, t1000: F, t1005: F, t1020: F, t1025: F, t1032: F, t350: F, t378: F, t964: F, t973: F, t997: F, t349: F, t225: F, t382: F, t386: F, t68: F, t1011: F, t1014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1038, t1040) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk366::<F>(t1036, t370, t368, t372, t364);
        let t1041 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk367::<F>(t1040, t354);
        let t1043 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk368::<F>(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk369::<F>(t1043, t61);
        let (t1046, t1049) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk370::<F>(t1044, t248, t884, t1000, t1005, t1020, t1025, t1032, t1038, t1041, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk371::<F>(t1049, t349, t225, t382);
        let (t1053, t1055) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk372::<F>(t386, t68);
        let t1057 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk373::<F>(t1011, t1014);
    (t1038, t1040, t1041, t1043, t1044, t1046, t1049, t1050, t1052, t1053, t1055, t1057)
}
