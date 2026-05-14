//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 938/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk938<F: Float>(t555: F, t7622: F, t123: F, t1354: F, t2349: F, t3645: F, t725: F, t1352: F, t2332: F, t2206: F, t3557: F, t2215: F, t3431: F, t681: F, t2112: F, t3642: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10350 = 2.0 * t555;
    let t10351 = 6.0 * t7622;
    let t10510 = t1354 * t123;
    let t10511 = t10510 * t2349;
    let t10520 = 2.0 * t3645 * t725;
    let t10521 = t1352 * t2332;
    let t10558 = t3557 * t2206;
    let t10560 = t3557 * t2215;
    let t10564 = t725 * t3431;
    let t10566 = 8.0 * t681 * t10564;
    let t10568 = 8.0 * t2112 * t3642;
    (t10350, t10351, t10511, t10520, t10521, t10558, t10560, t10566, t10568)
}
