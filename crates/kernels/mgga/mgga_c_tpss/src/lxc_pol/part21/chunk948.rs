//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 948/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk948<F: Float>(t1019: F, t2861: F, t3153: F, t475: F, t126: F, t3096: F, t242: F, t2841: F, t1125: F, t3060: F, t3081: F, t3080: F, t215: F, t442: F, t68: F, t441: F) -> (F, F, F, F, F) {
    let t9507 = t1019 * t2861;
    let t9519 = 1.0 / t3153 / t475;
    let t9523 = t126 * t3096;
    let t9525 = t242 * t9523 * t2841;
    let t9526 = t1125 * t9525;
    let t9529 = t242 * t3060 * t3081;
    let t9530 = t3080 * t9529;
    let t9533 = t215 * t68 * t442;
    let t9535 = 5.0 / 1296.0 * t441 * t9533;
    (t9507, t9519, t9526, t9530, t9535)
}
