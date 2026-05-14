//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1274/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1274<F: Float>(t1781: F, t3413: F, t1284: F, t5760: F, t18562: F, t550: F, t1279: F, t20116: F, t1668: F, t18593: F, t20119: F, t3403: F, t6293: F, t5705: F, t7309: F, t19582: F) -> (F, F, F, F, F, F, F, F) {
    let t62165 = t1781 * t3413;
    let t62167 = t5760 * t1284;
    let t62169 = t18562 * t550;
    let t63699 = 12.0 * t1279 * t20116;
    let t63701 = 12.0 * t1668 * t18593;
    let t63703 = 12.0 * t1279 * t20119;
    let t63705 = 3.0 * t3403 * t6293;
    let t63710 = t5705 * t7309;
    let t63712 = 4.0 * t63710 * t19582;
    (t62165, t62167, t62169, t63699, t63701, t63703, t63705, t63712)
}
