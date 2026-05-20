//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk778;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk779;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk780;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk781;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk782;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta115<F: Float>(t3236: F, t1124: F, t1128: F, t1127: F, t432: F, t427: F, t3293: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3319, t3327, t3330, t3331) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk778::<F>(t3236, t1124, t1128, t1127, t432);
        let t3332 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk779::<F>(t3331, t427);
        let (t3339, t3346, t3355) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk780::<F>(t3236, t3293, t1127);
        let t3356 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk781::<F>(t3355);
        let t3357 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk782::<F>(t3356, t427);
        let (t3358, t3359) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk783::<F>(t435);
    (t3319, t3327, t3330, t3331, t3332, t3339, t3346, t3355, t3356, t3357, t3358, t3359)
}
