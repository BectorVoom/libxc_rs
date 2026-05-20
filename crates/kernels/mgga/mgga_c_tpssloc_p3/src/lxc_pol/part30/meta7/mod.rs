//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta7 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk45;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk46;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk47;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk48;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk49;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk50;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk51;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk52;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk53;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta7<F: Float>(t101: F, t103: F, t100: F, t92: F, t96: F, t64: F, t89: F, t25: F, dens_threshold: F, rho0: F, zeta_threshold: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t104, t106, t107) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk45::<F>(t101, t103, t100, t92, t96);
        let (t111, t109) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk46::<F>(t107, t64);
        let t112 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk47::<F>(t111);
        let t113 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk48::<F>(t112, t89);
        let t116 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk49::<F>(t25, dens_threshold, rho0, zeta_threshold);
        let t117 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk50::<F>(t116);
        let t118 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk51::<F>(t117, t67);
        let t119 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk52::<F>();
        let t120 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk53::<F>(t119);
    (t104, t106, t107, t111, t109, t112, t113, t116, t117, t118, t119, t120)
}
