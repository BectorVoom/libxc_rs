//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 619/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk619<F: Float>(t1178: F, t2250: F, t1177: F, t3293: F, t3295: F, t3299: F, t3302: F, t3305: F, t457: F, t460: F, t974: F, t1184: F, t1174: F, t3430: F, t3433: F, t3436: F, t3443: F, t3447: F, t3452: F, t3457: F) -> (F, F, F, F, F, F) {
    let t3460 = t1178 * t2250;
    let t3461 = t1177 * t3460;
    let t3464 = 5.0 / 18.0 * t3293;
    let t3469 = -t3464 + 2.0 / 9.0 * t3295 + t3299 / 18.0 - t3302 / 3.0 - t3305 / 6.0;
    let t3470 = t457 * t3469;
    let t3471 = t3470 * t460;
    let t3472 = t974 * t3471;
    let t3475 = t1184 * t1184;
    let t3477 = t457 * t3475 * t460;
    let t3478 = t974 * t3477;
    let t3481 = -t3430 - 0.18518518518518518518e-3 * t3433 - 0.55555555555555555554e-3 * t3436 + 0.37037037037037037036e-3 * t1174 * t3443 + 0.55555555555555555554e-3 * t3447 * t3452 - 0.55555555555555555554e-3 * t1174 * t3457 - 0.27777777777777777777e-3 * t1174 * t3461 - 0.83333333333333333332e-3 * t1174 * t3472 - 0.83333333333333333332e-3 * t1174 * t3478;
    (t3460, t3469, t3471, t3475, t3477, t3481)
}
