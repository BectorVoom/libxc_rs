//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2437;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta645<F: Float>(t10402: F, t11037: F, t2402: F, t973: F, t999: F, t1030: F, t10477: F, t10472: F, t10475: F, t3128: F, t10903: F, t10948: F, t10890: F, t10508: F, t248: F, t3130: F, t3132: F, t1015: F, t3033: F, t42520: F, t3142: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42546, t42552, t42559, t42561, t42565, t42570) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2437::<F>(t10402, t11037, t2402, t973, t999, t1030, t10477, t10472, t10475, t3128, t10903, t10948);
        let (t42573, t42586, t42600, t42610) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2438::<F>(t10890, t10948, t10508, t248, t3130, t3132, t1015, t3033, t42520, t3142, t698, t973);
    (t42546, t42552, t42559, t42561, t42565, t42570, t42573, t42586, t42600, t42610)
}
