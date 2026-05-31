//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1349/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1349<F: Float>(t21390: F, t3032: F, t1618: F, t21393: F, t21398: F, t21405: F, t21603: F, t25580: F, t28578: F, t360: F, t5857: F, t5861: F, t5880: F, t6765: F, t82987: F, t83054: F, t83058: F, t83065: F, t83142: F, t88342: F, t88600: F, t99509: F, t99539: F) -> (F, F) {
    let t106209 = t21390 * t3032;
    let t106218 = -t99509 / F::cast_from(768.0_f64) + t83054 * t21393 / F::cast_from(256.0_f64) - t83058 * t21398 / F::cast_from(256.0_f64) - t88600 * t5880 / F::cast_from(512.0_f64) + t83065 * t21405 / F::cast_from(1536.0_f64) + t6765 * t21603 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t25580 * t5861 + F::cast_from(0.60559134141210586284e-3_f64) * t88342 * t28578 + F::cast_from(0.10093189023535097714e-3_f64) * t82987 * t83142 * t106209 * t360 + t25580 * t5857 / F::cast_from(768.0_f64) + t99539 * t1618 / F::cast_from(512.0_f64);
    (t106209, t106218)
}
