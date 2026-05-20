//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta634<F: Float>(t39382: F, t761: F, t2531: F, t9713: F, t39302: F, t2371: F, t9716: F, t2447: F, t32: F, t31: F, t717: F, t607: F, t707: F, t9862: F) -> (F, F, F, F, F, F, F) {
        let (t41258, t41259, t41262, t41274, t41279, t41284, t41291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2419::<F>(t39382, t761, t2531, t9713, t39302, t2371, t9716, t2447, t32, t31, t717, t607, t707, t9862);
    (t41258, t41259, t41262, t41274, t41279, t41284, t41291)
}
