//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 917/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk917<F: Float>(t1111: F, t9542: F, t3065: F, t8507: F, t3124: F, t3090: F, t774: F, t3138: F, t458: F, t8556: F, t1108: F, t8550: F, t1106: F, t453: F, t3054: F, t450: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9543 = t1111 * t9542;
    let t9555 = t3065 * t8507;
    let t9556 = t3124 * t9555;
    let t9561 = t774 * t3090;
    let t9573 = t3138 * t9555;
    let t9605 = t458 * t8556;
    let t9607 = t8550 * t1108 * t9605;
    let t9614 = t1106 * t1106;
    let t9615 = 1.0 / t9614;
    let t9616 = t9615 * t453;
    let t9618 = t8550 * t9616 * t9605;
    let t9619 = t3054 * t450;
    (t9543, t9556, t9561, t9573, t9605, t9607, t9615, t9618, t9619)
}
