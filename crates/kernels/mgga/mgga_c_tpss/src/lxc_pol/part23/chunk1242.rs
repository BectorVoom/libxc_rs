//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1242/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1242<F: Float>(t1980: F, t3416: F, t1286: F, t7689: F, t1321: F, t2105: F, t3490: F, t645: F, t1268: F, t4397: F, t1233: F, t4459: F, t1232: F, t13763: F, t4519: F, t1625: F, t3202: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42178 = t3416 * t1980;
    let t42181 = t1286 * t7689;
    let t42336 = t1321 * t2105;
    let t42719 = t3490 * t645;
    let t42962 = t4397 * t1268;
    let t43908 = t1233 * t4459;
    let t43933 = t13763 * t1232;
    let t43998 = t4519 * t1268;
    let t44045 = t1625 * t3202;
    (t42178, t42181, t42336, t42719, t42962, t43908, t43933, t43998, t44045)
}
