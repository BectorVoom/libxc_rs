//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta25 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk176;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk177;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk178;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk179;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk180;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk181;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk182;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta25<F: Float>(t477: F, t478: F, t46: F, t47: F, rho1: F, t471: F, t415: F, t374: F, t375: F, t456: F, t467: F, t466: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t479 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk176::<F>(t477, t478);
        let (t480, t483) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk177::<F>(t46, t47, rho1);
        let t484 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk178::<F>(t479, t483);
        let (t485, t486) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk179::<F>(t471, t484, t415);
        let t488 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk180::<F>(t374, t375, t486);
        let t491 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk181::<F>(t456, t467, t485, t488);
        let (t492, t493) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk182::<F>(t466, t491, t477, t68);
        let t494 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk183::<F>(t491, t493);
    (t479, t480, t483, t484, t485, t486, t488, t491, t492, t493, t494)
}
