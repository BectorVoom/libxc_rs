//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk161;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk162;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk163;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk164;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk165;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta21<F: Float>(t407: F, t405: F, t281: F, t282: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t409 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk161::<F>(t407);
        let t410 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk162::<F>(t407);
        let (t413, t415) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk163::<F>(t407, t405);
        let t417 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk164::<F>(t281, t282, t415);
        let (t419, t422, t423) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk165::<F>(t407, t410, t413, t417);
        let (t425, t427) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk166::<F>(t409, t423, t407);
    (t409, t410, t413, t415, t417, t419, t422, t423, t425, t427)
}
