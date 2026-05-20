//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk227;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk228;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk229;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta31<F: Float>(t588: F, t15: F, t3: F, t14: F, t2: F, t21: F, t583: F, t19: F, t582: F, t586: F, t83: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t589, t590, t591) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk227::<F>(t588, t15, t3);
        let t592 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk228::<F>(t14, t591);
        let (t593, t594, t596, t597, t598) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk229::<F>(t592, t14, t2, t21, t15, t583);
        let (t600, t601, t604) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk230::<F>(t19, t598, t582, t586, t589, t593, t596, t83, t85);
    (t589, t590, t591, t592, t593, t594, t596, t597, t598, t600, t601, t604)
}
