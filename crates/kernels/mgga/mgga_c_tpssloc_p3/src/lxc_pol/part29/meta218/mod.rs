//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1049;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta218<F: Float>(t1164: F, t4879: F, t1694: F, t3400: F, t1155: F, t3403: F, t1171: F, t1706: F, t1420: F, t972: F, t1709: F, t3431: F, t1174: F, t3439: F, t60: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4881, t4882, t4883, t4884, t4886, t4887, t4889) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1049::<F>(t1164, t4879, t1694, t3400, t1155, t3403, t1171, t1706, t1420, t972);
        let (t4897, t4899) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1050::<F>(t1709, t3431, t1174, t3439, t60);
    (t4881, t4882, t4883, t4884, t4886, t4887, t4889, t4897, t4899)
}
