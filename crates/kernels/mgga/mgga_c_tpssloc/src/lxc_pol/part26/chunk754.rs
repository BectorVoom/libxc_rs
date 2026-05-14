//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 754/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk754<F: Float>(t2319: F, t649: F, t107: F, t2585: F, t2281: F, t667: F, t2333: F, t626: F, t2359: F, t655: F, t2332: F, t666: F, t2331: F, t2358: F, t2261: F, t93: F, t94: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9351 = t649 * t2319;
    let t9358 = 154.0 / 27.0 * t2585 * t107;
    let t9359 = t2281 * t667;
    let t9361 = t626 * t2333;
    let t9363 = t626 * t2359;
    let t9364 = t655 * t655;
    let t9365 = 1.0 / t9364;
    let t9366 = t2332 * t666;
    let t9367 = t9365 * t9366;
    let t9370 = t2331 * t666;
    let t9371 = t9370 * t2358;
    let t9374 = tau0 * t2261;
    let t9383 = t94 * t93;
    (t9351, t9358, t9359, t9361, t9363, t9365, t9366, t9367, t9371, t9374, t9383)
}
