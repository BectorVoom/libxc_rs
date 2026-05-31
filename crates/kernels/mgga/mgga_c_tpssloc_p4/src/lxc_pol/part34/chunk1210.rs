//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1210/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1210<F: Float>(t107093: F, t107096: F, t107100: F, t107102: F, t107105: F, t107107: F, t107109: F, t107112: F, t107115: F, t107118: F, t107120: F, t107123: F, t107126: F, t84514: F, t91206: F, t97315: F, t97347: F, t97363: F, t97367: F, t97372: F) -> F {
    let t107822 = t107093 / F::cast_from(128.0_f64) + F::cast_from(0.20186378047070195427e-3_f64) * t97315 - t107096 / F::cast_from(2.0_f64) - F::cast_from(0.24223653656484234512e-2_f64) * t107100 - t107102 / F::cast_from(32.0_f64) - F::cast_from(0.18975195364245983701e-1_f64) * t91206 - t107105 / F::cast_from(64.0_f64) + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t107107 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t107109 - F::cast_from(0.24223653656484234512e-2_f64) * t107112 + F::cast_from(0.24223653656484234512e-2_f64) * t107115 - F::cast_from(0.40372756094140390853e-3_f64) * t107118 - t107120 / F::cast_from(24.0_f64) - F::cast_from(0.24223653656484234513e-2_f64) * t97347 - t84514 - t107123 / F::cast_from(768.0_f64) + F::cast_from(0.50869672678616892474e-1_f64) * t107126 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t97363 - F::cast_from(0.40372756094140390854e-3_f64) * t97367 + F::cast_from(0.20186378047070195427e-3_f64) * t97372;
    t107822
}
