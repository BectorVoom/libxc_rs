//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1275/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1275<F: Float>(t1586: F, t19150: F, t4322: F, t6032: F, t20920: F, t3154: F, t6527: F, t9519: F, t1906: F, t4543: F, t1665: F, t6071: F, t1901: F, t4562: F, t1284: F, t6547: F) -> (F, F, F, F, F, F, F, F) {
    let t68581 = t19150 * t1586;
    let t68585 = t6032 * t4322;
    let t68597 = t20920 * t3154;
    let t68601 = t6527 * t9519;
    let t68773 = 2.0 * t4543 * t1906;
    let t68776 = 2.0 * t1665 * t6071;
    let t68780 = 2.0 * t1901 * t4562;
    let t68782 = 2.0 * t6547 * t1284;
    (t68581, t68585, t68597, t68601, t68773, t68776, t68780, t68782)
}
