//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 870/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk870<F: Float>(t509: F, t6273: F, t1270: F, t1760: F, t1778: F, t4525: F, t118: F, t1322: F, t1339: F, t1600: F, t1663: F, t1684: F, t1753: F, t1757: F, t485: F, t544: F, t5514: F, t6096: F, t6102: F, t6105: F, t6108: F, t6115: F, t6117: F, t6228: F, t6239: F, t6244: F, t6248: F, t626: F) -> (F, F, F, F) {
    let t6274 = t509 * t6273;
    let t6275 = t6274 * t1270;
    let t6276 = t1760 * t6275;
    let t6277 = t1778 * t4525;
    let t6278 = t1760 * t6277;
    let t6279 = -t118 * t6228 - t1322 * t1753 - 2.0 * t1339 * t5514 - t1600 * t1684 + t1663 * t1757 - t485 * t6096 + t544 * t6239 - 2.0 * t6117 * t626 - t6102 - t6105 - t6108 - t6115 + t6244 + t6248 + t6276 - t6278;
    (t6274, t6275, t6277, t6279)
}
