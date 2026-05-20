//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta683<F: Float>(t28160: F, t6883: F, t19873: F, t26309: F, t19966: F, t6396: F, t80816: F, t19951: F, t22833: F, t19972: F, t19976: F, t5259: F, t91100: F) -> (F, F, F, F, F, F, F, F) {
        let (t97200, t97202, t97204, t97206, t97208, t97210, t97212, t97214) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2152::<F>(t28160, t6883, t19873, t26309, t19966, t6396, t80816, t19951, t22833, t19972, t19976, t5259, t91100);
    (t97200, t97202, t97204, t97206, t97208, t97210, t97212, t97214)
}
