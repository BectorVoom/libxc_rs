//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk439;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk440;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta74<F: Float>(t2281: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t107: F, t106: F, t655: F, t94: F, t102: F, t177: F, t738: F, t745: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2282, t2289, t2291, t2296, t2298, t2327, t2331, t2341, t2349) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk439::<F>(t2281, t40, t632, t73, t52, t636, t76, t107, t106, t655, t94, t102);
        let (t2367, t2368) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk440::<F>(t177, t738);
        let t2369 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk441::<F>(t745);
    (t2282, t2289, t2291, t2296, t2298, t2327, t2331, t2341, t2349, t2367, t2368, t2369)
}
