//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta535<F: Float>(t1329: F, t80775: F, t22822: F, t281: F, t6924: F, t22794: F, t120: F, t22816: F, t22814: F, t22855: F, t22823: F, t3862: F, t6940: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t80776, t80779, t80780, t80782, t80783, t80784, t80791, t80792, t80794) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1749::<F>(t1329, t80775, t22822, t281, t6924, t22794, t120, t22816, t22814, t22855, t22823, t3862, t6940);
    (t80776, t80779, t80780, t80782, t80783, t80784, t80791, t80792, t80794)
}
