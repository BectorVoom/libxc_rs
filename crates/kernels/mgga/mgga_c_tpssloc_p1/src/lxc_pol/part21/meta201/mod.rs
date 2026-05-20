//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1240;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1241;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1242;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1243;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta201<F: Float>(t4882: F, t4883: F, t1164: F, t1171: F, t1706: F, t1420: F, t972: F, t1709: F, t3431: F, t1174: F, t3439: F, t60: F, t461: F, t4724: F, t1409: F, t3450: F, t3449: F, t3448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4884, t4886, t4887, t4889) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1240::<F>(t4882, t4883, t1164, t1171, t1706, t1420, t972);
        let (t4896, t4897, t4899) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1241::<F>(t1709, t3431, t1174, t3439, t60);
        let t4900 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1242::<F>(t461, t4899);
        let (t4901, t4904) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1243::<F>(t4724, t4900, t1409, t3450);
        let (t4905, t4908) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1244::<F>(t3449, t4904, t3448, t461);
    (t4884, t4886, t4887, t4889, t4896, t4897, t4899, t4900, t4901, t4904, t4905, t4908)
}
