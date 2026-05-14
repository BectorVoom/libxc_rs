//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1145/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1145<F: Float>(t33: F, t259: F, t479: F, t20576: F, t1289: F, t1826: F, t20631: F, t3431: F, t57: F, t581: F, t5889: F, t6393: F, t20584: F, t13119: F, t1845: F, t1163: F, t118: F, t1273: F, t1339: F, t1663: F, t1760: F, t1834: F, t18898: F, t20288: F, t20294: F, t20396: F, t20407: F, t3502: F, t3538: F, t3542: F, t4541: F, t485: F, t5706: F, t5801: F, t5905: F, t626: F, t6309: F, t6409: F, t6437: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t20632 = piecewise3(t480, 0.0, t20576);
    let t20639 = piecewise3(t386, t20631, -t5889 * t1289 / 2.0 - t1826 * t3431 / 2.0 + t20632 * t57 / 2.0 - t6393 * t581 / 2.0);
    let t20640 = t20584 + t20639;
    let t20642 = t1845 * t13119;
    let t20646 = -t1163 * t6309 - t118 * t20640 + t1273 * t6409 - 2.0 * t1339 * t18898 - 2.0 * t1339 * t20294 + t1663 * t5905 + 3.0 * t1760 * t20407 - t1760 * t20642 + t1834 * t4541 - t20288 * t485 - 2.0 * t20396 * t626 - 2.0 * t3502 * t5801 - 2.0 * t3538 * t5801 - 2.0 * t3542 * t5801 + t5706 * t6437;
    (t20632, t20640, t20642, t20646)
}
