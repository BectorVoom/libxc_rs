//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 665/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk665<F: Float>(t1137: F, t3333: F, t3236: F, t3293: F, t3238: F, t3245: F, t3250: F, t3254: F, t3272: F, t3280: F, t3288: F, t3290: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F) {
    let t3334 = t3333 * t1137;
    let t3339 = F::cast_from(0.68863333333333333333e0_f64) * t3236;
    let t3346 = F::cast_from(0.17365833333333333333e0_f64) * t3293;
    let t3351 = -F::new(0.17648625e1) * t3272 + F::new(0.3529725e1) * t3280 + t3339 - F::cast_from(0.34431666666666666666e0_f64) * t3238 - F::cast_from(0.34431666666666666667e0_f64) * t3245 + F::new(0.103295e1) * t3250 + F::new(0.516475e0) * t3254 + F::new(0.31558125e0) * t3288 + F::new(0.6311625e0) * t3290 + t3346 - F::cast_from(0.13892666666666666667e0_f64) * t3295 - F::cast_from(0.34731666666666666667e-1_f64) * t3299 + F::new(0.20839e0) * t3302 + F::new(0.104195e0) * t3305;
    (t3334, t3339, t3346, t3351)
}
