//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 495/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk495<F: Float>(t252: F, t2591: F, t798: F, t852: F, t225: F, t799: F, t154: F, t2559: F, t222: F, t2563: F, t805: F, t119: F, t2379: F) -> (F, F, F, F, F, F, F) {
    let t2592 = t2591 * t252;
    let t2594 = t798 * t852;
    let t2597 = t799 * t225;
    let t2600 = t2559 * t154;
    let t2602 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2605 = t119 * t2379;
    (t2592, t2594, t2597, t2600, t2602, t2603, t2605)
}
