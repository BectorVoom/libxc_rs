//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 878/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk878<F: Float>(t20264: F, t33: F, t20217: F, t20234: F, t4007: F, t4012: F, t5398: F, t634: F, t638: F, t9321: F, t9330: F, t72: F, t1411: F, t1427: F, t1434: F, t19322: F, t20207: F, t20210: F, t20219: F, t20222: F, t20227: F, t5393: F, t5400: F, t5403: F, t5428: F, t5442: F, t66: F, t80: F) -> (F, F) {
    let t20265 = t33 * t20264;
    let t20284 = -280.0 / 27.0 * t9321 * t20234 + 28.0 / 3.0 * t4007 * t5398 - 4.0 / 3.0 * t634 * t20217 + 280.0 / 27.0 * t9330 * t20234 + 28.0 / 3.0 * t4012 * t5398 + 4.0 / 3.0 * t638 * t20217;
    let t20285 = t72 * t20284;
    let t20288 = -t19322 * t20207 / 4.0 - t20210 * t80 / 4.0 - t5393 * t1434 / 4.0 - t20219 * t80 / 12.0 - t20222 * t80 / 4.0 - t5400 * t1434 / 4.0 - t20227 * t80 / 4.0 - t5403 * t1434 / 2.0 - t1411 * t5442 / 4.0 + t20265 * t80 / 24.0 + t5428 * t1434 / 8.0 + t1427 * t5442 / 8.0 + t66 * t20285 / 24.0;
    (t20284, t20288)
}
