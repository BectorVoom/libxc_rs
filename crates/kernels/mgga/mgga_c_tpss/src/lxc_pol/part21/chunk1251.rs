//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1251/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1251<F: Float>(t1338: F, t18592: F, t547: F, t3537: F, t5772: F, t1279: F, t6293: F, t116: F, t6112: F, t645: F, t117: F, t19596: F, t1281: F, t1670: F, t1784: F, t20094: F, t20105: F, t20107: F, t20109: F, t20111: F, t20115: F, t4556: F, t4559: F, t548: F, t5766: F, t6284: F) -> (F, F, F, F, F, F) {
    let t20116 = t18592 * t1338;
    let t20118 = 6.0 * t547 * t20116;
    let t20119 = t5772 * t3537;
    let t20121 = 6.0 * t547 * t20119;
    let t20123 = 3.0 * t1279 * t6293;
    let t20124 = t116 * t6112;
    let t20125 = t20124 * t645;
    let t20127 = 6.0 * t547 * t20125;
    let t20128 = t117 * t19596;
    let t20130 = 3.0 * t547 * t20128;
    let t20131 = 3.0 * t1281 * t6284 + 3.0 * t1670 * t5766 + 6.0 * t1784 * t4556 + 3.0 * t1784 * t4559 + t20094 * t548 + t20105 + t20107 + t20109 + t20111 + t20115 + t20118 + t20121 + t20123 + t20127 + t20130;
    (t20116, t20119, t20124, t20125, t20128, t20131)
}
