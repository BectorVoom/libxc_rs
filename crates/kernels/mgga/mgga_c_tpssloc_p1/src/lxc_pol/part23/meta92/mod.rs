//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta92 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk523;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk524;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk525;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk526;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta92<F: Float>(t1013: F, t361: F, t363: F, t3037: F, t3033: F, t360: F, t2770: F, t2978: F, t2775: F, t976: F, t221: F, t2965: F, t339: F, t1053: F, t386: F, t68: F, t3032: F, t3031: F, t1932: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3127, t3128, t3129, t3130) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk523::<F>(t1013, t361, t363, t3037, t3033);
        let t3131 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk524::<F>(t360);
        let (t3146, t3151, t3160, t3174, t3185) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk525::<F>(t2770, t2978, t2775, t976, t221, t2965, t339, t1053, t386, t68, t3032, t3127);
        let t3186 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk526::<F>(t3031, t3185);
        let t3188 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk527::<F>(t1932, t3131);
    (t3127, t3128, t3129, t3130, t3131, t3146, t3151, t3160, t3174, t3185, t3186, t3188)
}
