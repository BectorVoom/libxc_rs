//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta53 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk384;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk385;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk386;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk387;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk388;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk389;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk390;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk391;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk392;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta53<F: Float>(t270: F, t283: F, t61: F, t248: F, t884: F, t1000: F, t1005: F, t1020: F, t1025: F, t1032: F, t1038: F, t1041: F, t350: F, t378: F, t964: F, t973: F, t997: F, t349: F, t225: F, t382: F, t386: F, t68: F, t1011: F, t1014: F, t1010: F, t1022: F, t381: F, t357: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1043 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk384::<F>(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk385::<F>(t1043, t61);
        let t1046 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk386::<F>(t1044, t248, t884);
        let t1049 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk387::<F>(t1000, t1005, t1020, t1025, t1032, t1038, t1041, t1046, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk388::<F>(t1049, t349, t225, t382);
        let (t1053, t1055) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk389::<F>(t386, t68);
        let t1057 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk390::<F>(t1011, t1014);
        let t1058 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk391::<F>(t1010, t1057);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk392::<F>(t1022, t381, t357, t360);
        let t1061 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk393::<F>(t1059, t1060);
    (t1043, t1044, t1046, t1049, t1050, t1052, t1053, t1055, t1057, t1058, t1060, t1061)
}
