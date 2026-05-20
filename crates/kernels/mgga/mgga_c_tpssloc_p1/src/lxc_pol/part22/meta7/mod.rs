//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta7 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk48;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk49;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk50;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk51;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk52;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk53;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk54;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk55;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk56;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk57;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk58;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta7<F: Float>(t107: F, t64: F, t89: F, t25: F, dens_threshold: F, rho0: F, zeta_threshold: F, t67: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t111, t109) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk48::<F>(t107, t64);
        let t112 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk49::<F>(t111);
        let t113 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk50::<F>(t112, t89);
        let t116 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk51::<F>(t25, dens_threshold, rho0, zeta_threshold);
        let t117 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk52::<F>(t116);
        let t118 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk53::<F>(t117, t67);
        let t119 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk54::<F>();
        let t120 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk55::<F>(t119);
        let t121 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk56::<F>(t60);
        let t122 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk57::<F>(t120, t121);
        let t123 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk58::<F>(t118, t122);
    (t111, t109, t112, t113, t116, t117, t118, t119, t120, t121, t122, t123)
}
