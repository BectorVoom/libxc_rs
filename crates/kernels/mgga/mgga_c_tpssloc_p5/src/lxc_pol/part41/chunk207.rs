//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 207/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk207<F: Float>(t31: F, t607: F, t65: F, t34: F, t36: F, rho0: F, sigma0: F) -> (F, F, F) {
    let t608 = t31 * t607;
    let t609 = t608 * t65;
    let t612 = t34 * rho0;
    let t614 = F::new(1.0) / t36 / t612;
    let t615 = sigma0 * t614;
    (t608, t609, t615)
}
