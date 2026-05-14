//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 950/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk950<F: Float>(t3243: F, t756: F, t3247: F, t1246: F, t159: F, t210: F, t3234: F, t520: F, t1212: F, t2139: F, t1215: F, t242: F, t527: F, t8200: F, t525: F, t219: F, t3358: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10137 = t756 * t3243;
    let t10138 = t10137 * t3247;
    let t10140 = t159 * t1246;
    let t10141 = t210 * t10140;
    let t10151 = t520 * t3234;
    let t10160 = t2139 * t1212;
    let t10161 = t10160 * t1215;
    let t10164 = t8200 * t527 * t242;
    let t10166 = 595.0 / 10368.0 * t525 * t10164;
    let t10171 = t3358 * t219;
    (t10137, t10138, t10141, t10151, t10160, t10161, t10164, t10166, t10171)
}
