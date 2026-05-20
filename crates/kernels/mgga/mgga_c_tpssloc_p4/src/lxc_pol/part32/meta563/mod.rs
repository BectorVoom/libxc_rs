//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta563<F: Float>(t22960: F, t28248: F, t1408: F, t1484: F, t25: F, t5544: F, t5657: F, t6571: F, t6553: F, t1880: F, t1527: F, t25191: F) -> (F, F, F, F, F, F, F) {
        let (t28249, t28252, t28256, t28263, t28264, t28265, t28267) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1929::<F>(t22960, t28248, t1408, t1484, t25, t5544, t5657, t6571, t6553, t1880, t1527, t25191);
    (t28249, t28252, t28256, t28263, t28264, t28265, t28267)
}
