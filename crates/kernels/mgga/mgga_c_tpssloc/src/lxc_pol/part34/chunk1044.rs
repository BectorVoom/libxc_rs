//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1044/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1044<F: Float>(t1649: F, t5527: F, t1484: F, t5966: F, t5544: F, t20778: F, t28: F, t105731: F, t25927: F, t20947: F, t25891: F, t5660: F, t105762: F, t23788: F, t5664: F, t28248: F, t89992: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t106640 = t1649 * t5527;
    let t106647 = t5966 * t1484;
    let t106651 = t1649 * t5544;
    let t106655 = t28 * t20778;
    let t106671 = t25927 * t105731;
    let t106677 = t25891 * t20947;
    let t106686 = t1649 * t5660;
    let t106690 = t23788 * t105762;
    let t106699 = t1649 * t5664;
    let t106706 = t89992 * t28248;
    (t106640, t106647, t106651, t106655, t106671, t106677, t106686, t106690, t106699, t106706)
}
