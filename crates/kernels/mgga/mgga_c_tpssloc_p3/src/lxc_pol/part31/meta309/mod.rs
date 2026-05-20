//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta309<F: Float>(t3355: F, t432: F, t427: F, t1094: F, t3263: F, t11135: F, t11203: F, t1176: F, t698: F, t1179: F, t1174: F, t135: F, t3439: F) -> (F, F, F, F, F, F, F, F) {
        let (t11420, t11424, t11444, t11459, t11487, t11529, t11531, t11539) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1198::<F>(t3355, t432, t427, t1094, t3263, t11135, t11203, t1176, t698, t1179, t1174, t135, t3439);
    (t11420, t11424, t11444, t11459, t11487, t11529, t11531, t11539)
}
