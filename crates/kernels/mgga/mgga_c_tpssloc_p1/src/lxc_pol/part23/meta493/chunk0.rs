//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1512/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1512<F: Float>(t225: F, t80048: F, t6387: F, t3792: F, t40046: F, t12250: F, t550: F, t12419: F, t16224: F, t16305: F, t16394: F, t1825: F, t19871: F, t19876: F, t19956: F, t20442: F, t20460: F, t20470: F, t20473: F, t20563: F, t28099: F, t3803: F, t3805: F, t5246: F, t5248: F, t6330: F, t6347: F, t6388: F, t6394: F, t6420: F, t74090: F, t74120: F) -> (F, F, F, F, F, F) {
    let t80175 = t80048 * t225;
    let t80180 = t6387 * t6387;
    let t80181 = t80180 * t3792;
    let t80185 = t80180 * t40046;
    let t80189 = t80180 * t12250;
    let t80193 = t80180 * t550;
    let t80265 = -t5246 * t16305 * t20473 * t28099 / F::new(32.0) - t3803 * t5248 * t19956 * t6420 / F::new(512.0) - t3803 * t5248 * t74090 * t1825 / F::new(768.0) - t5246 * t3805 * t19871 * t3792 * t6347 / F::new(64.0) - F::new(5.0) / F::new(64.0) * t3803 * t16224 * t20563 * t1825 - t16394 * t20442 / F::new(256.0) + t3803 * t3805 * t74120 * t6394 / F::new(192.0) + t3803 * t3805 * t74090 * t6394 / F::new(192.0) - t19876 * t20470 / F::new(32.0) + F::new(3.0) / F::new(256.0) * t5246 * t5248 * t19956 * t6388 + F::new(5.0) / F::new(64.0) * t5246 * t12419 * t19871 * t3792 * t6330 + t16394 * t20460 / F::new(64.0);
    (t80175, t80181, t80185, t80189, t80193, t80265)
}
