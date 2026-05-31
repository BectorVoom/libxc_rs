//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2708/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2708<F: Float>(t1307: F, t5286: F, t1351: F, t6387: F, t12283: F, t19894: F, t12240: F, t1352: F, t16224: F, t16233: F, t16271: F, t16275: F, t16305: F, t16394: F, t1825: F, t19871: F, t19956: F, t19994: F, t210: F, t3719: F, t3733: F, t3803: F, t3807: F, t40124: F, t40126: F, t40145: F, t5246: F, t5248: F, t54014: F, t54068: F, t54153: F, t54293: F, t54295: F, t54533: F, t54535: F, t6374: F, t6394: F) -> (F, F) {
    let t57086 = t1307 * t5286;
    let t57091 = t6387 * t1351;
    let t57127 = t12283 * t19894;
    let t57133 = t3803 * t16305 * t1825 * t57086 / F::cast_from(192.0_f64) + t3803 * t16305 * t57091 * t3807 / F::cast_from(384.0_f64) + t5246 * t5248 * t19871 * t12240 / F::cast_from(512.0_f64) + t5246 * t5248 * t19956 * t12240 / F::cast_from(1536.0_f64) + F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t40124 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t40126 - t16394 * t16271 / F::cast_from(768.0_f64) - t16394 * t16275 / F::cast_from(1536.0_f64) - F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t40145 + t3733 * t210 * t6374 * t3719 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t54293 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t54295 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t54533 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t54535 + t3803 * t16305 * t54153 * t6394 / F::cast_from(384.0_f64) + t16233 * t16305 * t54014 * t54068 / F::cast_from(64.0_f64) + F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t57127 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3803 * t16224 * t19994 * t1352;
    (t57086, t57133)
}
