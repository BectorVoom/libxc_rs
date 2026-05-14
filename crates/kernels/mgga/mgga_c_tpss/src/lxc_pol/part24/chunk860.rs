//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 860/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk860<F: Float>(t1398: F, t30: F, t1288: F, t1692: F, t1713: F, t2439: F, t5590: F, t6121: F, t6149: F, t1461: F, t1467: F, t1471: F, t5604: F, t5605: F, t5610: F, t5618: F, t5620: F) -> (F, F, F) {
    let t6153 = t30 * t1398;
    let t6160 = 3.0 / 2.0 * t2439 * t6121 + t1692 * t6149 * t30 / 2.0 - t1692 * t5590 * t6153 / 2.0 + t1692 * t1713 * t1288 / 2.0;
    let t6167 = t5604 + t5605 * t1461 / 288.0 + t5610 * t1467 / 1536.0 + t5618 + t5620 * t1471 / 2304.0;
    (t6153, t6160, t6167)
}
