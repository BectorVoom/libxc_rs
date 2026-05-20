//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta9 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk63;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk64;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk65;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk66;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk67;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk68;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk69;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta9<F: Float>(t123: F, t126: F, t129: F, t136: F, t125: F, t32: F, t40: F, zeta_threshold: F, t74: F, t52: F, t77: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t138, t141, t142, t144) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk63::<F>(t123, t126, t129, t136, t125);
        let t145 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk64::<F>(t32);
        let (t147, t148) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk65::<F>(t40, zeta_threshold);
        let t152 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk66::<F>(t40, t148, t74, t52, t77, zeta_threshold);
        let (t153, t154) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk67::<F>(t145, t152);
        let t157 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk68::<F>(t154);
        let t159 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk69::<F>(t123);
    (t138, t141, t142, t144, t145, t147, t148, t152, t153, t154, t157, t159)
}
