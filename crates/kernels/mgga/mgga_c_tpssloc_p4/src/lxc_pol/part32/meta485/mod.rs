//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta485<F: Float>(t7537: F, t865: F, t2718: F, t23204: F, t7488: F, t6562: F, t23168: F, t7480: F, t6547: F, t7489: F, t23237: F, t1880: F) -> (F, F, F, F, F, F, F) {
        let (t25200, t25205, t25206, t25209, t25211, t25213, t25214) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1791::<F>(t7537, t865, t2718, t23204, t7488, t6562, t23168, t7480, t6547, t7489, t23237, t1880);
    (t25200, t25205, t25206, t25209, t25211, t25213, t25214)
}
