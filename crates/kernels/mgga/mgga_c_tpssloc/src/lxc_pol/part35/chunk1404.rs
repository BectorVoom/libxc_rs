//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1404/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1404<F: Float>(t26257: F, t6427: F, t20433: F, t6952: F, t12289: F, t20490: F, t6936: F, t20495: F, t3788: F, t1339: F, t20568: F, t20501: F, t6916: F) -> (F, F, F, F, F, F) {
    let t107107 = t26257 * t6427;
    let t107109 = t6952 * t20433;
    let t107112 = t6936 * t12289 * t20490;
    let t107115 = t6936 * t3788 * t20495;
    let t107118 = t6936 * t1339 * t20568;
    let t107120 = t6916 * t20501;
    (t107107, t107109, t107112, t107115, t107118, t107120)
}
