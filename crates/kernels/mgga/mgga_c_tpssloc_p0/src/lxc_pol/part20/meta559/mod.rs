//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta559<F: Float>(t42101: F, t10619: F, t942: F, t2928: F, t315: F, t2931: F, t10843: F, t923: F, t2853: F, t2885: F, t10523: F, t938: F) -> (F, F, F, F, F, F, F, F) {
        let (t42102, t42106, t42110, t42111, t42113, t42117, t42123, t42128) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2115::<F>(t42101, t10619, t942, t2928, t315, t2931, t10843, t923, t2853, t2885, t10523, t938);
    (t42102, t42106, t42110, t42111, t42113, t42117, t42123, t42128)
}
