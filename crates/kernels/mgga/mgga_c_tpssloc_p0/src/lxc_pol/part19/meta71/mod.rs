//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta71 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk439;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta71<F: Float>(t42: F, t2244: F, t2250: F, t43: F, t54: F, t55: F, t240: F, t59: F, t2262: F, t39: F, t44: F, t51: F, t615: F, t618: F, t33: F) -> (F, F, F, F, F, F, F) {
        let (t2267, t2268, t2271, t2274, t2275, t2278, t2281) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk439::<F>(t42, t2244, t2250, t43, t54, t55, t240, t59);
        let (t2283, t2284) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk440::<F>(t2281, t2262, t2268, t2271, t2275, t2278, t39, t44, t51, t615, t618, t33);
    (t2267, t2268, t2271, t2274, t2281, t2283, t2284)
}
