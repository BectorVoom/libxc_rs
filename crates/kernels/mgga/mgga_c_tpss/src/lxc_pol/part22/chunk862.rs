//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 862/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk862<F: Float>(t6134: F, t935: F, t1398: F, t30: F, t1364: F, t33: F, t1338: F, t93: F, t1604: F, t196: F, t197: F) -> (F, F, F, F, F, F, F) {
    let t6135 = t6134 * t935;
    let t6153 = t30 * t1398;
    let t6207 = t33 * t1364;
    let t6214 = t33 * t1398;
    let t6234 = t93 * t1338;
    let t6242 = t1604 * t196;
    let t6243 = t6242 * t197;
    (t6135, t6153, t6207, t6214, t6234, t6242, t6243)
}
