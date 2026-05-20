//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2380/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2380<F: Float>(t68498: F, t68500: F, t68502: F, t68504: F, t68506: F, t68509: F, t68511: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F) -> F {
    let t68532 = -F::new(0.60385e0) * t68498 + F::cast_from(0.24528888888888888889e-1_f64) * t68500 + F::new(0.5519e-1) * t68502 + F::new(0.33114e0) * t68504 - F::new(0.11038e0) * t68506 + F::new(0.58258125e1) * t68509 - F::cast_from(0.1237865625e0_f64) * t68511 - F::new(0.149013e1) * t68515 + F::new(0.49671e0) * t68518 + F::new(0.44152e0) * t68523 - F::new(0.11038e0) * t68527 - F::cast_from(0.8585111111111111111e-1_f64) * t68530;
    t68532
}
