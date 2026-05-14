//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1185/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1185<F: Float>(t1695: F, t17942: F, t510: F, t517: F, t5543: F, t1215: F, t1693: F, t527: F, t3247: F, t3251: F, t5716: F, t3255: F, t64: F, t234: F, t339: F, t3263: F) -> (F, F, F, F, F, F, F, F) {
    let t18434 = t17942 * t510 * t1695;
    let t18435 = 35.0 / 432.0 * t18434;
    let t18436 = t5543 * t517;
    let t18437 = t18436 * t1215;
    let t18438 = 7.0 / 72.0 * t18437;
    let t18439 = t1693 * t527;
    let t18440 = t18439 * t3247;
    let t18442 = t5716 * t3251;
    let t18444 = t3255 * t64;
    let t18446 = t339 * t18444 * t234;
    let t18447 = t18446 * t3263;
    (t18435, t18436, t18437, t18438, t18440, t18442, t18444, t18447)
}
