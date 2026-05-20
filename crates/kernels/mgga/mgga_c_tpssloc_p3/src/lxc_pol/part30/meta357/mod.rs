//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1398;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta357<F: Float>(t4734: F, t690: F, t4778: F, t699: F, t4725: F, t4730: F) -> (F, F, F, F, F, F, F) {
        let (t14704, t14705, t14710, t14711, t14720) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1398::<F>(t4734, t690, t4778, t699, t4725);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1399::<F>(t14720, t4730, t690);
    (t14704, t14705, t14710, t14711, t14720, t14721, t14722)
}
