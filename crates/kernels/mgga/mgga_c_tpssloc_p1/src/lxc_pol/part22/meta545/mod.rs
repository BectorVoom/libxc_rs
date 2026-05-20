//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta545<F: Float>(t154: F, t1995: F, t205: F, t12290: F, t3777: F, t12247: F, t551: F, t236: F, t3792: F, t10021: F, t1336: F, t1361: F) -> (F, F, F, F, F, F, F) {
        let (t40024, t40025, t40035, t40041, t40042, t40046, t40059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2041::<F>(t154, t1995, t205, t12290, t3777, t12247, t551, t236, t3792, t10021, t1336, t1361);
    (t40024, t40025, t40035, t40041, t40042, t40046, t40059)
}
