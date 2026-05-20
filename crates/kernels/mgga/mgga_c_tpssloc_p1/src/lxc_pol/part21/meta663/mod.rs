//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta663<F: Float>(t11147: F, t460: F, t11588: F, t3469: F, t1184: F, t15418: F, t4899: F, t3475: F, t11545: F, t135: F, t3439: F, t698: F) -> (F, F, F, F, F, F, F) {
        let (t44505, t44510, t44525, t44529, t44558, t44562, t44571) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2464::<F>(t11147, t460, t11588, t3469, t1184, t15418, t4899, t3475, t11545, t135, t3439, t698);
    (t44505, t44510, t44525, t44529, t44558, t44562, t44571)
}
