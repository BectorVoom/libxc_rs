//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1654;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta307<F: Float>(t457: F, t63: F, t461: F, t221: F, t456: F, t1186: F, t698: F, t1174: F, t135: F, t3471: F, t1184: F, t4899: F, t3242: F, t460: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11552, t11554, t11556, t11557, t11558, t11560, t11561, t11569) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1654::<F>(t457, t63, t461, t221, t456, t1186, t698, t1174, t135, t3471, t1184, t4899);
        let t11570 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1655::<F>(t3242, t460);
    (t11552, t11554, t11556, t11557, t11558, t11560, t11561, t11569, t11570)
}
