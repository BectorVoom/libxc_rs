//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 867/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk867<F: Float>(t1688: F, t6234: F, t1165: F, t6112: F, t1338: F, t5514: F, t6096: F, t6233: F, t1604: F, t196: F, t197: F) -> (F, F, F) {
    let t6236 = 2.0 * t6234 * t1688;
    let t6238 = 2.0 * t1165 * t6112;
    let t6239 = 2.0 * t1338 * t5514 + t6096 + t6233 + t6236 + t6238;
    let t6242 = t1604 * t196;
    let t6243 = t6242 * t197;
    (t6239, t6242, t6243)
}
