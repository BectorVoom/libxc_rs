//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta127 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk854;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk855;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk856;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk857;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta127<F: Float>(t3087: F, t364: F, t354: F, t1032: F, t1036: F, t1004: F, t1031: F, t1044: F, t248: F, t2776: F, t121: F, t376: F, t1023: F, t1020: F, t1041: F, t1046: F, t3039: F, t3043: F, t3048: F, t3054: F, t3057: F, t3064: F, t3070: F, t3073: F, t3078: F, t3084: F, t378: F, t1017: F, t1030: F, t1015: F, t1012: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3088, t3089, t3092, t3094, t3098, t3101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk854::<F>(t3087, t364, t354, t1032, t1036, t1004, t1031, t1044, t248, t2776, t121, t376);
        let t3103 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk855::<F>(t1023, t248, t3101);
        let (t3104, t3106) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk856::<F>(t1020, t3103, t1041, t1046, t3039, t3043, t3048, t3054, t3057, t3064, t3070, t3073, t3078, t3084, t3089, t3092, t3094, t3098, t378);
        let t3108 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk857::<F>(t1017, t1030, t1015);
        let t3109 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk858::<F>(t1012, t3108);
    (t3088, t3089, t3092, t3094, t3098, t3101, t3103, t3104, t3106, t3108, t3109)
}
