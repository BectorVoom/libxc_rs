//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1067;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1068;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1069;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta164<F: Float>(t1376: F, t566: F, t68: F, t1385: F, t3787: F, t562: F, t3793: F, t1338: F, t1372: F, t1352: F, t1380: F, t3851: F, t3856: F, t3879: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3777: F, t544: F, t564: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3887 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1067::<F>(t1376, t566, t68);
        let (t3888, t3889) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1068::<F>(t1385, t3887);
        let (t3897, t3898, t3901) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1069::<F>(t3787, t562, t3793, t1338, t1372);
        let (t3902, t3905, t3907, t3909, t3911) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1070::<F>(t1352, t3901, t1380, t3851, t3856, t3879, t553, t1332, t1336, t1381, t1383, t3773, t3777, t3898, t544, t564);
    (t3887, t3888, t3889, t3897, t3898, t3901, t3902, t3905, t3907, t3909, t3911)
}
