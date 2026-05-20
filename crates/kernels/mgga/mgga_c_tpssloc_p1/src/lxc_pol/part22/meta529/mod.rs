//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta529<F: Float>(t21: F, t9: F, t587: F, t598: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F, t601: F, t9238: F) -> (F, F, F, F, F, F) {
        let (t39033, t39035, t39037, t39039, t39043, t39054) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2000::<F>(t21, t9, t587, t598, t14, t2230, t594, t9223, t22811, t19, t601, t9238);
    (t39033, t39035, t39037, t39039, t39043, t39054)
}
