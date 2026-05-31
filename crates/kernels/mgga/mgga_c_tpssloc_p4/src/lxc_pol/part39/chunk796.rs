//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 796/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk796<F: Float>(t4347: F, t882: F, t123: F, t2765: F, t2766: F, t4335: F, t4340: F, t4345: F, t291: F, t1543: F, t892: F, t914: F) -> (F, F, F, F, F, F) {
    let t4348 = t882 * t4347;
    let t4349 = t123 * t4348;
    let t4351 = t2765 + F::cast_from(0.5936111111111111111e-2_f64) * t2766 + F::cast_from(0.5936111111111111111e-2_f64) * t4335 - F::cast_from(0.11872222222222222222e-1_f64) * t4340 + F::cast_from(0.35616666666666666666e-1_f64) * t4345 - F::cast_from(0.17808333333333333333e-1_f64) * t4349;
    let t4353 = F::cast_from(0.621814e-1_f64) * t4351 * t291;
    let t4354 = t1543 * t892;
    let t4356 = F::cast_from(1.0_f64) * t4354 * t914;
    (t4348, t4349, t4351, t4353, t4354, t4356)
}
