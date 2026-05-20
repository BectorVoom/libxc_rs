//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta24 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk181;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk182;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk183;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk184;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk185;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk186;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta24<F: Float>(t470: F, t68: F, t225: F, t358: F, t425: F, t453: F, t455: F, sigma2: F, t46: F, t47: F, rho1: F, t415: F, t374: F, t375: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t471, t475) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk181::<F>(t470, t68, t225, t358, t425, t453, t455);
        let (t476, t477, t478) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk182::<F>(t475, sigma2);
        let (t479, t480, t483) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk183::<F>(t477, t478, t46, t47, rho1);
        let t484 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk184::<F>(t479, t483);
        let t485 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk185::<F>(t471, t484);
        let t486 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk186::<F>(t415);
        let t488 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk187::<F>(t374, t375, t486);
    (t471, t475, t476, t477, t478, t479, t480, t483, t484, t485, t486, t488)
}
