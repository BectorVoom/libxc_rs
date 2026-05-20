//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk277;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk278;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk279;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk280;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk281;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk282;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk283;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk284;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta40<F: Float>(t40: F, t607: F, t767: F, t76: F, zeta_threshold: F, t52: F, t583: F, t60: F, t59: F, t207: F, t215: F, t154: F, t229: F, t205: F, t210: F, t214: F, t16: F, t120: F, t212: F, t118: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t770, t771) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk277::<F>(t40, t607, t767, t76, zeta_threshold);
        let t776 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk278::<F>(t52, t607, t771, t770, zeta_threshold);
        let t781 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk279::<F>(t583, t60);
        let t782 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk280::<F>(t59, t781);
        let (t785, t786) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk281::<F>(t207, t215, t782, t154, t229);
        let t787 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk282::<F>(t205, t786);
        let (t789, t792) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk283::<F>(t210, t214, t776, t16, t59);
        let t794 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk284::<F>(t120, t212);
        let t795 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk285::<F>(t118, t794);
    (t771, t776, t781, t782, t785, t786, t787, t789, t792, t794, t795)
}
