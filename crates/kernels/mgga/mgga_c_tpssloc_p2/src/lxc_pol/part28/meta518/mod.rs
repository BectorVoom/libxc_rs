//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta518<F: Float>(t22751: F, t22930: F, t22917: F, t22723: F, t22891: F, t22920: F, t117: F, t5247: F, t6559: F, t22674: F, t22686: F, t22663: F, t6883: F) -> (F, F, F, F, F, F, F) {
        let (t80665, t80667, t80670, t80671, t80681, t80683, t80689) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1766::<F>(t22751, t22930, t22917, t22723, t22891, t22920, t117, t5247, t6559, t22674, t22686, t22663, t6883);
    (t80665, t80667, t80670, t80671, t80681, t80683, t80689)
}
