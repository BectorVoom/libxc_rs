//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1106/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1106<F: Float>(t1268: F, t1659: F, t1333: F, t18394: F, t640: F, t18397: F, t3532: F, t5527: F, t1270: F, t4397: F, t1206: F, t197: F, t507: F, t1759: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19581 = t1659 * t1268;
    let t19588 = t18394 * t1333;
    let t19590 = t1333 * t640;
    let t19591 = t18397 * t19590;
    let t19593 = t5527 * t3532;
    let t19604 = t1270 * t4397;
    let t19609 = t1659 * t1206;
    let t19619 = t197 * t507;
    let t19620 = t1759 * t19619;
    (t19581, t19588, t19590, t19591, t19593, t19604, t19609, t19619, t19620)
}
