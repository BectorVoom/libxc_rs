//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta8 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk59;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk60;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk61;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk62;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk63;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk64;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk65;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk66;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta8<F: Float>(t123: F, t67: F, t117: F, t61: F, t119: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t125 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk59::<F>(t123);
        let t126 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk60::<F>(t123);
        let (t129, t131) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk61::<F>(t123, t67);
        let (t132, t133) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk62::<F>(t117, t131);
        let t134 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk63::<F>(t61);
        let t135 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk64::<F>(t119, t134);
        let t136 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk65::<F>(t133, t135);
        let (t138, t141, t142) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk66::<F>(t123, t126, t129, t136);
    (t125, t126, t129, t131, t132, t133, t134, t135, t136, t138, t141, t142)
}
