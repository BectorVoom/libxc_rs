//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1369/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369<F: Float>(t12250: F, t12419: F, t16233: F, t16305: F, t16394: F, t1799: F, t19871: F, t19956: F, t20416: F, t20448: F, t20450: F, t20454: F, t20463: F, t20465: F, t3803: F, t3805: F, t5248: F, t5249: F, t550: F, t56878: F, t6394: F, t6396: F, t74110: F, t74120: F, t74147: F, t74189: F, t74415: F, t75008: F) -> (F,) {
    let t80303 = -5.0 / 64.0 * t16394 * t20450 - 5.0 / 128.0 * t3803 * t12419 * t19871 * t20448 + t16394 * t20454 / 64.0 - 7.0 / 96.0 * t74110 + t16233 * t3805 * t74120 * t12250 * t1799 / 32.0 - 3.0 / 256.0 * t16233 * t5248 * t19871 * t75008 + t3803 * t3805 * t19956 * t20463 / 128.0 + t3803 * t3805 * t5249 * t550 * t20416 / 192.0 + t56878 * t6396 / 64.0 + t16394 * t20465 / 64.0 - 7.0 / 192.0 * t74147 + t3803 * t16305 * t74415 * t6394 / 64.0 - 7.0 / 96.0 * t74189;
    (t80303,)
}
