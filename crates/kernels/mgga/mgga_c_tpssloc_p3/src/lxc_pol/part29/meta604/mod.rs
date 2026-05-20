//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2040;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta604<F: Float>(t22998: F, t23185: F, t81914: F, t6604: F, t9971: F, t23110: F, t23180: F, t206: F, t22723: F, t268: F, t23186: F, t23163: F, t23165: F, t1879: F, t80845: F, t1906: F, t23229: F, t81715: F, t225: F, t23226: F, t23228: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82016, t82018, t82028, t82031, t82032, t82038) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2040::<F>(t22998, t23185, t81914, t6604, t9971, t23110, t23180, t206, t22723, t268, t23186, t23163);
        let (t82039, t82045, t82047, t82070, t82071, t82074) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2041::<F>(t23165, t82038, t1879, t80845, t1906, t23229, t81715, t225, t23226, t23228);
    (t82016, t82018, t82028, t82031, t82032, t82038, t82039, t82045, t82047, t82070, t82071, t82074)
}
