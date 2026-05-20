//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta24 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk174;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk175;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk176;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk177;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk178;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk179;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk180;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta24<F: Float>(t440: F, t449: F, t300: F, t425: F, t427: F, t436: F, t338: F, t51: F, t405: F, t60: F, t417: F, t221: F, t225: F, t68: F, t358: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t453, t455, t456) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk174::<F>(t440, t449, t300, t425, t427, t436, t338, t51);
        let t457 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk175::<F>(t405);
        let (t458, t460) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk176::<F>(t457, t60, t417);
        let t461 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk177::<F>(t460);
        let (t463, t466) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk178::<F>(t458, t461, t221, t456);
        let (t467, t470) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk179::<F>(t221, t458, t225, t466);
        let t471 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk180::<F>(t470, t68);
        let t475 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk181::<F>(t225, t358, t425, t453, t455);
    (t453, t455, t456, t457, t460, t461, t463, t466, t467, t470, t471, t475)
}
