//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2542/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2542<F: Float>(t2860: F, t4408: F, t10770: F, t1561: F, t10811: F, t1568: F, t14255: F, t892: F, t2791: F, t4351: F, t10660: F, t1543: F) -> (F, F, F, F, F, F) {
    let t49427 = t4408 * t2860;
    let t49430 = t1561 * t10770;
    let t49478 = t10811 * t1568;
    let t49483 = t14255 * t892;
    let t49486 = t4351 * t2791;
    let t49489 = t1543 * t10660;
    (t49427, t49430, t49478, t49483, t49486, t49489)
}
