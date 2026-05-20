//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta575<F: Float>(t3355: F, t427: F, t3358: F, t11292: F, t1143: F, t1124: F, t11419: F, t11282: F, t43689: F, t440: F, t43776: F, t43819: F) -> (F, F, F, F, F, F, F, F) {
        let (t44177, t44179, t44205, t44214, t44220, t44223, t44249, t44275) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2084::<F>(t3355, t427, t3358, t11292, t1143, t1124, t11419, t11282, t43689, t440, t43776, t43819);
    (t44177, t44179, t44205, t44214, t44220, t44223, t44249, t44275)
}
