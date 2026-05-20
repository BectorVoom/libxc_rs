//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1893;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta596<F: Float>(t2307: F, t72: F, t7431: F, t1437: F, t6509: F, t1864: F, t4021: F, t1410: F, t9231: F, t2240: F, t3961: F, t3967: F, t12571: F, t608: F, t33: F, t46099: F, t2244: F, t3953: F, t9239: F, t2241: F, t12648: F, t605: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90080, t90090, t90094, t90098, t90101, t90104) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1893::<F>(t2307, t72, t7431, t1437, t6509, t1864, t4021, t1410, t9231, t2240, t3961, t3967);
        let (t90114, t90121, t90132, t90137, t90141, t90150) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1894::<F>(t12571, t608, t33, t46099, t2244, t3953, t1410, t9239, t2241, t72, t7431, t12648, t605);
    (t90080, t90090, t90094, t90098, t90101, t90104, t90114, t90121, t90132, t90137, t90141, t90150)
}
