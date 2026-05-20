//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta46 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk340;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk341;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk342;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk343;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk344;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta46<F: Float>(t884: F, t908: F, t136: F, t886: F, t897: F, t899: F, t902: F, t907: F, t290: F, t893: F, t880: F, t307: F, t302: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t909, t910, t912) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk340::<F>(t884, t908, t136, t886, t897, t899, t902, t907);
        let t913 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk341::<F>(t290);
        let t914 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk342::<F>(t912, t913);
        let (t916, t917, t919) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk343::<F>(t893, t914, t880, t886);
        let (t922, t923) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk344::<F>(t307);
        let t924 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk345::<F>(t302, t923);
    (t909, t910, t912, t913, t914, t916, t917, t919, t922, t923, t924)
}
