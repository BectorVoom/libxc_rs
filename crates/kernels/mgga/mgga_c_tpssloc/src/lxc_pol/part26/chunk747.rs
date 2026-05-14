//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 747/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk747<F: Float>(t19: F, t9223: F, t9211: F, t9213: F, t9215: F, t9217: F, t9219: F, t9221: F, t2233: F, t604: F, t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F) -> (F, F, F, F, F) {
    let t9225 = 0.75936e3 * t19 * t9223;
    let t9226 = -t9211 + t9213 - t9215 + t9217 - t9219 + t9221 - t9225;
    let t9228 = t2233 * t604;
    let t9231 = t601 * t2239;
    let t9238 = 1.0 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    (t9226, t9228, t9231, t9238, t9239)
}
