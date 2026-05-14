//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 747/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk747<F: Float>(t9212: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F, t2230: F, t594: F, t2229: F, t3: F, t19: F, t9211: F, t2233: F, t604: F, t2239: F, t601: F) -> (F, F, F, F, F, F, F) {
    let t9213 = 0.4332e2 * t9212;
    let t9214 = t9 * t591;
    let t9215 = 0.9288e2 * t9214;
    let t9216 = t587 * t21;
    let t9217 = 0.3912e3 * t9216;
    let t9218 = t14 * t598;
    let t9219 = 0.12804e4 * t9218;
    let t9220 = t594 * t2230;
    let t9221 = 0.170856e4 * t9220;
    let t9222 = t2229 * t3;
    let t9223 = 1.0 / t9222;
    let t9225 = 0.75936e3 * t19 * t9223;
    let t9226 = -t9211 + t9213 - t9215 + t9217 - t9219 + t9221 - t9225;
    let t9228 = t2233 * t604;
    let t9231 = t601 * t2239;
    (t9214, t9216, t9218, t9223, t9226, t9228, t9231)
}
