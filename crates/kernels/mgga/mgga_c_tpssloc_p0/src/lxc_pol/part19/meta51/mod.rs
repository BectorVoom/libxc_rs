//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta51 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk334;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk335;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk336;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk337;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk338;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk339;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta51<F: Float>(t376: F, t61: F, t890: F, t916: F, t956: F, t958: F, t963: F, t360: F, t248: F, t34: F, t365: F, t35: F, t364: F, t354: F, t122: F, t374: F, t370: F, t368: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1021 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk334::<F>(t376, t61);
        let t1022 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk335::<F>(t890, t916, t956, t958, t963);
        let t1023 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk336::<F>(t1022, t360);
        let t1025 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk337::<F>(t1021, t1023, t248);
        let (t1028, t1030) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk338::<F>(t34, t365, t35);
        let (t1031, t1032, t1036) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk339::<F>(t1030, t364, t354, t122, t374, t376);
        let (t1038, t1040, t1041) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk340::<F>(t1036, t370, t368, t372, t364, t354);
    (t1021, t1022, t1023, t1025, t1028, t1030, t1031, t1032, t1036, t1038, t1040, t1041)
}
