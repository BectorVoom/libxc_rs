//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1209/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1209<F: Float>(t10413: F, t10414: F, t10422: F, t10393: F, t3070: F, t11046: F, t42387: F, t10457: F, t820: F, t10409: F, t10936: F, t3180: F, t10390: F, t10394: F, t10398: F, t1041: F, t10428: F, t10433: F, t10884: F, t10891: F, t10904: F, t10915: F, t10919: F, t10932: F, t14187: F, t2960: F, t3048: F, t3071: F, t3073: F, t42460: F, t42468: F, t4582: F, t884: F) -> (F,) {
    let t42478 = t10413 * t10422 * t10414;
    let t42481 = t3070 * t10422 * t10393;
    let t42483 = t11046 * t42387;
    let t42488 = t820 * t10457;
    let t42490 = t3070 * t42488 * t10409;
    let t42496 = t3180 * t10936;
    let t42499 = 2.0 / 27.0 * t42460 + 8.0 / 27.0 * t2960 * t10932 - t10904 * t10428 / 24.0 + t10891 * t10433 / 48.0 + 5.0 / 864.0 * t1041 * t4582 * t14187 * t42468 + t3048 * t10915 / 36.0 - 5.0 / 216.0 * t3048 * t10919 - t42478 / 576.0 + t42481 / 576.0 + t42483 * t3071 * t10884 * t884 / 1152.0 + 5.0 / 1728.0 * t42490 + t10390 * t10394 / 384.0 + t10390 * t10398 / 384.0 - t42496 * t3073 / 36.0;
    (t42499,)
}
