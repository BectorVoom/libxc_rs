//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1284/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1284<F: Float>(t19: F, t9223: F, t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F) -> (F, F, F, F) {
    let t9225 = F::cast_from(0.75936e3_f64) * t19 * t9223;
    let t9231 = t601 * t2239;
    let t9238 = F::cast_from(1.0_f64) / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    (t9225, t9231, t9238, t9239)
}
