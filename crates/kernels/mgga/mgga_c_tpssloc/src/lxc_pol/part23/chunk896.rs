//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 896/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk896<F: Float>(t20473: F, t5248: F, t5249: F, t1367: F, t20416: F, t820: F, t1363: F, t16317: F, t16394: F, t19853: F, t19879: F, t20450: F, t20454: F, t20460: F, t20465: F, t20470: F, t3803: F, t5246: F, t6396: F) -> (F, F, F) {
    let t20475 = t5248 * t5249 * t20473;
    let t20479 = t1367 * t820 * t20416;
    let t20484 = 7.0 / 768.0 * t19853 - 5.0 / 256.0 * t3803 * t20450 + t3803 * t20454 / 256.0 + t16394 * t6396 / 128.0 + t3803 * t20460 / 256.0 + t3803 * t20465 / 256.0 - t5246 * t20470 / 128.0 + t5246 * t20475 / 512.0 - t1363 * t20479 / 768.0 - 7.0 / 192.0 * t19879 - 119.0 / 1152.0 * t16317;
    (t20475, t20479, t20484)
}
