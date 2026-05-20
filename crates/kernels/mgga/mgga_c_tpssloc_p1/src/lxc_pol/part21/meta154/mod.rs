//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta154 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk998;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk999;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1000;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1001;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1002;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1003;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1004;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1005;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1006;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta154<F: Float>(t1238: F, t1252: F, t3482: F, t3484: F, t3487: F, t3591: F, t3593: F, t3600: F, t3631: F, t498: F, t1254: F, t500: F, t1256: F, t193: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t336: F, t3408: F, t3410: F, t3413: F, t3417: F, t3421: F, t3425: F, t28: F, t265: F, t504: F, t2756: F, t1081: F, t1260: F, t2250: F, t3231: F, t506: F, t52: F, t607: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t3227: F, t1268: F, t2312: F, t2314: F, t2319: F, t2363: F, t671: F, t88: F, t526: F, t606: F, t25: F, t2249: F, t514: F, t528: F, t517: F, t157: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3633, t3637, t3639) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk998::<F>(t1238, t1252, t3482, t3484, t3487, t3591, t3593, t3600, t3631, t498, t1254, t500);
        let t3640 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk999::<F>(t3639);
        let t3643 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1000::<F>(t1256, t193, t3258, t3261, t3268, t3310, t3318, t336, t3408, t3410, t3413, t3417, t3421, t3425, t3633, t3637, t3640);
        let (t3644, t3651) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1001::<F>(t28, t265, t504, t2756, t3643, t1081, t1260, t2250, t3231, t506, t52, t607, t873, dens_threshold, rho1, zeta_threshold);
        let t3652 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1002::<F>(t3227, t3651);
        let (t3660, t3664) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1003::<F>(t1268, t2312, t2314, t2319, t2363, t671, t88, t526);
        let t3665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1004::<F>(t606);
        let (t3671, t3672) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1005::<F>(t25, t2249, t3664, t3665, t514, t528, zeta_threshold);
        let t3673 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1006::<F>(t1081);
        let t3681 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1007::<F>(t28, t3231, t3672, t3673, t517, t157, t3671, zeta_threshold);
    (t3633, t3637, t3639, t3640, t3644, t3652, t3660, t3664, t3665, t3672, t3673, t3681)
}
