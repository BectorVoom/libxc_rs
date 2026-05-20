//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta130 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk869;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk870;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk871;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk872;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta130<F: Float>(t3147: F, t974: F, t2775: F, t976: F, t2244: F, t1005: F, t1036: F, t221: F, t2965: F, t339: F, t964: F, t995: F, t1000: F, t1020: F, t1025: F, t1046: F, t2955: F, t2960: F, t3109: F, t3114: F, t3117: F, t3123: F, t3130: F, t3134: F, t3140: F, t3143: F, t350: F, t973: F, t3106: F, t349: F, t1050: F, t225: F, t1053: F, t386: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3148, t3151, t3152, t3153, t3156, t3158, t3160, t3163) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk869::<F>(t3147, t974, t2775, t976, t2244, t1005, t1036, t221, t2965, t339, t964, t995);
        let t3165 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk870::<F>(t1000, t1020, t1025, t1046, t2955, t2960, t3109, t3114, t3117, t3123, t3130, t3134, t3140, t3143, t3148, t3153, t3156, t3160, t3163, t350, t973);
        let t3166 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk871::<F>(t3106, t3165);
        let (t3167, t3169) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk872::<F>(t3166, t349, t1050, t225);
        let t3174 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk873::<F>(t1053, t386, t68);
    (t3151, t3152, t3156, t3158, t3160, t3163, t3166, t3167, t3169, t3174)
}
