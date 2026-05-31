//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 925/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk925<F: Float>(t1363: F, t16317: F, t16394: F, t19853: F, t19879: F, t20450: F, t20454: F, t20460: F, t20465: F, t20470: F, t20475: F, t20479: F, t3803: F, t5246: F, t6396: F) -> F {
    let t20484 = F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t19853 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t20450 + t3803 * t20454 / F::cast_from(256.0_f64) + t16394 * t6396 / F::cast_from(128.0_f64) + t3803 * t20460 / F::cast_from(256.0_f64) + t3803 * t20465 / F::cast_from(256.0_f64) - t5246 * t20470 / F::cast_from(128.0_f64) + t5246 * t20475 / F::cast_from(512.0_f64) - t1363 * t20479 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t19879 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t16317;
    t20484
}
