//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta594<F: Float>(t23967: F, t26090: F, t23993: F, t7435: F, t46104: F, t7025: F, t26055: F, t7032: F, t26063: F, t7432: F, t84241: F, t45844: F) -> (F, F, F, F, F, F, F) {
        let (t91904, t91905, t91907, t91913, t91921, t91922, t91954) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1839::<F>(t23967, t26090, t23993, t7435, t46104, t7025, t26055, t7032, t26063, t7432, t84241, t45844);
    (t91904, t91905, t91907, t91913, t91921, t91922, t91954)
}
