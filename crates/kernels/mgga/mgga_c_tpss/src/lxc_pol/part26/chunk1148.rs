//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1148/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1148<F: Float>(t18434: F, t517: F, t5543: F, t1215: F, t1693: F, t527: F, t3255: F, t64: F, t234: F, t339: F, t5719: F, t789: F) -> (F, F, F, F, F, F, F) {
    let t18435 = 35.0 / 432.0 * t18434;
    let t18436 = t5543 * t517;
    let t18437 = t18436 * t1215;
    let t18439 = t1693 * t527;
    let t18444 = t3255 * t64;
    let t18446 = t339 * t18444 * t234;
    let t18450 = t339 * t5719 * t789;
    (t18435, t18436, t18437, t18439, t18444, t18446, t18450)
}
