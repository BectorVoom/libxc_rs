//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk225;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk226;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta30<F: Float>(t592: F, t14: F, t2: F, t21: F, t15: F, t583: F, t19: F, t582: F, t586: F, t589: F, t83: F, t85: F, t24: F, t4: F, t581: F, t25: F, t28: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t593, t594, t596, t597, t598) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk225::<F>(t592, t14, t2, t21, t15, t583);
        let (t601, t604, t605, t606) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk226::<F>(t19, t598, t582, t586, t589, t593, t596, t83, t85, t24, t4, t581);
        let t607 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk227::<F>(t25, t28, t606, zeta_threshold);
    (t594, t597, t598, t601, t604, t605, t606, t607)
}
