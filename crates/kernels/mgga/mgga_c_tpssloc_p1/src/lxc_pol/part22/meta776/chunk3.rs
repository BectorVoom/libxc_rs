//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2654/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2654<F: Float>(t16398: F, t20475: F, t19731: F, t3792: F, t12429: F, t16242: F, t16394: F, t16401: F, t19631: F, t19871: F, t19956: F, t19958: F, t19989: F, t20460: F, t20463: F, t20465: F, t20470: F, t20473: F, t3803: F, t3805: F, t5187: F, t5246: F, t5248: F, t5249: F, t5250: F, t550: F, t56817: F, t6394: F, t74120: F) -> (F, F) {
    let t74147 = t16398 * t20475;
    let t74174 = t3792 * t19731;
    let t74181 = F::new(7.0) / F::new(1536.0) * t5246 * t5248 * t74120 * t5250 + t3803 * t3805 * t16242 * t20463 / F::new(256.0) + t3803 * t3805 * t5249 * t550 * t19631 / F::new(256.0) - F::new(7.0) / F::new(768.0) * t74147 + t12429 * t20460 / F::new(256.0) + t3803 * t3805 * t56817 * t6394 / F::new(256.0) + t3803 * t3805 * t19956 * t19989 / F::new(256.0) + t12429 * t20465 / F::new(256.0) - t16401 * t20470 / F::new(128.0) - t5246 * t3805 * t19871 * t3792 * t5187 / F::new(128.0) + t16401 * t20475 / F::new(512.0) + t5246 * t5248 * t16242 * t20473 / F::new(512.0) + t5246 * t5248 * t5249 * t74174 / F::new(512.0) + t16394 * t19958 / F::new(256.0);
    (t74174, t74181)
}
