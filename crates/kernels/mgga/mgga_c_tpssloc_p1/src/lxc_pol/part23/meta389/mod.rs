//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta389<F: Float>(t15971: F, t588: F, t12364: F, t5234: F, t1811: F, t40005: F, t40406: F, t5202: F, t1804: F, t16118: F, t9577: F, t133: F, t1799: F, t40369: F, t6600: F) -> (F, F, F, F, F, F, F) {
        let (t54477, t54532, t54582, t54633, t54639, t54663, t54725) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1193::<F>(t15971, t588, t12364, t5234, t1811, t40005, t40406, t5202, t1804, t16118, t9577, t133, t1799, t40369, t6600);
    (t54477, t54532, t54582, t54633, t54639, t54663, t54725)
}
