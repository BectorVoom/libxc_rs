//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta111 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk751;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk752;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk753;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk754;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta111<F: Float>(t3037: F, t3128: F, t3033: F, t360: F, t135: F, t999: F, t973: F, t2770: F, t2978: F, t2775: F, t976: F, t1005: F, t1036: F, t221: F, t2965: F, t339: F, t964: F, t995: F, t1050: F, t225: F, t1053: F, t386: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3129 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk751::<F>(t3037, t3128);
        let t3130 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk752::<F>(t3033, t3129);
        let t3131 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk753::<F>(t360);
        let (t3140, t3146, t3151, t3156, t3158, t3160) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk754::<F>(t135, t999, t973, t2770, t2978, t2775, t976, t1005, t1036, t221, t2965, t339);
        let (t3163, t3169, t3174) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk755::<F>(t964, t995, t1050, t225, t1053, t386, t68);
    (t3129, t3130, t3131, t3140, t3146, t3151, t3156, t3158, t3160, t3163, t3169, t3174)
}
