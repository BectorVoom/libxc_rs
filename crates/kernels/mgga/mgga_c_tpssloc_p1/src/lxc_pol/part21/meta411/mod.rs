//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1921;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1922;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta411<F: Float>(t1088: F, t14753: F, t123: F, t11137: F, t11139: F, t11141: F, t11143: F, t11247: F, t14702: F, t14708: F, t14721: F, t14723: F, t14724: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t1100: F, t1667: F, t2403: F, t14720: F, t11215: F, t11217: F, t14722: F, t11219: F, t14726: F) -> (F, F, F, F, F, F, F, F) {
        let (t14754, t14755) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1920::<F>(t1088, t14753, t123);
        let t14758 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1921::<F>(t11137, t11139, t11141, t11143, t11247, t14702, t14708, t14721, t14723, t14724, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14759, t14766) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1922::<F>(t1100, t14758, t1667, t2403);
        let (t14768, t14776, t14778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1923::<F>(t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766, t11219, t14726);
    (t14754, t14755, t14758, t14759, t14766, t14768, t14776, t14778)
}
