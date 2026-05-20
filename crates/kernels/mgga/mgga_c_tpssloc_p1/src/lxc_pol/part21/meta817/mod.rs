//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta817 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2879;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta817<F: Float>(t4370: F, t2798: F, t17292: F, t699: F, t136: F, t59682: F, t908: F, t2403: F, t5720: F, t59690: F, t5723: F, t60149: F, t894: F, t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t59657: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t60160, t60161, t60163, t60166, t60168, t60171, t60173, t60176) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2879::<F>(t4370, t2798, t17292, t699, t136, t59682, t908, t2403, t5720, t59690, t5723, t60149, t894);
        let t60185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2880::<F>(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t59657, t60161, t60163, t60166, t60168, t60171, t60173, t60176);
    (t60160, t60161, t60163, t60166, t60168, t60171, t60173, t60176, t60185)
}
