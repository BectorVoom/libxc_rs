//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1287/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1287<F: Float>(t1233: F, t4459: F, t1232: F, t13763: F, t1268: F, t4519: F, t1625: F, t3202: F, t1206: F, t1364: F, t2436: F, t10514: F) -> (F, F, F, F, F, F) {
    let t43908 = t1233 * t4459;
    let t43933 = t13763 * t1232;
    let t43998 = t4519 * t1268;
    let t44045 = t1625 * t3202;
    let t44070 = t1206 * t4519;
    let t44169 = t2436 * t1364;
    let t44170 = t44169 * t10514;
    (t43908, t43933, t43998, t44045, t44070, t44170)
}
