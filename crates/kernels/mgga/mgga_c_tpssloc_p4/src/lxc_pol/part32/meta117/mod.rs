//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta117<F: Float>(t891: F, t275: F) -> (F, F, F) {
        let (t2840, t2841, t2842) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk699::<F>(t891, t275);
    (t2840, t2841, t2842)
}
