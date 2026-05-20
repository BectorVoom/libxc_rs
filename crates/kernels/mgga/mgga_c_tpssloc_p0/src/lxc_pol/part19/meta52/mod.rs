//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta52 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk341;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk342;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk343;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk344;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk345;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk346;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta52<F: Float>(t270: F, t283: F, t61: F, t248: F, t884: F, t1000: F, t1005: F, t1020: F, t1025: F, t1032: F, t1038: F, t1041: F, t350: F, t378: F, t964: F, t973: F, t997: F, t349: F, t225: F, t382: F, t386: F, t68: F, t1011: F, t1014: F, t1010: F, t1022: F, t381: F, t357: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1043 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk341::<F>(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk342::<F>(t1043, t61);
        let t1046 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk343::<F>(t1044, t248, t884);
        let t1049 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk344::<F>(t1000, t1005, t1020, t1025, t1032, t1038, t1041, t1046, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk345::<F>(t1049, t349, t225, t382);
        let (t1053, t1055, t1057, t1058) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk346::<F>(t386, t68, t1011, t1014, t1010);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk347::<F>(t1022, t381, t357, t360);
    (t1043, t1044, t1046, t1049, t1050, t1052, t1053, t1055, t1057, t1058, t1059, t1060)
}
