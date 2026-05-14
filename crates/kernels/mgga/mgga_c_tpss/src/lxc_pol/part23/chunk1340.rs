//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1340/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1340<F: Float>(t19128: F, t6513: F, t3048: F, t6509: F, t20862: F, t6030: F, t1586: F, t19150: F, t4322: F, t6032: F, t1143: F, t1148: F, t12352: F, t12577: F, t1587: F, t1884: F, t1885: F, t19115: F, t19123: F, t19129: F, t19131: F, t19143: F, t19146: F, t19147: F, t19155: F, t19158: F, t20856: F, t20877: F, t20878: F, t20883: F, t20886: F, t20891: F, t20903: F, t20913: F, t3119: F, t3120: F, t342: F, t450: F, t452: F, t6016: F, t6019: F, t6024: F, t6025: F, t6031: F, t6034: F, t6035: F, t63219: F, t63237: F, t63339: F, t68356: F, t68527: F) -> (F,) {
    let t68532 = t6513 * t19128;
    let t68557 = t3048 * t6509;
    let t68572 = t20862 * t6030;
    let t68581 = t19150 * t1586;
    let t68585 = t6032 * t4322;
    let t68591 = -t63339 * t1587 + 4.0 * t68532 * t19131 + 2.0 * t19129 * t20877 * t19155 + 4.0 * t19115 * t20883 + 2.0 * t20856 * t3120 + 4.0 * t19129 * t20913 * t1148 * t6034 - 6.0 * t63219 * t20891 * t1143 * t3119 - 12.0 * t6024 * t19123 * t20886 * t1148 - 2.0 * t6031 * t19150 * t20903 - 2.0 * t19143 * t68557 * t19146 + t19143 * t20913 * t19158 - t1884 * t1885 * t452 * t68527 + 4.0 * t6024 * t6025 * t6016 * t4322 - 2.0 * t68356 * t19147 - 2.0 * t68572 * t6035 - t6031 * t6032 * t12352 * t342 * t450 + 2.0 * t6019 * t12577 + 4.0 * t19129 * t68581 * t6034 + 4.0 * t19129 * t68585 * t6034 + 4.0 * t63237 * t20878;
    (t68591,)
}
