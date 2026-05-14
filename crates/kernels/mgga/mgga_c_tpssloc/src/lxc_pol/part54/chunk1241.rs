//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1241/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1241<F: Float>(t33483: F, t776: F, t1877: F, t2219: F, t8566: F, t101840: F, t118410: F, t24191: F, t24339: F, t2522: F, t25373: F, t25381: F, t25392: F, t31434: F, t31441: F, t31449: F, t32899: F, t33466: F, t33477: F, t33484: F, t6542: F, t7114: F, t84797: F, t8569: F, t86721: F, t92271: F, t92276: F) -> (F, F, F) {
    let t121837 = t33483 * t776;
    let t121861 = t1877 * t8566 * t2219;
    let t121865 = -t1877 * t92276 * t8569 / 2.0 + 3.0 * t24191 * t25373 * t121837 + t92271 * t33484 - 3.0 / 2.0 * t24191 * t86721 * t31441 - t1877 * t24339 * t32899 / 2.0 - t1877 * t7114 * t118410 / 2.0 - t1877 * t31434 * t25381 / 2.0 + 3.0 / 2.0 * t2522 * t33466 * t6542 - t1877 * t31434 * t25392 / 2.0 + t121861 - 3.0 / 2.0 * t84797 * t33477 + t101840 * t31449;
    (t121837, t121861, t121865)
}
