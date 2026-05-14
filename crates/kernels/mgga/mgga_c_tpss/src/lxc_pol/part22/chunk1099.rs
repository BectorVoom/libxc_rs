//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1099/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1099<F: Float>(t114: F, t1270: F, t3234: F, t1799: F, t3166: F, t18392: F, t18395: F, t18398: F, t18400: F) -> (F, F, F, F) {
    let t115 = 1.0 < t114;
    let t18551 = t1270 * t3234;
    let t18613 = t3166 * t1799;
    let t18622 = 22.0 / 9.0 * t18392;
    let t18627 = piecewise3(t115, 0.0, t18622 + 4.0 / 3.0 * t18395 + t18398 / 2.0 - t18400 / 4.0);
    (t18551, t18613, t18622, t18627)
}
