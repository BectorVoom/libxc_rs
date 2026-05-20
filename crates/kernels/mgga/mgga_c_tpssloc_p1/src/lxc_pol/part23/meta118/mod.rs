//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk605;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk606;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta118<F: Float>(t1694: F, t3375: F, t1171: F, t1706: F, t1420: F, t972: F, t1709: F, t3431: F, t1174: F, t3439: F, t60: F, t461: F, t1409: F, t3450: F, t3448: F, t135: F, t1716: F, t1714: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4874, t4887, t4889) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk605::<F>(t1694, t3375, t1171, t1706, t1420, t972);
        let (t4896, t4897, t4899, t4900, t4904) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk606::<F>(t1709, t3431, t1174, t3439, t60, t461, t1409, t3450);
        let (t4908, t4916, t4917, t4919) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk607::<F>(t3448, t461, t135, t1716, t1174, t1714);
    (t4874, t4887, t4889, t4896, t4897, t4899, t4900, t4904, t4908, t4916, t4917, t4919)
}
