//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta519<F: Float>(t214: F, t3879: F, t22675: F, t22724: F, t22716: F, t6903: F, t22662: F, t22674: F, t6897: F, t22684: F, t6546: F, t22687: F) -> (F, F, F, F, F, F) {
        let (t80707, t80711, t80722, t80725, t80727, t80728) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1767::<F>(t214, t3879, t22675, t22724, t22716, t6903, t22662, t22674, t6897, t22684, t6546, t22687);
    (t80707, t80711, t80722, t80725, t80727, t80728)
}
