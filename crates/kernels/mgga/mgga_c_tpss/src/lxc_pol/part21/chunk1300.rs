//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1300/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1300<F: Float>(t18107: F, t3941: F, t11464: F, t11478: F, t11501: F, t11631: F, t11679: F, t11683: F, t1467: F, t18069: F, t18083: F, t3983: F, t5620: F, t61354: F, t61365: F, t61368: F, t61449: F, t64420: F) -> (F,) {
    let t64427 = t18107 * t3941 / 216.0;
    let t64428 = 5.0 / 3456.0 * t5620 * t11464 - 5.0 / 1152.0 * t5620 * t11478 + t61354 * t11501 / 1536.0 - t61365 / 1728.0 + t18069 * t11679 / 2304.0 + 5.0 / 6912.0 * t18069 * t11683 - t61449 * t11631 / 2304.0 - t64420 / 6912.0 - t18083 * t3983 / 216.0 + 19.0 / 864.0 * t61368 * t1467 - t64427;
    (t64428,)
}
