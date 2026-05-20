//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta588<F: Float>(t5544: F, t868: F, t5527: F, t1484: F, t4303: F, t4233: F, t828: F, t1388: F, t6347: F, t1799: F, t5356: F, t1351: F) -> (F, F, F, F, F, F, F, F) {
        let (t67123, t67128, t67164, t67783, t67793, t74032, t74060, t74366) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1967::<F>(t5544, t868, t5527, t1484, t4303, t4233, t828, t1388, t6347, t1799, t5356, t1351);
    (t67123, t67128, t67164, t67783, t67793, t74032, t74060, t74366)
}
