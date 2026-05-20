//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta510<F: Float>(t1509: F, t2678: F, t1484: F, t2631: F, t9975: F, t2710: F, t4233: F, t852: F, t13170: F, t252: F, t1519: F, t13068: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46693, t47012, t47262, t47285, t47425, t47439, t47448, t47528, t47568) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1758::<F>(t1509, t2678, t1484, t2631, t9975, t2710, t4233, t852, t13170, t252, t1519, t13068, t225);
    (t46693, t47012, t47262, t47285, t47425, t47439, t47448, t47528, t47568)
}
