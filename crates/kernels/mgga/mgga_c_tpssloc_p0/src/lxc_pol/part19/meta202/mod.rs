//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk871;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk872;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta202<F: Float>(t10321: F, t908: F, t136: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10311: F, t10314: F, t10318: F, t10320: F, t340: F, t343: F, t974: F, t2955: F, t969: F, t2967: F, t964: F, t63: F, t344: F, t221: F, t339: F, t2960: F, t2974: F, t3014: F, t984: F, t135: F, t3016: F, t973: F, t10263: F, t10267: F, t10274: F, t10280: F, t10283: F, t10287: F, t10290: F, t2996: F, t3000: F, t3011: F, t3017: F, t346: F, t987: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10322, t10323, t10325) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk871::<F>(t10321, t908, t136, t10295, t10296, t10298, t10300, t10302, t10307, t10311, t10314, t10318, t10320);
        let (t10327, t10328, t10331, t10333, t10335, t10337, t10339) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk872::<F>(t10325, t340, t343, t974, t2955, t969, t2967, t964, t63, t344, t221, t339);
        let (t10346, t10348, t10349, t10352, t10357) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk873::<F>(t2960, t2974, t3014, t984, t340, t343, t974, t135, t3016, t973, t10263, t10267, t10274, t10280, t10283, t10287, t10290, t10328, t10331, t10333, t10339, t2996, t3000, t3011, t3017, t346, t987);
    (t10322, t10323, t10325, t10327, t10328, t10335, t10337, t10346, t10348, t10349, t10352, t10357)
}
