//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk612;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta98<F: Float>(t2251: F, t65: F, t608: F, t628: F, t36: F, t365: F, t42: F, t2244: F, t2250: F, t43: F, t54: F, t55: F, sigma0: F, t240: F, t59: F, t39: F, t44: F, t51: F, t615: F, t618: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2252, t2255, t2262, t2267, t2268, t2271, t2274, t2275, t2278) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk612::<F>(t2251, t65, t608, t628, t36, t365, t42, t2244, t2250, t43, t54, t55, sigma0);
        let (t2281, t2282, t2283) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk613::<F>(t240, t59, t2262, t2268, t2271, t2275, t2278, t39, t44, t51, t615, t618);
    (t2252, t2255, t2262, t2267, t2274, t2275, t2278, t2281, t2282, t2283)
}
