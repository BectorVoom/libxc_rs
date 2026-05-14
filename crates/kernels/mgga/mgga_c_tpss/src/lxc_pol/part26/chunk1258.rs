//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1258/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1258<F: Float>(t789: F, t582: F, t7690: F, t1138: F, t1883: F, t61283: F, t5637: F, t9738: F, t19128: F, t6021: F, t8550: F, t9605: F, t9615: F, t19075: F, t1872: F, t9533: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t61976 = 1232.0 / 27.0 * t789;
    let t62019 = t7690 * t582;
    let t63200 = t1883 * t61283 * t1138;
    let t63219 = t1883 * t5637 * t9738;
    let t63237 = t6021 * t19128;
    let t63254 = t8550 * t9615 * sigma2 * t9605;
    let t63258 = t8550 * t19075 * t9605;
    let t63268 = 5.0 / 1296.0 * t1872 * t9533;
    (t61976, t62019, t63200, t63219, t63237, t63254, t63258, t63268)
}
