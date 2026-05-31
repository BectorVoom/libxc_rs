//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1183/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183<F: Float>(t40389: F, t40437: F, t225: F, t3774: F, t3862: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t12368: F, t12369: F, t12402: F, t12407: F, t12419: F, t12420: F, t12422: F, t12426: F, t12429: F, t1352: F, t16233: F, t16305: F, t3803: F, t3805: F, t3807: F, t40183: F, t40197: F, t40304: F, t40329: F, t40335: F, t5246: F, t5248: F, t5250: F, t554: F, t559: F) -> (F, F, F, F) {
    let t40438 = t40389 + t40437;
    let t40439 = t40438 * t225;
    let t40443 = t3774 * t3862;
    let t40445 = t6597 * t241;
    let t40449 = F::cast_from(13685.0_f64) / F::cast_from(31104.0_f64) * t555 * t40445 * t557 * t248;
    let t40450 = t3803 * t3805 * t40304 * t3807 / F::cast_from(192.0_f64) - t3803 * t5248 * t40304 * t1352 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t12429 * t12422 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t3803 * t12419 * t12402 * t12420 + t12429 * t12426 / F::cast_from(64.0_f64) + t3803 * t3805 * t12368 * t12407 / F::cast_from(128.0_f64) - t5246 * t16305 * t5250 * t40197 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t40329 - t5246 * t3805 * t40183 * t12369 / F::cast_from(32.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t16233 * t5248 * t12368 * t40335 + t40439 * t554 * t559 / F::cast_from(3072.0_f64) + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t40443 + t40449;
    (t40438, t40439, t40445, t40450)
}
