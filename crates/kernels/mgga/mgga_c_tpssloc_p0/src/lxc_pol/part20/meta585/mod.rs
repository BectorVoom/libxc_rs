//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta585<F: Float>(t3199: F, t42741: F, t1057: F, t42754: F, t10474: F, t42340: F, t42341: F, t10482: F, t23508: F, t11045: F, t42332: F, t43288: F) -> (F, F, F, F, F, F) {
        let (t43536, t43542, t43553, t43554, t43562, t43576) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2155::<F>(t3199, t42741, t1057, t42754, t10474, t42340, t42341, t10482, t23508, t11045, t42332, t43288);
    (t43536, t43542, t43553, t43554, t43562, t43576)
}
