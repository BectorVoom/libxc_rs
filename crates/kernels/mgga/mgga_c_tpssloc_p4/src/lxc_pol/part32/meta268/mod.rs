//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta268 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1217;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1218;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1219;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1220;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1221;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1222;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1223;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta268<F: Float>(t1245: F, t7376: F, t7375: F, t1235: F, t2147: F, t462: F, t1215: F, t2144: F, t1246: F, t493: F, t7348: F, t1201: F, t1244: F, t2121: F, t2152: F, t470: F, t7283: F, t7361: F, t7365: F, t7368: F, t7373: F, t1241: F, t1238: F, t1252: F, t2155: F, t3487: F, t3593: F, t498: F, t7282: F, t7288: F, t7291: F, t7296: F, t7303: F, t7306: F, t7349: F, t7351: F, t7356: F, t2157: F, t3640: F, t28: F, t265: F, t504: F, t1254: F, t1256: F, t193: F, t336: F, t4700: F, t6834: F, t2161: F, t52: F, t607: F, t6855: F, dens_threshold: F, rho1: F, zeta_threshold: F, t7279: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7377, t7378) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1217::<F>(t1245, t7376, t7375);
        let t7381 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1218::<F>(t1235, t2147);
        let (t7382, t7387, t7389, t7391) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1219::<F>(t462, t7381, t1215, t2144, t1246, t493, t7348, t1201, t1244, t2121, t2152, t470, t7283, t7361, t7365, t7368, t7373, t7378);
        let t7392 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1220::<F>(t1241, t7391);
        let t7394 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1221::<F>(t1238, t1252, t2121, t2155, t3487, t3593, t498, t7282, t7283, t7288, t7291, t7296, t7303, t7306, t7349, t7351, t7356, t7392);
        let t7398 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1222::<F>(t2157, t3640);
        let (t7402, t7407) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1223::<F>(t28, t265, t504, t1254, t1256, t193, t336, t4700, t6834, t7394, t7398, t2161, t52, t607, t6855, dens_threshold, rho1, zeta_threshold);
        let t7408 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1224::<F>(t7279, t7407);
    (t7377, t7378, t7381, t7382, t7387, t7389, t7391, t7392, t7394, t7398, t7402, t7408)
}
