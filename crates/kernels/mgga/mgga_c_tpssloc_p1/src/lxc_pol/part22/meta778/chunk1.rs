//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2664/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2664<F: Float>(t1824: F, t6414: F, t119: F, t1315: F, t16101: F, t16224: F, t16305: F, t16321: F, t19994: F, t20433: F, t20570: F, t210: F, t221: F, t3778: F, t3783: F, t3803: F, t3807: F, t40168: F, t5301: F, t5308: F, t54614: F, t6415: F, t6420: F, t6427: F, t74355: F, t74389: F, t74393: F, t74395: F, t74401: F, t74403: F, t74405: F) -> (F, F) {
    let t74415 = t6414 * t1824;
    let t74428 = -t3778 * t20570 / F::cast_from(3072.0_f64) - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t16101 * t221 * t74389 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t74393 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t74395 - t1315 * t210 * t119 * t74355 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t74401 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t74403 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t74405 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t16321 * t6427 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t3783 * t20433 - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t54614 * t40168 * t5301 * t19994 + t3803 * t16305 * t74415 * t3807 / F::cast_from(256.0_f64) - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t6415 * t5308 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t16224 * t6420 * t5308;
    (t74415, t74428)
}
