//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta559<F: Float>(t13036: F, t225: F, t4119: F, t828: F, t1484: F, t2678: F, t1509: F, t2631: F, t9975: F, t2710: F, t4233: F, t852: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46508, t46565, t46644, t46693, t47012, t47262, t47285, t47425, t47439) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2002::<F>(t13036, t225, t4119, t828, t1484, t2678, t1509, t2631, t9975, t2710, t4233, t852);
    (t46508, t46565, t46644, t46693, t47012, t47262, t47285, t47425, t47439)
}
