//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1230/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230<F: Float>(t5866: F, t5872: F, t1021: F, t10408: F, t1041: F, t10413: F, t10482: F, t1622: F, t17177: F, t17607: F, t17923: F, t18030: F, t21393: F, t21398: F, t21516: F, t248: F, t28651: F, t3039: F, t3070: F, t3071: F, t360: F, t43291: F, t43292: F, t43385: F, t43399: F, t4644: F, t48570: F, t50265: F, t5857: F, t5861: F, t5869: F, t5875: F, t61663: F, t61736: F, t70122: F, t70132: F, t70138: F, t70153: F, t76572: F) -> (F, F, F) {
    let t76722 = t5866 * t5866;
    let t76740 = t5872 * t5872;
    let t76768 = t61736 * t5875 / 256.0 - t70132 / 288.0 - t3039 * t248 * t1021 * t76722 * t360 / 1024.0 + 5.0 / 1296.0 * t4644 * t21516 + 55.0 / 15552.0 * t1041 * t248 * t43399 * t76572 + t70153 * t1622 / 1152.0 + t48570 * t21393 / 128.0 - t50265 * t21398 / 128.0 + t43291 * t248 * t1021 * t76740 * t43292 / 128.0 - 3.0 / 256.0 * t43385 * t248 * t1021 * t76740 * t10482 + t70138 / 576.0 - t10413 * t3071 * t70122 * t17923 / 384.0 + 5.0 / 1152.0 * t3070 * t10408 * t17177 * t28651 * t360 - t61663 / 1152.0 + t17607 * t5857 / 768.0 + 5.0 / 2304.0 * t17607 * t5861 + t18030 * t5869 / 512.0;
    (t76722, t76740, t76768)
}
