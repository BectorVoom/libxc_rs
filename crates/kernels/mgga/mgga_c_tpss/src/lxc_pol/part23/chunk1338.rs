//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1338/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1338<F: Float>(t12535: F, t6002: F, t1100: F, t139: F, t20808: F, t4052: F, t20802: F, t3028: F, t1141: F, t1569: F, t2738: F, t11869: F, t11878: F, t11883: F, t11888: F, t11894: F, t11902: F, t11906: F, t12321: F, t12395: F, t12411: F, t12539: F, t19084: F, t20809: F, t20813: F, t3035: F, t3040: F, t3044: F, t3070: F, t40574: F, t63309: F, t63319: F, t63327: F, sigma2: F) -> (F,) {
    let t68476 = t6002 * t12535 / 432.0;
    let t68489 = t20808 * t139 * t1100 * t4052 / 216.0;
    let t68511 = t20802 * t3028 / 162.0;
    let t68522 = t1141 * sigma2 * t1569 * t2738;
    let t68525 = -t68476 - t20802 * t3035 / 81.0 - 7.0 / 648.0 * t20808 * t40574 * t11878 - t63319 / 432.0 + t63327 / 648.0 - t19084 * t12321 / 1152.0 - t68489 + t20808 * t20809 * t11888 / 108.0 + t20808 * t20809 * t11894 / 216.0 + t20808 * t20809 * t11883 / 36.0 - t20808 * t20813 * t11902 / 72.0 - t20808 * t20813 * t11906 / 144.0 - t20808 * t20813 * t11869 / 48.0 + t63309 * t12411 / 2304.0 + t68511 + t20802 * t3044 / 108.0 + t20802 * t3040 / 54.0 + 5.0 / 3456.0 * t19084 * t12395 - t19084 * t12539 / 1152.0 + t68522 * t3070 / 216.0;
    (t68525,)
}
