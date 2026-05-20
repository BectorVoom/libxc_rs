//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta352<F: Float>(t2374: F, t39497: F, t39500: F, t39506: F, t10108: F, t257: F, t68: F, t233: F, t9970: F, t252: F, t2632: F, t10021: F, t812: F, t841: F) -> (F, F, F, F, F, F, F, F) {
        let (t40799, t40801, t40803, t40890, t40931, t40932, t40933, t40965) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1147::<F>(t2374, t39497, t39500, t39506, t10108, t257, t68, t233, t9970, t252, t2632, t10021, t812, t841);
    (t40799, t40801, t40803, t40890, t40931, t40932, t40933, t40965)
}
