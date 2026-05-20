//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta36 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk257;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk258;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk259;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk260;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk261;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk262;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta36<F: Float>(t761: F, t763: F, t201: F, t262: F, t73: F, t76: F, t583: F, t60: F, t59: F, t207: F, t215: F, t154: F, t229: F, t205: F, t16: F, t120: F, t212: F, t118: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t765, t766, t767, t771, t781) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk257::<F>(t761, t763, t201, t262, t73, t76, t583, t60);
        let t782 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk258::<F>(t59, t781);
        let (t785, t786) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk259::<F>(t207, t215, t782, t154, t229);
        let t787 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk260::<F>(t205, t786);
        let t792 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk261::<F>(t16, t59);
        let t794 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk262::<F>(t120, t212);
        let t795 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk263::<F>(t118, t794);
    (t765, t766, t767, t771, t781, t782, t785, t786, t787, t792, t794, t795)
}
