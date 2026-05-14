//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1331/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1331<F: Float>(t1338: F, t547: F, t66108: F, t20124: F, t3537: F, t4549: F, t6293: F, t117: F, t68888: F, t6290: F, t116: F, t21190: F, t645: F, t1668: F, t20128: F, t20113: F) -> (F, F, F, F, F, F, F, F) {
    let t71070 = 12.0 * t547 * t66108 * t1338;
    let t71074 = 12.0 * t547 * t20124 * t3537;
    let t71076 = 6.0 * t4549 * t6293;
    let t71085 = 3.0 * t547 * t117 * t68888;
    let t71087 = 12.0 * t4549 * t6290;
    let t71088 = t116 * t21190;
    let t71091 = 6.0 * t547 * t71088 * t645;
    let t71093 = 6.0 * t1668 * t20128;
    let t71097 = 12.0 * t1668 * t20113;
    (t71070, t71074, t71076, t71085, t71087, t71091, t71093, t71097)
}
