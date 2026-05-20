//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta270 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1530;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1531;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1532;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1533;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta270<F: Float>(t123: F, t116: F, t16: F, t2397: F, t9691: F, t693: F, t9694: F, t119: F, t133: F, t625: F, t9689: F, t9692: F, t9695: F, t9698: F, t739: F, t746: F, t761: F, t172: F, t2448: F, t763: F, t177: F, t2508: F, t2512: F, t9490: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9701, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1530::<F>(t123, t116, t16, t2397, t9691, t693, t9694, t119, t133, t625);
        let t9711 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1531::<F>(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
        let t9713 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1532::<F>(t739, t746, t9711);
        let (t9715, t9716, t9717, t9720) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1533::<F>(t761, t9713, t172, t2448, t763, t177, t2508);
        let t9722 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1534::<F>(t2512, t9490, t9720);
    (t9701, t9702, t9704, t9706, t9709, t9711, t9713, t9715, t9716, t9717, t9720, t9722)
}
