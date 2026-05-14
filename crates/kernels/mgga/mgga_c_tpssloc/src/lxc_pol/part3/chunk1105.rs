//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1105/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1105<F: Float>(t1336: F, t16397: F, t5252: F, t3777: F, t5245: F, t12419: F, t12420: F, t5249: F, t12215: F, t12335: F, t12340: F, t12346: F, t12356: F, t12358: F, t12366: F, t12386: F, t12388: F, t12395: F, t12429: F, t16366: F, t16370: F, t16379: F, t16383: F, t16387: F, t16391: F, t16394: F, t3803: F, t3809: F, t5246: F, t5303: F) -> (F,) {
    let t16398 = t1336 * t16397;
    let t16400 = 7.0 / 1152.0 * t16398 * t5252;
    let t16401 = t3777 * t5245;
    let t16405 = t12419 * t5249 * t12420;
    let t16411 = -t12335 + t12429 * t5303 / 384.0 + t3803 * t16366 / 384.0 + t3803 * t16370 / 768.0 + 7.0 / 576.0 * t12340 - 119.0 / 1728.0 * t12346 - 35.0 / 1152.0 * t12356 + 7.0 / 1152.0 * t12358 - 119.0 / 6912.0 * t12366 - t12215 * t16379 / 4.0 + t3803 * t16383 / 768.0 + t5246 * t16387 / 512.0 - t5246 * t16391 / 384.0 + t16394 * t3809 / 384.0 - t16400 + t16401 * t5252 / 768.0 - 5.0 / 768.0 * t3803 * t16405 - 7.0 / 2304.0 * t12386 + 7.0 / 4608.0 * t12388 + 7.0 / 4608.0 * t12395;
    (t16411,)
}
