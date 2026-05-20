//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2048;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta549<F: Float>(t2229: F, t59: F, t60: F, t535: F, t9538: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t12248: F, t1372: F, t12019: F, t566: F, t68: F, t3700: F, t195: F, t632: F, t197: F, t636: F, t2531: F, t9892: F, t718: F, t9862: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40419, t40422, t40445, t40449, t40492) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2048::<F>(t2229, t59, t60, t535, t9538, t241, t6597, t248, t555, t557, t12248, t1372);
        let (t40591, t40611, t40632, t40647, t40667, t40673) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2049::<F>(t12019, t566, t68, t3700, t195, t632, t197, t636, t2531, t9892, t718, t9862);
    (t40419, t40422, t40445, t40449, t40492, t40591, t40611, t40632, t40647, t40667, t40673)
}
