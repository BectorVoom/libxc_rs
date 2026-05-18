//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1290/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1290<F: Float>(t1235: F, t60706: F, t18450: F, t3334: F, t3329: F, t159: F, t7091: F, t1695: F, t510: F, t527: F, t5543: F, t3247: F) -> (F, F, F, F, F, F) {
    let t60707 = t60706 * t1235;
    let t60709 = t18450 * t3334;
    let t60713 = t18450 * t3329;
    let t60720 = t7091 * t159;
    let t60722 = t60720 * t510 * t1695;
    let t60724 = t5543 * t527;
    let t60725 = t60724 * t3247;
    (t60707, t60709, t60713, t60720, t60722, t60725)
}
