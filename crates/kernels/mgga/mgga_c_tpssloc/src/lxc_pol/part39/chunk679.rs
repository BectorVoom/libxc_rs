//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 679/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk679<F: Float>(t1090: F, t248: F, t3521: F, t1227: F, t1230: F, t3252: F, t3248: F, t1009: F, t1190: F, t1011: F, t1212: F) -> (F, F, F, F, F, F, F) {
    let t3523 = t248 * t3521 * t1090;
    let t3524 = t1227 * t3523;
    let t3527 = t248 * t1230 * t3252;
    let t3531 = t248 * t1230 * t3248;
    let t3534 = t1190 * t1009;
    let t3535 = t3534 * t1011;
    let t3536 = t3535 * t1212;
    (t3523, t3524, t3527, t3531, t3534, t3535, t3536)
}
