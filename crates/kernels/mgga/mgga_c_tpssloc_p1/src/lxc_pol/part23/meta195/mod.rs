//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta195<F: Float>(t11282: F, t440: F, t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t11292: F, t432: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11310, t11314, t11317, t11349, t11350, t11352, t11365, t11369, t11372, t11419, t11420, t11444) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk833::<F>(t11282, t440, t11135, t11203, t1127, t3355, t427, t3358, t435, t11292, t432);
    (t11310, t11314, t11317, t11349, t11350, t11352, t11365, t11369, t11372, t11419, t11420, t11444)
}
