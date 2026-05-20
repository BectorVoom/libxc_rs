//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta539<F: Float>(t10140: F, t10143: F, t2374: F, t39354: F, t39516: F, t9879: F, t9885: F, t39325: F, t39497: F, t39500: F, t39506: F, t9882: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40785, t40790, t40793, t40794, t40797, t40799, t40801, t40803, t40804) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2080::<F>(t10140, t10143, t2374, t39354, t39516, t9879, t9885, t39325, t39497, t39500, t39506, t9882);
    (t40785, t40790, t40793, t40794, t40797, t40799, t40801, t40803, t40804)
}
