//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 889/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk889<F: Float>(t3138: F, t9555: F, t294: F, t2966: F, t458: F, t8556: F, t1108: F, t8550: F, t1106: F, t453: F, t3054: F, t450: F, t3049: F, t140: F, t3034: F, t1098: F) -> (F, F, F, F, F, F, F, F) {
    let t9573 = t3138 * t9555;
    let t9589 = t294 * t2966;
    let t9605 = t458 * t8556;
    let t9607 = t8550 * t1108 * t9605;
    let t9614 = t1106 * t1106;
    let t9615 = 1.0 / t9614;
    let t9616 = t9615 * t453;
    let t9618 = t8550 * t9616 * t9605;
    let t9619 = t3054 * t450;
    let t9626 = t8550 * t3049 * t9605;
    let t9632 = t140 * t3034;
    let t9633 = t1098 * t9632;
    (t9573, t9589, t9607, t9615, t9618, t9619, t9626, t9633)
}
