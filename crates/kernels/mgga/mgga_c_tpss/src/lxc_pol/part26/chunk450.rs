//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 450/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk450<F: Float>(t1054: F, t1073: F, t1507: F, t1521: F, t1523: F, t1531: F, t1536: F, t1543: F, t294: F, t421: F, t1072: F, t1081: F, t1542: F, t1089: F, t1300: F, t332: F) -> (F, F, F, F, F) {
    let t1547 = t294 * (-0.310907e-1 * t1523 * t421 + 1.0 * t1054 * t1531 + t1507 - t1521 - 0.19751673498613801407e-1 * t1536 + 0.5848223622634646207e0 * t1073 * t1543);
    let t1549 = 0.19751673498613801407e-1 * t294 * t1536;
    let t1551 = t1072 * t1542 * t1081;
    let t1553 = 0.5848223622634646207e0 * t1089 * t1551;
    let t1554 = t1300 * t332;
    (t1547, t1549, t1551, t1553, t1554)
}
