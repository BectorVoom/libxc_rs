//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1206/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206<F: Float>(t10477: F, t67: F, t3067: F, t11059: F, t10970: F, t820: F, t10418: F, t10422: F, t3070: F, t1021: F, t1023: F, t10305: F, t10316: F, t10321: F, t10403: F, t10408: F, t1041: F, t10426: F, t10483: F, t10883: F, t10886: F, t248: F, t2771: F, t3041: F, t3071: F, t3131: F, t3132: F, t360: F, t42347: F, t42348: F, t42354: F, t42358: F, t42369: F, t42372: F, t42374: F, t42380: F, t4582: F, t4583: F, t884: F) -> (F, F, F) {
    let t42386 = t10477 * t67;
    let t42387 = t3067 * t42386;
    let t42388 = t11059 * t42387;
    let t42397 = t820 * t10970;
    let t42403 = t3070 * t10422 * t10418;
    let t42409 = 7.0 / 1536.0 * t42347 * t248 * t1021 * t42348 * t3131 + t42354 * t10886 / 768.0 - t42358 * t248 * t1021 * t42348 * t360 / 3072.0 + t10883 * t4582 * t10426 * t3041 / 512.0 - t42369 / 288.0 + 5.0 / 1728.0 * t42372 - t1041 * t4582 * t4583 * t42374 / 576.0 + t42380 / 288.0 + t3070 * t3071 * t10316 * t1023 / 192.0 + t42388 * t3071 * t10483 * t884 / 192.0 + 5.0 / 1152.0 * t10403 * t10408 * t3132 * t2771 + 5.0 / 1296.0 * t3070 * t42397 * t10305 * t1023 - t42403 / 288.0 + t3070 * t3071 * t10321 * t1023 / 1152.0;
    (t42386, t42387, t42409)
}
