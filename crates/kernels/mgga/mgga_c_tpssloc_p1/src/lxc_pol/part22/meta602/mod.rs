//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta602<F: Float>(t10701: F, t1543: F, t10810: F, t1561: F, t47705: F, t47707: F, t48096: F, t47730: F, t48155: F, t48157: F, t2929: F, t4446: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t49274, t49285, t49304, t49306, t49317, t49322, t49378, t49379, t49411) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2124::<F>(t10701, t1543, t10810, t1561, t47705, t47707, t48096, t47730, t48155, t48157, t2929, t4446);
    (t49274, t49285, t49304, t49306, t49317, t49322, t49378, t49379, t49411)
}
