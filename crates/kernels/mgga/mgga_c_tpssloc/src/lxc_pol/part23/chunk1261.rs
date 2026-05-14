//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1261/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1261<F: Float>(t1021: F, t10403: F, t1041: F, t1044: F, t14211: F, t14508: F, t14511: F, t1616: F, t21138: F, t21487: F, t21503: F, t21597: F, t21603: F, t21609: F, t248: F, t3070: F, t3071: F, t3130: F, t3131: F, t3151: F, t42444: F, t4582: F, t4641: F, t4644: F, t5685: F, t5873: F, t62137: F, t62148: F, t62177: F, t62183: F, t70391: F, t70497: F, t75836: F, t75847: F, t76576: F, t76616: F, t76722: F, t973: F, t974: F, t977: F) -> (F,) {
    let t77587 = t4644 * t21609 / 192.0 + t3130 * t4582 * t70391 * t14211 / 384.0 + t14508 * t21487 / 128.0 - t14511 * t21503 / 256.0 + t62137 / 1728.0 - t62148 / 1152.0 - t973 * t974 * t3151 * t75847 / 48.0 - t62177 / 2304.0 + t62183 / 2304.0 + t4641 * t21597 / 768.0 + t4644 * t21603 / 1152.0 + t1041 * t248 * t1044 * t76576 / 4608.0 + t70497 / 36.0 + t973 * t977 * t76616 / 8.0 + t3130 * t248 * t1021 * t76722 * t3131 / 512.0 + t3070 * t3071 * t21138 * t1616 / 192.0 - t973 * t974 * t42444 * t75836 / 12.0 + t10403 * t3071 * t5873 * t5685 / 384.0;
    (t77587,)
}
