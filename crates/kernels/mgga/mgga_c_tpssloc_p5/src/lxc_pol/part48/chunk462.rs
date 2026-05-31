//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 462/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk462<F: Float>(t1124: F, t1128: F, t1127: F, t432: F, t427: F, t1136: F, t1137: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F, F) {
    let t3327 = t1124 * t1128;
    let t3330 = t1127 * t432;
    let t3331 = F::cast_from(1.0_f64) / t3330;
    let t3332 = t427 * t3331;
    let t3333 = t1136 * t1136;
    let t3334 = t3333 * t1137;
    let t3339 = F::cast_from(0.68863333333333333333e0_f64) * t3236;
    let t3346 = F::cast_from(0.17365833333333333333e0_f64) * t3293;
    let t3351 = -F::cast_from(0.17648625e1_f64) * t3272 + F::cast_from(0.3529725e1_f64) * t3280 + t3339 - F::cast_from(0.34431666666666666666e0_f64) * t3238 - F::cast_from(0.34431666666666666667e0_f64) * t3245 + F::cast_from(0.103295e1_f64) * t3250 + F::cast_from(0.516475e0_f64) * t3254 + F::cast_from(0.31558125e0_f64) * t3288 + F::cast_from(0.6311625e0_f64) * t3290 + t3346 - F::cast_from(0.13892666666666666667e0_f64) * t3295 - F::cast_from(0.34731666666666666667e-1_f64) * t3299 + F::cast_from(0.20839e0_f64) * t3302 + F::cast_from(0.104195e0_f64) * t3305;
    (t3327, t3332, t3333, t3334, t3351)
}
