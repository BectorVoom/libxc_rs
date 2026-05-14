//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 888/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk888<F: Float>(t1307: F, t1388: F, t1351: F, t2319: F, t576: F, t671: F, t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2332: F, t2358: F, t6530: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15904 = t1388 * t1307;
    let t16312 = t1351 * t1307;
    let t16535 = t576 * t2319;
    let t20173 = t576 * t671;
    let t22468 = t240 * t107;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22473 = t63 * t2331;
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    (t15904, t16312, t16535, t20173, t22468, t22470, t22471, t22473, t22474, t22476)
}
