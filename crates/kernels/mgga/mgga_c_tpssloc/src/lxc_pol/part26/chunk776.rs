//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 776/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk776<F: Float>(t116: F, t9700: F, t16: F, t2397: F, t9691: F, t693: F, t9694: F, t119: F, t133: F, t625: F, t9689: F, t9692: F, t9695: F, t9698: F, t739: F, t746: F) -> (F, F, F, F, F, F) {
    let t9701 = t9700 * t116;
    let t9702 = t9701 * t16;
    let t9704 = t2397 * t9691;
    let t9706 = t693 * t9694;
    let t9709 = t133 * t119 * t625;
    let t9711 = -0.34523333333333333333e1 * t9689 + 0.23015555555555555556e1 * t9692 - 0.26851481481481481482e1 * t9695 - 0.93932222222222222223e0 * t9698 + 0.73355e-1 * t9702 - 0.14671e0 * t9704 - 0.17116166666666666667e0 * t9706 - 0.36793333333333333333e0 * t9709;
    let t9713 = t739 * t9711 * t746;
    (t9702, t9704, t9706, t9709, t9711, t9713)
}
