//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 962/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk962<F: Float>(t10117: F, t3277: F, t3346: F, t72: F, t240: F, t3245: F, t520: F, t3240: F, t3251: F, t3243: F, t756: F, t3247: F) -> (F, F, F, F, F, F, F) {
    let t10118 = t10117 * t3277;
    let t10120 = t3346 * t72;
    let t10121 = t10120 * t240;
    let t10122 = t520 * t3245;
    let t10131 = t3240 * t3251;
    let t10137 = t756 * t3243;
    let t10138 = t10137 * t3247;
    (t10118, t10120, t10121, t10122, t10131, t10137, t10138)
}
