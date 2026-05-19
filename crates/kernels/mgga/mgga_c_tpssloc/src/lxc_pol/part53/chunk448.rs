//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 448/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk448<F: Float>(t3374: F, t440: F, t3236: F, t3293: F, t1146: F, t448: F, t1143: F, t300: F, t457: F, t697: F, t461: F, t221: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3375 = F::new(1.0) / t3374;
    let t3376 = t440 * t3375;
    let t3383 = F::cast_from(0.40256666666666666667e0_f64) * t3236;
    let t3390 = F::new(0.137975e0) * t3293;
    let t3399 = t1146 * t1146;
    let t3400 = F::new(1.0) / t3399;
    let t3401 = t440 * t3400;
    let t3402 = t448 * t448;
    let t3403 = F::new(1.0) / t3402;
    let t3411 = t300 * t1143;
    let t3426 = t697 * t457;
    let t3427 = t3426 * t461;
    let t3428 = t221 * t3427;
    (t3375, t3376, t3383, t3390, t3400, t3401, t3403, t3411, t3426, t3428)
}
