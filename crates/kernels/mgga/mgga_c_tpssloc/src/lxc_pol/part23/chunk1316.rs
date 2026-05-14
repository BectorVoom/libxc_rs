//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1316/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316<F: Float>(t6218: F, t11668: F, t11779: F, t1214: F, t1227: F, t15615: F, t1735: F, t1748: F, t19033: F, t21745: F, t21749: F, t22197: F, t22208: F, t248: F, t3506: F, t3508: F, t3577: F, t3578: F, t4582: F, t47: F, t471: F, t479: F, t488: F, t5005: F, t6207: F, t65600: F, t65605: F, t72255: F, t72352: F, t72366: F, t77606: F, t77957: F, t8025: F) -> (F, F) {
    let t78757 = t6218 * t6218;
    let t78775 = -t3577 * t3578 * t1735 * t21749 / 192.0 + 5.0 / 1152.0 * t3577 * t11668 * t1735 * t21745 + 5.0 / 1152.0 * t5005 * t22197 - t1227 * t4582 * t15615 * t77606 / 128.0 - 5.0 / 1296.0 * t5005 * t22208 - 5.0 / 432.0 * t1227 * t248 * t11779 * t77957 - t72255 * t1748 / 1152.0 + t3506 * t248 * t1214 * t78757 * t3508 / 512.0 - 11.0 / 81.0 * t72352 + t65600 / 216.0 - t65605 / 1152.0 + 5225.0 / 7776.0 * t471 * t479 / t47 / t8025 * t488 - 19.0 / 432.0 * t19033 * t6207 + t72366 / 384.0;
    (t78757, t78775)
}
