//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta620<F: Float>(t24574: F, t24860: F, t24594: F, t24847: F, t974: F, t27551: F, t7327: F, t135: F, t7284: F, t24853: F, t24778: F, t24762: F) -> (F, F, F, F, F, F, F) {
        let (t86073, t86076, t86077, t86094, t86095, t86106, t86113) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2062::<F>(t24574, t24860, t24594, t24847, t974, t27551, t7327, t135, t7284, t24853, t24778, t24762);
    (t86073, t86076, t86077, t86094, t86095, t86106, t86113)
}
