//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta167 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk888;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk889;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk890;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk891;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk892;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk893;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta167<F: Float>(t1932: F, t475: F, t3611: F, t3590: F, t493: F, t1201: F, t1244: F, t1247: F, t1249: F, t3565: F, t3604: F, t3610: F, t3613: F, t3617: F, t3621: F, t3624: F, t470: F, t494: F, t1241: F, t1238: F, t1252: F, t3482: F, t3484: F, t3487: F, t3591: F, t3593: F, t3600: F, t498: F, t1254: F, t500: F, t1256: F, t193: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t336: F, t3408: F, t3410: F, t3413: F, t3417: F, t3421: F, t3425: F, t28: F, t265: F, t504: F, t2756: F, t1081: F, t1260: F, t2250: F, t3231: F, t506: F, t52: F, t607: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t3227: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3625, t3626, t3628, t3630) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk888::<F>(t1932, t475, t3611, t3590, t493, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t470, t494);
        let t3631 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk889::<F>(t1241, t3630);
        let (t3633, t3637, t3639) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk890::<F>(t1238, t1252, t3482, t3484, t3487, t3591, t3593, t3600, t3631, t498, t1254, t500);
        let t3640 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk891::<F>(t3639);
        let t3643 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk892::<F>(t1256, t193, t3258, t3261, t3268, t3310, t3318, t336, t3408, t3410, t3413, t3417, t3421, t3425, t3633, t3637, t3640);
        let (t3644, t3651) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk893::<F>(t28, t265, t504, t2756, t3643, t1081, t1260, t2250, t3231, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t3652 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk894::<F>(t3227, t3651);
    (t3625, t3626, t3628, t3630, t3631, t3633, t3637, t3639, t3640, t3644, t3652)
}
