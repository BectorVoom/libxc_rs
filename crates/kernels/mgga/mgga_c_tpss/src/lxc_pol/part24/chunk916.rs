//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 916/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk916<F: Float>(t2912: F, t407: F, t1019: F, t2910: F, t2861: F, t3153: F, t475: F, t126: F, t3096: F, t215: F, t442: F, t68: F, t441: F, t461: F, t650: F, t1114: F, t242: F) -> (F, F, F, F, F, F, F, F) {
    let t9495 = 1.0 / t2912 / t407;
    let t9504 = t1019 * t2910;
    let t9507 = t1019 * t2861;
    let t9519 = 1.0 / t3153 / t475;
    let t9523 = t126 * t3096;
    let t9533 = t215 * t68 * t442;
    let t9535 = 5.0 / 1296.0 * t441 * t9533;
    let t9540 = t650 * t461;
    let t9542 = t242 * t9540 * t1114;
    (t9495, t9504, t9507, t9519, t9523, t9535, t9540, t9542)
}
