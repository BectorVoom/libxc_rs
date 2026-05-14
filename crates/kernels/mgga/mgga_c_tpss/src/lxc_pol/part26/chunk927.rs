//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 927/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk927<F: Float>(t1108: F, t8550: F, t9605: F, t1106: F, t453: F, t3054: F, t450: F, t3049: F, t2845: F, t390: F, t1102: F, t672: F, t1098: F, t1127: F, t650: F, t1015: F, t242: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9607 = t8550 * t1108 * t9605;
    let t9614 = t1106 * t1106;
    let t9615 = 1.0 / t9614;
    let t9616 = t9615 * t453;
    let t9618 = t8550 * t9616 * t9605;
    let t9619 = t3054 * t450;
    let t9626 = t8550 * t3049 * t9605;
    let t9637 = 1.0 / t390 / t2845;
    let t9657 = t672 * t1102;
    let t9658 = t1098 * t9657;
    let t9666 = t650 * t1127;
    let t9668 = t242 * t9666 * t1015;
    (t9607, t9615, t9618, t9619, t9626, t9637, t9657, t9658, t9666, t9668)
}
