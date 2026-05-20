//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta530<F: Float>(t5611: F, t852: F, t17100: F, t225: F, t17087: F, t17060: F, t17095: F, t17098: F, t112: F, t20148: F, t5544: F, t868: F) -> (F, F, F, F, F, F, F, F) {
        let (t59331, t59466, t59498, t59503, t59519, t59537, t66958, t67123) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1744::<F>(t5611, t852, t17100, t225, t17087, t17060, t17095, t17098, t112, t20148, t5544, t868);
    (t59331, t59466, t59498, t59503, t59519, t59537, t66958, t67123)
}
