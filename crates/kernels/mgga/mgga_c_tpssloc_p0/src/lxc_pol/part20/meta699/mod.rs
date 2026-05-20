//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2666;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta699<F: Float>(t5154: F, t9905: F, t15968: F, t67: F, t758: F, t17: F, t750: F, t2225: F, t5166: F, t15921: F, t592: F, t39478: F, t15977: F, t2516: F, t5151: F, t1787: F, t9861: F, t15971: F, t39491: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39490: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54393, t54396, t54399, t54401, t54403, t54404) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2666::<F>(t5154, t9905, t15968, t67, t758, t17, t750, t2225, t5166, t15921, t592, t39478);
        let (t54406, t54409, t54411, t54413, t54414, t54415) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2667::<F>(t15977, t592, t17, t2516, t5151, t1787, t9861, t15971, t39491, t39463, t39468, t39472, t39476, t39483, t39490, t54393, t54396, t54399, t54401, t54403, t54404);
    (t54393, t54396, t54399, t54401, t54403, t54404, t54406, t54409, t54411, t54413, t54414, t54415)
}
