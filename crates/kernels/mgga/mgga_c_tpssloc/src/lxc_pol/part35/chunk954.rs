//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 954/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk954<F: Float>(t1363: F, t16317: F, t16394: F, t19853: F, t19879: F, t20450: F, t20454: F, t20460: F, t20465: F, t20470: F, t20475: F, t20479: F, t3803: F, t5246: F, t6396: F) -> F {
    let t20484 = F::new(7.0) / F::new(768.0) * t19853 - F::new(5.0) / F::new(256.0) * t3803 * t20450 + t3803 * t20454 / F::new(256.0) + t16394 * t6396 / F::new(128.0) + t3803 * t20460 / F::new(256.0) + t3803 * t20465 / F::new(256.0) - t5246 * t20470 / F::new(128.0) + t5246 * t20475 / F::new(512.0) - t1363 * t20479 / F::new(768.0) - F::new(7.0) / F::new(192.0) * t19879 - F::new(119.0) / F::new(1152.0) * t16317;
    t20484
}
