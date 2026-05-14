//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1356/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1356<F: Float>(t1695: F, t6495: F, t22011: F, t762: F, t15526: F, t6007: F, t1103: F, t15257: F, t15262: F, t15582: F, t15596: F, t19084: F, t20802: F, t20808: F, t20809: F, t20814: F, t40574: F, t4228: F, t6002: F, t63268: F, t68408: F, t68423: F) -> (F, F) {
    let t73360 = t6495 * t1695;
    let t73367 = t22011 * t762;
    let t73373 = t6007 * t15526;
    let t73375 = t20808 * t20809 * t15262 / 36.0 - 7.0 / 648.0 * t20808 * t40574 * t15257 + t73360 * t20814 / 27.0 + t20802 * t4228 / 54.0 - t6002 * t15596 / 288.0 - 11.0 / 324.0 * t73367 * t1103 + t63268 + 5.0 / 3456.0 * t19084 * t15582 + t68408 / 648.0 + t68423 + t73373 / 2304.0;
    (t73360, t73375)
}
