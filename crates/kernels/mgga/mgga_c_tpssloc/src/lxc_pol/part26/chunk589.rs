//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 589/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk589<F: Float>(t3507: F, t3508: F, t1214: F, t248: F, t1210: F, t3504: F, t3500: F, t475: F, t121: F, t1229: F, t1090: F, t1227: F, t1230: F, t3252: F, t3248: F, t1009: F, t1190: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3509 = t3507 * t3508;
    let t3511 = t248 * t1214 * t3509;
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    let t3516 = t3507 * t475;
    let t3518 = t248 * t1214 * t3516;
    let t3521 = t121 * t1229;
    let t3523 = t248 * t3521 * t1090;
    let t3524 = t1227 * t3523;
    let t3527 = t248 * t1230 * t3252;
    let t3531 = t248 * t1230 * t3248;
    let t3534 = t1190 * t1009;
    (t3509, t3511, t3514, t3515, t3516, t3518, t3521, t3523, t3524, t3527, t3531, t3534)
}
