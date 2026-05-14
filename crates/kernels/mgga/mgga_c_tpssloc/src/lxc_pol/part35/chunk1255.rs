//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1255/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1255<F: Float>(t20495: F, t3788: F, t6936: F, t1339: F, t20568: F, t20501: F, t6916: F, t20570: F, t6945: F, t1361: F, t20563: F, t26288: F, t107093: F, t107096: F, t107100: F, t107102: F, t107105: F, t107107: F, t107109: F, t107112: F, t80826: F, t91206: F, t97315: F, t97347: F, t97363: F, t97367: F, t97372: F) -> (F,) {
    let t107115 = t6936 * t3788 * t20495;
    let t107118 = t6936 * t1339 * t20568;
    let t107120 = t6916 * t20501;
    let t107123 = t6945 * t20570;
    let t107126 = t26288 * t1361 * t20563;
    let t107131 = t107093 / 256.0 + 0.10093189023535097714e-3 * t97315 - t107096 / 4.0 - 0.12111826828242117256e-2 * t107100 - t107102 / 64.0 - 0.94875976821229918508e-2 * t91206 - t107105 / 128.0 + 5.0 / 128.0 * t107107 - 5.0 / 64.0 * t107109 - 0.12111826828242117256e-2 * t107112 + 0.12111826828242117256e-2 * t107115 - 0.20186378047070195427e-3 * t107118 - t107120 / 48.0 - 0.12111826828242117256e-2 * t97347 - t80826 - t107123 / 1536.0 + 0.25434836339308446237e-1 * t107126 - 7.0 / 768.0 * t97363 - 0.20186378047070195427e-3 * t97367 + 0.10093189023535097714e-3 * t97372;
    (t107131,)
}
