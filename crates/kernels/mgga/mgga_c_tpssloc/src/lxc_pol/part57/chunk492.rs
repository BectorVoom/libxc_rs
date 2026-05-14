//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 492/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk492<F: Float>(t6579: F, t1878: F, t229: F, t1891: F, t2230: F, t213: F, t1895: F, t202: F, t243: F, t598: F, t2229: F, t61: F, t133: F, t119: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6580 = 7.0 / 288.0 * t6579;
    let t6581 = t1878 * t229;
    let t6584 = t2230 * t1891;
    let t6585 = t6584 * t213;
    let t6586 = t6585 * t1895;
    let t6587 = 0.14130464632949136799e-2 * t6586;
    let t6589 = 1.0 / t243 / t202;
    let t6590 = t598 * t6589;
    let t6591 = t6590 * t213;
    let t6597 = 1.0 / t61 / t2229;
    let t6598 = t6597 * t1891;
    let t6599 = t6598 * t133;
    let t6600 = t119 * t212;
    let t6601 = t6600 * t1895;
    let t6602 = t6599 * t6601;
    let t6603 = 0.33643963411783659045e-4 * t6602;
    (t6580, t6581, t6584, t6585, t6586, t6587, t6589, t6590, t6591, t6597, t6598, t6599, t6600, t6601, t6602, t6603)
}
