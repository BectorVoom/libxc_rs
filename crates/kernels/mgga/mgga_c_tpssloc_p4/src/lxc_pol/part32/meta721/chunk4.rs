//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2294/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2294<F: Float>(t24574: F, t29804: F, t18525: F, t19249: F, t2123: F, t2155: F, t24589: F, t24590: F, t29532: F, t29808: F, t29812: F, t3487: F, t6140: F, t64595: F, t7283: F, t7295: F, t7392: F, t85701: F, t86403: F, t94427: F, t94436: F, t94439: F, t94446: F, t94451: F, t94456: F) -> F {
    let t103261 = t24574 * t29804;
    let t103279 = -t64595 * t2155 - t94427 + F::cast_from(0.18277045187202515961e-2_f64) * t85701 + F::cast_from(0.54831135561607547883e-2_f64) * t103261 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t86403 * t29808 - F::cast_from(0.36554090374405031923e-2_f64) * t94436 - t94439 - t94446 - t19249 * t7392 + t94451 - t94456 + F::new(4.0) * t3487 * t29532 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t18525 * t2123 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t6140 * t7295 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t24590 * t29812;
    t103279
}
