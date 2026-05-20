//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta625<F: Float>(t53782: F, t16169: F, t2663: F, t15892: F, t2371: F, t5154: F, t9919: F, t12344: F, t5234: F, t1369: F, t1831: F, t40059: F) -> (F, F, F, F, F, F, F) {
        let (t53783, t53788, t53797, t53798, t53880, t53882, t53901) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2159::<F>(t53782, t16169, t2663, t15892, t2371, t5154, t9919, t12344, t5234, t1369, t1831, t40059);
    (t53783, t53788, t53797, t53798, t53880, t53882, t53901)
}
