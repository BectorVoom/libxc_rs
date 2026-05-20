//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta675<F: Float>(t92: F, t9384: F, t100: F, t9398: F, t2341: F, t657: F, t4063: F, t591: F, t4053: F, t1406: F, t9238: F, t39031: F) -> (F, F, F, F, F, F, F) {
        let (t45697, t45707, t45717, t45751, t45762, t45844, t45870) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2480::<F>(t92, t9384, t100, t9398, t2341, t657, t4063, t591, t4053, t1406, t9238, t39031);
    (t45697, t45707, t45717, t45751, t45762, t45844, t45870)
}
