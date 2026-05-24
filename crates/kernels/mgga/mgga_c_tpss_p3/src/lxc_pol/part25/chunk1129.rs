//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1129/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1129<F: Float>(t12278: F, t15257: F, t1098: F, t1125: F, t12290: F, t12294: F, t12319: F, t15519: F, t15523: F, t15527: F, t15533: F, t15536: F, t4265: F, t4289: F, t9543: F) -> F {
    let t15539 = t12278 * t15257;
    let t15542 = t15519 / F::new(648.0) - t15523 / F::new(4608.0) + t15527 / F::new(4608.0) - t9543 / F::new(13824.0) + t12290 - t12294 + t4265 * t4289 / F::new(432.0) - t1125 * t15533 / F::new(4608.0) - t12319 + t1098 * t15536 / F::new(36.0) - F::new(7.0) / F::new(648.0) * t1098 * t15539;
    t15542
}
