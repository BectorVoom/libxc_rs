//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1172/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1172<F: Float>(t114: F, t1268: F, t1659: F, t19580: F, t19579: F, t4525: F, t5754: F, t1760: F, t1333: F, t18394: F, t640: F, t18397: F, t3532: F, t5527: F, t18393: F, t18395: F) -> (F, F, F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t19581 = t1659 * t1268;
    let t19582 = t19580 * t19581;
    let t19584 = 2.0 * t19579 * t19582;
    let t19585 = t5754 * t4525;
    let t19586 = t1760 * t19585;
    let t19588 = t18394 * t1333;
    let t19590 = t1333 * t640;
    let t19591 = t18397 * t19590;
    let t19593 = t5527 * t3532;
    let t19596 = piecewise3(t115, 0.0, t18393 + t18395 / 3.0 + t19588 / 3.0 + t19591 / 4.0 - t19593 / 8.0);
    (t19581, t19582, t19584, t19585, t19586, t19588, t19590, t19596)
}
