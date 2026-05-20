//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk730;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk731;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta127<F: Float>(t2296: F, t1094: F, t1098: F, t1097: F, t419: F, t409: F, t407: F, t410: F, t3236: F, t281: F, t2820: F, t415: F, t1114: F, t699: F, t1176: F, t241: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3247 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk730::<F>(t2296);
        let (t3259, t3263, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3295) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk731::<F>(t1094, t1098, t1097, t419, t409, t407, t410, t3236, t281, t2820, t415, t1114, t699);
        let t3297 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk732::<F>(t1176, t241);
    (t3247, t3259, t3263, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3295, t3297)
}
