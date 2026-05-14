//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1250/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1250<F: Float>(t19334: F, t19455: F, t19613: F, t20080: F, t3: F, t1786: F, t4549: F, t1668: F, t5773: F, t5776: F, t1279: F, t6290: F, t1688: F, t645: F, t1338: F, t547: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20082 = t19334 + t19455 + t19613 + t20080;
    let t20083 = t3 * t20082;
    let t20094 = param_d * t20082;
    let t20105 = 3.0 * t4549 * t1786;
    let t20107 = 6.0 * t1668 * t5773;
    let t20109 = 3.0 * t1668 * t5776;
    let t20111 = 6.0 * t1279 * t6290;
    let t20112 = t645 * t1688;
    let t20113 = t20112 * t1338;
    let t20115 = 6.0 * t547 * t20113;
    (t20082, t20083, t20094, t20105, t20107, t20109, t20111, t20112, t20113, t20115)
}
