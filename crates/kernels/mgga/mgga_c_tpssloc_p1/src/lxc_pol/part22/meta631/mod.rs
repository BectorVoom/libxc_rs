//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta631<F: Float>(t5154: F, t9722: F, t39659: F, t2221: F, t5166: F, t2223: F, t1788: F, t9216: F, t9218: F, t39855: F, t39857: F, t9494: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54451, t54453, t54457, t54459, t54461, t54462, t54465, t54466, t54467) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2166::<F>(t5154, t9722, t39659, t2221, t5166, t2223, t1788, t9216, t9218, t39855, t39857, t9494);
    (t54451, t54453, t54457, t54459, t54461, t54462, t54465, t54466, t54467)
}
