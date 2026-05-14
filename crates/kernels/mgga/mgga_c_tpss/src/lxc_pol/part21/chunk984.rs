//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 984/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk984<F: Float>(t10652: F, t3632: F, t10623: F, t10630: F, t10632: F, t10635: F, t10638: F, t10642: F, t10644: F, t10648: F, t2147: F, t761: F, t797: F, t8127: F, t8131: F, t8133: F, t8168: F, t8171: F) -> (F,) {
    let t10654 = 7.0 / 1152.0 * t10652 * t3632;
    let t10656 = -t797 * t10623 / 768.0 - 35.0 / 1152.0 * t8127 - 119.0 / 1728.0 * t8131 + 7.0 / 1152.0 * t8133 + t10630 - t761 * t10632 / 48.0 - 35.0 / 216.0 * t10635 - t8171 * t10638 / 4.0 - t10642 + t2147 * t10644 / 8.0 + t2147 * t10648 / 16.0 - t10654 - 7.0 / 48.0 * t8168;
    (t10656,)
}
