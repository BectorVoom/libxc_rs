//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta265 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1245;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1246;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1247;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1248;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1249;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1250;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta265<F: Float>(t1241: F, t7391: F, t1238: F, t1252: F, t2121: F, t2155: F, t3487: F, t3593: F, t498: F, t7282: F, t7283: F, t7288: F, t7291: F, t7296: F, t7303: F, t7306: F, t7349: F, t7351: F, t7356: F, t2157: F, t3640: F, t28: F, t265: F, t504: F, t1254: F, t1256: F, t193: F, t336: F, t4700: F, t6834: F, t2161: F, t52: F, t607: F, t6855: F, dens_threshold: F, rho1: F, zeta_threshold: F, t7279: F, t671: F, t6867: F, t6869: F, t6871: F, t7264: F, t7266: F, t113: F, t1266: F, t1393: F, t2114: F, t2165: F, t2167: F, t510: F, t574: F, t650: F, t652: F, t6522: F, t6524: F, t6527: F, t6537: F, t672: F, t6877: F, t6882: F, t6998: F, t7001: F, t7271: F, t3: F, t112: F, t2169: F) -> (F, F, F, F, F, F, F, F, F) {
        let t7392 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1245::<F>(t1241, t7391);
        let t7394 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1246::<F>(t1238, t1252, t2121, t2155, t3487, t3593, t498, t7282, t7283, t7288, t7291, t7296, t7303, t7306, t7349, t7351, t7356, t7392);
        let t7398 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1247::<F>(t2157, t3640);
        let (t7402, t7407) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1248::<F>(t28, t265, t504, t1254, t1256, t193, t336, t4700, t6834, t7394, t7398, t2161, t52, t607, t6855, dens_threshold, rho1, zeta_threshold);
        let t7408 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1249::<F>(t7279, t7407);
        let (t7412, t7415) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1250::<F>(t671, t6867, t6869, t6871, t7264, t7266, t113, t1266, t1393, t2114, t2165, t2167, t510, t574, t650, t652, t6522, t6524, t6527, t6537, t672, t6877, t6882, t6998, t7001, t7271, t7408);
        let (t7416, t7423) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1251::<F>(t3, t7415, t112, t2169);
    (t7392, t7394, t7398, t7402, t7408, t7412, t7415, t7416, t7423)
}
