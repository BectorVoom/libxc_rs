//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta59 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk378;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk379;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk380;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk381;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta59<F: Float>(t1201: F, t68: F, t484: F, t1009: F, t466: F, t1011: F, t476: F, t478: F, t1017: F, t483: F, t486: F, t61: F, t1096: F, t1121: F, t1161: F, t1163: F, t1168: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1202, t1203, t1206, t1207, t1208, t1209) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk378::<F>(t1201, t68, t484, t1009, t466, t1011, t476);
        let (t1210, t1212, t1213) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk379::<F>(t1209, t478, t1017, t483, t1207);
        let t1214 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk380::<F>(t486, t61);
        let t1215 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk381::<F>(t1096, t1121, t1161, t1163, t1168);
        let t1216 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk382::<F>(t1215, t475);
    (t1202, t1203, t1206, t1208, t1209, t1210, t1212, t1213, t1214, t1215, t1216)
}
