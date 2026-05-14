//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 892/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk892<F: Float>(t193: F, t8756: F, t200: F, t8743: F, t1877: F, t2219: F, t8744: F, t776: F, t7844: F, t1530: F, t7109: F, t116481: F, t118377: F, t118407: F, t1408: F, t22960: F, t24191: F, t24339: F, t25: F, t25015: F, t25024: F, t25028: F, t2522: F, t25373: F, t25377: F, t25381: F, t25392: F, t26739: F, t26756: F, t32034: F, t32047: F, t33991: F, t34004: F, t606: F, t7114: F, t8748: F) -> (F, F, F, F, F, F) {
    let t123378 = t193 * t8756;
    let t123382 = t193 * t200 * t8743;
    let t123398 = t1877 * t8744 * t2219;
    let t123414 = t7844 * t776;
    let t123418 = t1530 * t7109;
    let t123428 = 3.0 * t116481 * t118407 - 3.0 * t123378 * t118377 + 3.0 * t123382 * t25015 + t1877 * t33991 * t606 / 2.0 + 3.0 / 2.0 * t2522 * t8744 * t25024 - t1877 * t32034 * t25377 / 2.0 - t1877 * t32034 * t25392 / 2.0 + t123398 + 3.0 / 2.0 * t2522 * t8744 * t25028 + t1877 * t32047 * t25392 - 3.0 / 2.0 * t2522 * t8748 * t25024 - 3.0 / 2.0 * t2522 * t8748 * t25028 - t1877 * t24339 * t34004 + t1877 * t32047 * t25381 - 3.0 * t24191 * t22960 * t123414 + 2.0 * t26756 * t25373 * t123418 - t1877 * t7114 * t1408 * t7109 - t1877 * t7114 * t25 * t26739;
    (t123378, t123382, t123398, t123414, t123418, t123428)
}
