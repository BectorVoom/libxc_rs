//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1027/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1027<F: Float>(t115461: F, t115462: F, t115465: F, t120410: F, t124154: F, t124163: F, t127278: F, t127283: F, t127285: F, t127289: F, t127293: F, t127296: F, t127299: F) -> F {
    let t128625 = t127278 / F::new(768.0) + t124154 + t115461 + t127283 / F::new(384.0) - t127285 / F::new(384.0) - t127289 / F::new(768.0) - t127293 / F::new(768.0) + F::new(0.22608743412718618878e-1) * t120410 - t124163 + t115462 - F::new(0.16149102437656156341e-2) * t127296 + t115465 + F::new(0.32298204875312312682e-2) * t127299;
    t128625
}
