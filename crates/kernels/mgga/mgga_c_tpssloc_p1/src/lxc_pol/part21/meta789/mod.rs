//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2747;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta789<F: Float>(t17109: F, t870: F, t46206: F, t12939: F, t16716: F, t2250: F, t16558: F, t184: F, t4194: F, t607: F, t16619: F, t16689: F, t2430: F, t12971: F, t2522: F, t39397: F, t39400: F, t39408: F, t39411: F, t40708: F, t4310: F, t4314: F, t4315: F, t776: F) -> (F, F, F, F, F, F) {
        let (t57932, t57936, t57939, t57943, t57946, t57947) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2747::<F>(t17109, t870, t46206, t12939, t16716, t2250, t16558, t184, t4194, t607, t16619, t16689, t2430);
        let (t57948, t57955) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2748::<F>(t57947, t12971, t2522, t39397, t39400, t39408, t39411, t40708, t4310, t4314, t4315, t57932, t57936, t57939, t57943, t57946, t776);
    (t57936, t57939, t57943, t57946, t57948, t57955)
}
