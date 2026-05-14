//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 827/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk827<F: Float>(t509: F, t6435: F, t1270: F, t1845: F, t4525: F, t118: F, t1322: F, t1339: F, t1600: F, t1663: F, t1760: F, t1796: F, t1800: F, t1830: F, t1834: F, t1846: F, t3493: F, t485: F, t544: F, t5801: F, t6103: F, t6243: F, t626: F, t6309: F, t6318: F, t6324: F, t6328: F, t6399: F, t6409: F, t6413: F) -> (F, F, F, F) {
    let t6436 = t509 * t6435;
    let t6437 = t6436 * t1270;
    let t6439 = t1845 * t4525;
    let t6441 = -t118 * t6399 - t1322 * t1830 - 2.0 * t1339 * t5801 - t1600 * t1796 + t1663 * t1834 + 3.0 * t1760 * t6413 + t1760 * t6437 - t1760 * t6439 - 2.0 * t1800 * t3493 - 2.0 * t1800 * t6103 + t1846 * t6243 - t485 * t6309 + t544 * t6409 - 2.0 * t626 * t6318 - 2.0 * t626 * t6324 - 2.0 * t626 * t6328;
    (t6436, t6437, t6439, t6441)
}
