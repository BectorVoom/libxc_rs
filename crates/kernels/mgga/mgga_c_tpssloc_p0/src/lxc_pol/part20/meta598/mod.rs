//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta598<F: Float>(t3475: F, t4899: F, t11545: F, t135: F, t11548: F, t1174: F, t3439: F, t698: F, t3442: F, t11588: F, t3447: F, t3451: F) -> (F, F, F, F, F, F, F) {
        let (t44558, t44562, t44564, t44571, t44573, t44579, t44581) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2178::<F>(t3475, t4899, t11545, t135, t11548, t1174, t3439, t698, t3442, t11588, t3447, t3451);
    (t44558, t44562, t44564, t44571, t44573, t44579, t44581)
}
