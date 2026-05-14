//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1217/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1217<F: Float>(t1786: F, t4549: F, t1668: F, t5773: F, t5776: F, t1279: F, t6290: F, t1688: F, t645: F, t1338: F, t547: F, t18592: F, t3537: F, t5772: F, t6293: F, t116: F, t6112: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20105 = 3.0 * t4549 * t1786;
    let t20107 = 6.0 * t1668 * t5773;
    let t20109 = 3.0 * t1668 * t5776;
    let t20111 = 6.0 * t1279 * t6290;
    let t20112 = t645 * t1688;
    let t20113 = t20112 * t1338;
    let t20115 = 6.0 * t547 * t20113;
    let t20116 = t18592 * t1338;
    let t20118 = 6.0 * t547 * t20116;
    let t20119 = t5772 * t3537;
    let t20121 = 6.0 * t547 * t20119;
    let t20123 = 3.0 * t1279 * t6293;
    let t20124 = t116 * t6112;
    (t20105, t20107, t20109, t20111, t20112, t20113, t20115, t20116, t20118, t20119, t20121, t20123, t20124)
}
