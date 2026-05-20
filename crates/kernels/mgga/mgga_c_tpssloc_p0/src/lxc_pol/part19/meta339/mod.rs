//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1208;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta339<F: Float>(t2632: F, t40925: F, t233: F, t9970: F, t252: F, t9975: F, t2678: F, t852: F, t2703: F, t9993: F, t2696: F, t9612: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40926, t40931, t40932, t40934, t40938, t40951, t40955, t40959, t40961) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1208::<F>(t2632, t40925, t233, t9970, t252, t9975, t2678, t852, t2703, t9993, t2696, t9612);
    (t40926, t40931, t40932, t40934, t40938, t40951, t40955, t40959, t40961)
}
