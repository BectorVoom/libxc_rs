//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta178 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk868;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk869;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk870;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk871;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta178<F: Float>(t1385: F, t3887: F, t3787: F, t562: F, t3793: F, t1338: F, t1372: F, t1352: F, t1380: F, t3851: F, t3856: F, t3879: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3777: F, t544: F, t564: F, t1378: F, t1375: F, t1386: F, t3753: F, t3755: F, t3758: F, t3880: F, t3882: F, t568: F, t193: F, t532: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3888 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk867::<F>(t1385);
        let t3889 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk868::<F>(t3887, t3888);
        let (t3898, t3901, t3902, t3905, t3907, t3909, t3911) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk869::<F>(t3787, t562, t3793, t1338, t1372, t1352, t1380, t3851, t3856, t3879, t553, t1332, t1336, t1381, t1383, t3773, t3777, t544, t564);
        let t3912 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk870::<F>(t1378, t3911);
        let t3914 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk871::<F>(t1375, t1386, t3753, t3755, t3758, t3880, t3882, t3889, t3912, t568);
        let t3918 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk872::<F>(t193, t532);
    (t3888, t3889, t3898, t3901, t3902, t3905, t3907, t3909, t3911, t3912, t3914, t3918)
}
