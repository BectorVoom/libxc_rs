//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1202/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1202<F: Float>(t10459: F, t3117: F, t10469: F, t990: F, t10471: F, t10875: F, t10214: F, t10378: F, t1041: F, t10463: F, t10863: F, t10879: F, t248: F, t2960: F, t2979: F, t3062: F, t3098: F, t39097: F, t41644: F, t41693: F, t41697: F, t41701: F, t41705: F, t42303: F, t42309: F, t42322: F, t973: F, t974: F, t977: F) -> (F, F, F) {
    let t42324 = t3117 * t10459;
    let t42332 = t990 * t10469;
    let t42333 = t42332 * t10471;
    let t42334 = t42333 * t10875;
    let t42337 = 2.0 / 9.0 * t2960 * t10378 + 7.0 / 108.0 * t973 * t10214 * t41693 + 5.0 / 4608.0 * t1041 * t248 * t3062 * t41701 + 19.0 / 324.0 * t42303 + t10863 * t3098 / 36.0 + 35.0 / 972.0 * t973 * t974 * t42309 * t39097 - t973 * t977 * t41644 / 36.0 + t973 * t2979 * t41705 / 54.0 + t42322 / 1728.0 + 5.0 / 1728.0 * t42324 + t3117 * t10463 / 1152.0 + 5.0 / 384.0 * t1041 * t248 * t3062 * t41697 - t42334 * t10879 / 128.0;
    (t42332, t42333, t42337)
}
