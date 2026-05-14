//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1198/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1198<F: Float>(t20124: F, t645: F, t547: F, t117: F, t19596: F, t19466: F, t19479: F, t19491: F, t19588: F, t19693: F, t19706: F, t19718: F, t1864: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20125 = t20124 * t645;
    let t20127 = 6.0 * t547 * t20125;
    let t20128 = t117 * t19596;
    let t20130 = 3.0 * t547 * t20128;
    let t20142 = 7.0 / 72.0 * t19466;
    let t20146 = 7.0 / 1152.0 * t19479;
    let t20151 = 7.0 / 288.0 * t19491;
    let t20315 = 2.0 / 3.0 * t19588;
    let t20434 = 7.0 / 72.0 * t19693;
    let t20438 = 7.0 / 1152.0 * t19706;
    let t20443 = 7.0 / 288.0 * t19718;
    let t20706 = t1864 * t645;
    (t20125, t20127, t20128, t20130, t20142, t20146, t20151, t20315, t20434, t20438, t20443, t20706)
}
