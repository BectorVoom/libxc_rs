//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1441/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1441<F: Float>(t120410: F, t120416: F, t114013: F, t114031: F, t114035: F, t114046: F, t115461: F, t115462: F, t115465: F, t120388: F, t120393: F, t120395: F, t120397: F, t120399: F, t120401: F, t120405: F, t120408: F, t120413: F, t120419: F) -> F {
    let t122432 = F::cast_from(0.11304371706359309439e-1_f64) * t120410;
    let t122434 = F::new(7.0) / F::new(1152.0) * t120416;
    let t122438 = F::cast_from(0.32298204875312312682e-2_f64) * t120388 + t114013 + F::cast_from(0.16149102437656156341e-2_f64) * t120393 + t120395 / F::new(192.0) - t120397 / F::new(768.0) + t120399 / F::new(192.0) + t120401 / F::new(384.0) + t115461 - F::cast_from(0.96894614625936938046e-2_f64) * t120405 - F::cast_from(0.16149102437656156341e-2_f64) * t120408 + t122432 + t120413 / F::new(768.0) - t122434 + F::cast_from(0.67826230238155856632e-1_f64) * t120419 + t115462 + F::cast_from(0.16149102437656156341e-2_f64) * t114031 - t114035 + t115465 + F::cast_from(0.26915170729426927235e-3_f64) * t114046;
    t122438
}
