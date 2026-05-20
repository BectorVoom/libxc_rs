//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta332<F: Float>(t39035: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F, t85: F, t24: F, t10276: F, t73: F, t11152: F, t76: F) -> (F, F, F, F, F, F, F, F) {
        let (t39036, t39037, t39038, t39040, t39043, t39063, t39096, t39114) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1100::<F>(t39035, t14, t2230, t594, t9223, t22811, t19, t85, t24, t10276, t73, t11152, t76);
    (t39036, t39037, t39038, t39040, t39043, t39063, t39096, t39114)
}
