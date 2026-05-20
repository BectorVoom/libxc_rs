//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta545<F: Float>(t2707: F, t9993: F, t2642: F, t9612: F, t9638: F, t9649: F, t2678: F, t828: F, t786: F, t9569: F, t805: F, t2610: F, t9541: F) -> (F, F, F, F, F, F, F) {
        let (t41055, t41063, t41066, t41078, t41083, t41084, t41086) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2086::<F>(t2707, t9993, t2642, t9612, t9638, t9649, t2678, t828, t786, t9569, t805, t2610, t9541);
    (t41055, t41063, t41066, t41078, t41083, t41084, t41086)
}
