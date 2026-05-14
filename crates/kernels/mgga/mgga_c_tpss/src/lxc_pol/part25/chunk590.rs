//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 590/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk590<F: Float>(t3462: F, t38: F, t1289: F, t2033: F, t3431: F, t608: F, t2040: F, t612: F, t581: F, t77: F, t1291: F, t1307: F, t1314: F, t3427: F, t3433: F, t3436: F, t3441: F, t583: F, t603: F, t616: F, t71: F, t85: F) -> (F, F, F, F, F, F) {
    let t3463 = t38 * t3462;
    let t3472 = t2033 * t1289;
    let t3475 = t608 * t3431;
    let t3477 = t2040 * t1289;
    let t3480 = t612 * t3431;
    let t3482 = 28.0 / 9.0 * t3472 * t581 - 4.0 / 3.0 * t3475 + 28.0 / 9.0 * t3477 * t581 + 4.0 / 3.0 * t3480;
    let t3483 = t77 * t3482;
    let t3486 = -t3427 * t85 / 12.0 - t3433 * t85 / 12.0 - t3436 * t85 / 12.0 - t1291 * t616 / 12.0 - t3441 * t85 / 12.0 + t3463 * t85 / 24.0 + t1307 * t616 / 24.0 - t583 * t1314 / 12.0 + t603 * t1314 / 24.0 + t71 * t3483 / 24.0;
    (t3463, t3472, t3477, t3482, t3483, t3486)
}
