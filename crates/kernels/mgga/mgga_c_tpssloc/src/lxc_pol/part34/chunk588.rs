//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 588/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk588<F: Float>(t1315: F, t1341: F, t1363: F, t1827: F, t1831: F, t3733: F, t3762: F, t3790: F, t3803: F, t3864: F, t5220: F, t5235: F, t5238: F, t5240: F, t5255: F, t5306: F, t559: F, t6371: F, t6375: F, t6379: F, t6390: F, t6396: F, t6417: F, t6422: F, t6427: F, t6431: F) -> (F,) {
    let t6434 = t3762 + 7.0 / 72.0 * t5220 + t3733 * t6371 / 16.0 - t1315 * t6375 / 48.0 + t6379 * t559 / 3072.0 - t5235 * t1827 / 1536.0 - 7.0 / 2304.0 * t5238 - t5240 * t1831 / 384.0 + t3790 * t6390 / 1536.0 + 7.0 / 2304.0 * t5255 + t3803 * t6396 / 384.0 - t1341 * t6417 / 3072.0 - t1341 * t6422 / 3072.0 + t3864 + 7.0 / 576.0 * t5306 + 5.0 / 768.0 * t1363 * t6427 - t1363 * t6431 / 768.0;
    (t6434,)
}
