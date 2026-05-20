//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta665<F: Float>(t27495: F, t85964: F, t15702: F, t8038: F, t85822: F, t27563: F, t85639: F, t24826: F, t27502: F, t27558: F, t7368: F, t94490: F) -> (F, F, F, F, F, F) {
        let (t94874, t94881, t94885, t94889, t94891, t94901) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2097::<F>(t27495, t85964, t15702, t8038, t85822, t27563, t85639, t24826, t27502, t27558, t7368, t94490);
    (t94874, t94881, t94885, t94889, t94891, t94901)
}
