//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta693<F: Float>(t25: F, t53796: F, t5154: F, t9919: F, t39305: F, t3665: F, t584: F, t2249: F, t606: F, t16: F, t5173: F, t591: F, t11987: F, t11988: F, t1298: F, t1408: F, t15989: F, t15992: F, t2: F, t3704: F, t39861: F, t5170: F, t9257: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t53797, t53799, t53800, t53805, t53808, t53814, t53817, t53827) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2642::<F>(t25, t53796, t5154, t9919, t39305, t3665, t584, t2249, t606, t16, t5173, t591, t11987, t11988, t1298, t1408, t15989, t15992, t2, t3704, t39861, t5170, t9257, zeta_threshold);
    (t53797, t53799, t53800, t53805, t53808, t53814, t53817, t53827)
}
