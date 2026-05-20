//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta52 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk375;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk376;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk377;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk378;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk379;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk380;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk381;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk382;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk383;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta52<F: Float>(t376: F, t61: F, t890: F, t916: F, t956: F, t958: F, t963: F, t360: F, t248: F, t34: F, t365: F, t35: F, t364: F, t354: F, t122: F, t374: F, t370: F, t368: F, t372: F, t270: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1021 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk375::<F>(t376, t61);
        let t1022 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk376::<F>(t890, t916, t956, t958, t963);
        let t1023 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk377::<F>(t1022, t360);
        let t1025 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk378::<F>(t1021, t1023, t248);
        let t1030 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk379::<F>(t34, t365, t35);
        let t1031 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk380::<F>(t1030, t364);
        let (t1032, t1036) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk381::<F>(t1031, t354, t122, t374, t376);
        let (t1038, t1040) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk382::<F>(t1036, t370, t368, t372, t364);
        let t1041 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk383::<F>(t1040, t354);
        let t1043 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk384::<F>(t270, t283);
    (t1021, t1022, t1023, t1025, t1030, t1031, t1032, t1036, t1038, t1040, t1041, t1043)
}
