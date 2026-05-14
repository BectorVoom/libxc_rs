//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1286/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1286<F: Float>(t11972: F, t1266: F, t1393: F, t22461: F, t22483: F, t22559: F, t2314: F, t2323: F, t2364: F, t23829: F, t23855: F, t3652: F, t574: F, t6515: F, t6517: F, t652: F, t671: F, t672: F, t83896: F, t83905: F, t83913: F, t83917: F, t83919: F, t83921: F, t83924: F, t83928: F, t83932: F, t83935: F, t83939: F, t83969: F) -> (F,) {
    let t83971 = -6.0 * t23829 * t652 * t671 - 2.0 * t11972 * t6517 - 3.0 * t1266 * t22559 + 3.0 * t1393 * t23855 - 12.0 * t22461 * t2323 - 6.0 * t22461 * t2364 - 6.0 * t22483 * t2314 - 3.0 * t3652 * t6515 + t574 * t83969 - 6.0 * t672 * t83935 - t83896 + t83905 - t83913 - t83917 - t83919 - t83921 - t83924 - t83928 + t83932 - t83939;
    (t83971,)
}
