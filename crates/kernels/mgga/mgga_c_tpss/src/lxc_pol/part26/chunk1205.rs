//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1205/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1205<F: Float>(t3032: F, t65: F, t4047: F, t1100: F, t4052: F, t4238: F, t6007: F, t1103: F, t19066: F, t19067: F, t19077: F, t19084: F, t20800: F, t20802: F, t20806: F, t20808: F, t4228: F, t4234: F, t4242: F, t4248: F, t6002: F) -> (F, F, F, F, F, F) {
    let t20809 = t65 * t3032;
    let t20810 = t20809 * t4047;
    let t20813 = t65 * t1100;
    let t20814 = t20813 * t4052;
    let t20821 = t6007 * t4238;
    let t20827 = -t20800 / 108.0 + t20802 * t1103 / 108.0 - t19066 - t19067 / 864.0 - t20806 / 864.0 + t20808 * t20810 / 216.0 - t20808 * t20814 / 144.0 - t6002 * t4228 / 288.0 + t19077 * t4234 / 768.0 + t20821 / 2304.0 - t19084 * t4242 / 2304.0 + t6007 * t4248 / 1536.0;
    (t20809, t20810, t20813, t20814, t20821, t20827)
}
