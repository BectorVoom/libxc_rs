//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1045/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1045<F: Float>(t4265: F, t4275: F, t242: F, t3060: F, t5254: F, t3080: F, t5243: F, t1111: F, t1128: F, t15235: F, t15262: F, t4219: F, t12278: F, t15257: F, t1098: F, t1125: F, t12290: F, t12294: F, t12319: F, t4289: F, t9543: F) -> (F,) {
    let t15519 = t4265 * t4275;
    let t15522 = t242 * t3060 * t5254;
    let t15523 = t3080 * t15522;
    let t15526 = t242 * t3060 * t5243;
    let t15527 = t1111 * t15526;
    let t15533 = t242 * t1128 * t15235;
    let t15536 = t4219 * t15262;
    let t15539 = t12278 * t15257;
    let t15542 = t15519 / 648.0 - t15523 / 4608.0 + t15527 / 4608.0 - t9543 / 13824.0 + t12290 - t12294 + t4265 * t4289 / 432.0 - t1125 * t15533 / 4608.0 - t12319 + t1098 * t15536 / 36.0 - 7.0 / 648.0 * t1098 * t15539;
    (t15542,)
}
