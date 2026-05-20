//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta127 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk680;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk681;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk682;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk683;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta127<F: Float>(t1214: F, t248: F, t3494: F, t3030: F, t466: F, t3032: F, t1208: F, t476: F, t478: F, t3036: F, t483: F, t1215: F, t475: F, t1210: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3496, t3499, t3500, t3502, t3503, t3504, t3505, t3506) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk680::<F>(t1214, t248, t3494, t3030, t466, t3032, t1208, t476, t478, t3036, t483);
        let t3507 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk681::<F>(t1215);
        let t3508 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk682::<F>(t475);
        let t3509 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk683::<F>(t3507, t3508);
        let (t3511, t3514, t3515) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk684::<F>(t1214, t248, t3509, t1210, t3504, t3500);
    (t3496, t3499, t3502, t3503, t3505, t3506, t3507, t3508, t3509, t3511, t3514, t3515)
}
