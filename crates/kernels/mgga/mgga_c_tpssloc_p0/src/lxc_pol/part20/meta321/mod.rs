//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta321<F: Float>(t11543: F, t11597: F, t491: F, t1235: F, t3481: F, t1239: F, t68: F, t1251: F, t3599: F, t225: F, t3484: F, t3493: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11598, t11599, t11601, t11604, t11605, t11606, t11607, t11608, t11613, t11616, t11620) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1591::<F>(t11543, t11597, t491, t1235, t3481, t1239, t68, t1251, t3599, t225, t3484, t3493);
    (t11598, t11599, t11601, t11604, t11605, t11606, t11607, t11608, t11613, t11616, t11620)
}
