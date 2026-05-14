//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1207/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1207<F: Float>(t10397: F, t10422: F, t3070: F, t3120: F, t10517: F, t3103: F, t1041: F, t10868: F, t248: F, t2780: F, t10316: F, t3051: F, t10277: F, t976: F, t1021: F, t10263: F, t10403: F, t10493: F, t2776: F, t3039: F, t3048: F, t3071: F, t3121: F, t3132: F, t3146: F, t3151: F, t3153: F, t360: F, t39097: F, t39103: F, t42374: F, t4582: F, t4588: F, t973: F, t974: F) -> (F, F) {
    let t42412 = t3070 * t10422 * t10397;
    let t42422 = t3120 * t3120;
    let t42428 = t10517 * t3103;
    let t42432 = t1041 * t248 * t10868 * t2780;
    let t42436 = t1041 * t248 * t3051 * t10316;
    let t42444 = t976 * t10277;
    let t42459 = t42412 / 576.0 - t3070 * t3071 * t3121 * t2776 / 384.0 - t10403 * t3071 * t3132 * t2776 / 192.0 - t3039 * t248 * t1021 * t42422 * t360 / 1024.0 + 19.0 / 216.0 * t42428 - t42432 / 3456.0 + t42436 / 288.0 - t3048 * t10493 / 36.0 + 5.0 / 3456.0 * t1041 * t4582 * t4588 * t42374 - t973 * t974 * t42444 * t39097 / 12.0 - t973 * t974 * t3151 * t39103 / 48.0 + t973 * t974 * t3146 * t39103 / 72.0 - 11.0 / 27.0 * t10263 * t3153;
    (t42422, t42459)
}
