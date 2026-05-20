//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk301;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk302;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk303;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk304;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta45<F: Float>(t386: F, t68: F, t1011: F, t1014: F, t1010: F, t357: F, t360: F, t390: F, t268: F, t405: F, t878: F, t154: F, t486: F, t636: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1053, t1055, t1057, t1058) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk301::<F>(t386, t68, t1011, t1014, t1010);
        let t1060 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk302::<F>(t357, t360);
        let (t1070, t1086) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk303::<F>(t390, t268, t405, t878);
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk304::<F>(t1086, t154, t486);
        let t1089 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk305::<F>(t636);
    (t1053, t1055, t1057, t1058, t1060, t1070, t1086, t1087, t1088, t1089)
}
