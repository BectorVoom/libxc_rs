//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta351<F: Float>(t39344: F, t761: F, t39362: F, t2751: F, t39494: F, t153: F, t157: F, t39842: F, t2374: F, t39354: F, t39516: F, t39325: F) -> (F, F, F, F, F, F, F, F) {
        let (t40764, t40766, t40772, t40779, t40784, t40790, t40793, t40797) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1146::<F>(t39344, t761, t39362, t2751, t39494, t153, t157, t39842, t2374, t39354, t39516, t39325);
    (t40764, t40766, t40772, t40779, t40784, t40790, t40793, t40797)
}
