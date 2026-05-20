//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2168;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta633<F: Float>(t54555: F, t12289: F, t1336: F, t836: F, t1811: F, t40005: F, t40281: F, t5259: F, t1361: F, t242: F, t12189: F, t5206: F, t40406: F, t5202: F, t12199: F, t16111: F, t1804: F, t2585: F, t3732: F, t46853: F, t5308: F, t16118: F, t9577: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54556, t54566, t54582, t54612, t54614, t54631) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2168::<F>(t54555, t12289, t1336, t836, t1811, t40005, t40281, t5259, t1361, t242, t12189, t5206);
        let (t54633, t54638, t54639, t54644, t54663) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2169::<F>(t40406, t5202, t12199, t16111, t1804, t40005, t2585, t3732, t46853, t5308, t16118, t9577);
    (t54556, t54566, t54582, t54612, t54614, t54631, t54633, t54638, t54639, t54644, t54663)
}
