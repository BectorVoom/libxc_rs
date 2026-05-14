//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 876/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk876<F: Float>(t1338: F, t5986: F, t6233: F, t6236: F, t6238: F, t6480: F, t118: F, t1322: F, t1339: F, t1600: F, t1663: F, t1865: F, t1897: F, t1899: F, t485: F, t544: F, t6102: F, t6105: F, t6108: F, t6115: F, t6244: F, t6248: F, t626: F, t6276: F, t6278: F, t6486: F, t6540: F) -> (F, F) {
    let t6544 = 2.0 * t1338 * t5986 + t6233 + t6236 + t6238 + t6480;
    let t6547 = -t118 * t6540 - t1322 * t1897 - 2.0 * t1339 * t5986 - t1600 * t1865 + t1663 * t1899 - t485 * t6480 + t544 * t6544 - 2.0 * t626 * t6486 - t6102 - t6105 - t6108 - t6115 + t6244 + t6248 + t6276 - t6278;
    (t6544, t6547)
}
