//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta763 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2638;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta763<F: Float>(t2585: F, t3732: F, t46853: F, t5308: F, t16118: F, t9577: F, t212: F, t5187: F, t12225: F, t2586: F, t16100: F, t782: F, t16103: F, t16081: F, t16090: F, t16093: F, t16097: F, t2566: F, t1307: F, t16094: F, t686: F, t16095: F, t3719: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54643, t54663, t54665, t54667, t54670) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2638::<F>(t2585, t3732, t46853, t5308, t16118, t9577, t212, t5187, t12225, t2586, t16100, t782);
        let (t54671, t54673, t54676, t54690, t54698) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2639::<F>(t16103, t54670, t16081, t16090, t16093, t16097, t2566, t1307, t16094, t54665, t686, t16095, t3719);
    (t54643, t54663, t54667, t54670, t54671, t54673, t54676, t54690, t54698)
}
