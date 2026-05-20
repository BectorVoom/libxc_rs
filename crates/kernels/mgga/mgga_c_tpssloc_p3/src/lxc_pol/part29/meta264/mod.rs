//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta264 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1239;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1240;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1241;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1242;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1243;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta264<F: Float>(t50: F, t6794: F, t131: F, t467: F, t1009: F, t461: F, t1209: F, t475: F, t68: F, t1245: F, t1235: F, t2147: F, t462: F, t1215: F, t2144: F, t1246: F, t493: F, t7348: F, t1201: F, t1244: F, t2121: F, t2152: F, t470: F, t7283: F, t7361: F, t7365: F, t7368: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7371, t7372, t7373) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1239::<F>(t50, t6794, t131, t467);
        let t7375 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1240::<F>(t1009, t461, t1209);
        let t7376 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1241::<F>(t475, t68);
        let (t7377, t7378) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1242::<F>(t1245, t7376, t7375);
        let t7381 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1243::<F>(t1235, t2147);
        let (t7382, t7387, t7389, t7391) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1244::<F>(t462, t7381, t1215, t2144, t1246, t493, t7348, t1201, t1244, t2121, t2152, t470, t7283, t7361, t7365, t7368, t7373, t7378);
    (t7371, t7372, t7373, t7375, t7376, t7377, t7378, t7381, t7382, t7387, t7389, t7391)
}
