//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta9 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk67;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk68;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk69;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk70;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk71;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk72;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk73;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk74;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk75;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta9<F: Float>(t125: F, t142: F, t32: F, t40: F, zeta_threshold: F, t74: F, t52: F, t77: F, t123: F, t126: F, t129: F, t136: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t144 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk67::<F>(t125, t142);
        let t145 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk68::<F>(t32);
        let (t147, t148) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk69::<F>(t40, zeta_threshold);
        let t152 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk70::<F>(t40, t148, t74, t52, t77, zeta_threshold);
        let t153 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk71::<F>(t145, t152);
        let t154 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk72::<F>();
        let t157 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk73::<F>(t154);
        let t159 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk74::<F>(t123);
        let (t164, t167, t168) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk75::<F>(t123, t126, t129, t136);
    (t144, t145, t147, t148, t152, t153, t154, t157, t159, t164, t167, t168)
}
