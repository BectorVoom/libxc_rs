//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta629<F: Float>(t22960: F, t59580: F, t1408: F, t2745: F, t25365: F, t81547: F, t1530: F, t2553: F, t12971: F, t25: F, t2379: F, t4255: F, t606: F, t870: F) -> (F, F, F, F, F, F, F, F) {
        let (t86803, t86806, t86810, t86815, t86816, t86821, t86825, t86830) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2118::<F>(t22960, t59580, t1408, t2745, t25365, t81547, t1530, t2553, t12971, t25, t2379, t4255, t606, t870);
    (t86803, t86806, t86810, t86815, t86816, t86821, t86825, t86830)
}
