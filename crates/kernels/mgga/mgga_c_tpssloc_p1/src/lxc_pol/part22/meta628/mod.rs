//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta628<F: Float>(t1788: F, t9214: F, t2223: F, t5168: F, t5157: F, t9874: F, t15908: F, t9885: F, t9888: F, t5154: F, t9713: F, t9905: F) -> (F, F, F, F, F, F, F) {
        let (t54315, t54317, t54325, t54380, t54382, t54389, t54392) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2163::<F>(t1788, t9214, t2223, t5168, t5157, t9874, t15908, t9885, t9888, t5154, t9713, t9905);
    (t54315, t54317, t54325, t54380, t54382, t54389, t54392)
}
