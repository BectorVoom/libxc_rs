//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1387/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1387<F: Float>(t20570: F, t6945: F, t1361: F, t20563: F, t26288: F, t107093: F, t107096: F, t107100: F, t107102: F, t107105: F, t107107: F, t107109: F, t107112: F, t107115: F, t107118: F, t107120: F, t80826: F, t91206: F, t97315: F, t97347: F, t97363: F, t97367: F, t97372: F) -> F {
    let t107123 = t6945 * t20570;
    let t107126 = t26288 * t1361 * t20563;
    let t107131 = t107093 / F::cast_from(256.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t97315 - t107096 / F::cast_from(4.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t107100 - t107102 / F::cast_from(64.0_f64) - F::cast_from(0.94875976821229918508e-2_f64) * t91206 - t107105 / F::cast_from(128.0_f64) + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t107107 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t107109 - F::cast_from(0.12111826828242117256e-2_f64) * t107112 + F::cast_from(0.12111826828242117256e-2_f64) * t107115 - F::cast_from(0.20186378047070195427e-3_f64) * t107118 - t107120 / F::cast_from(48.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t97347 - t80826 - t107123 / F::cast_from(1536.0_f64) + F::cast_from(0.25434836339308446237e-1_f64) * t107126 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t97363 - F::cast_from(0.20186378047070195427e-3_f64) * t97367 + F::cast_from(0.10093189023535097714e-3_f64) * t97372;
    t107131
}
