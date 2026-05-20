//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1549/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1549<F: Float>(t12215: F, t12335: F, t12340: F, t12346: F, t12356: F, t12358: F, t12366: F, t12386: F, t12388: F, t12395: F, t12429: F, t16366: F, t16370: F, t16379: F, t16383: F, t16387: F, t16391: F, t16394: F, t16400: F, t16401: F, t16405: F, t3803: F, t3809: F, t5246: F, t5252: F, t5303: F) -> F {
    let t16411 = -t12335 + t12429 * t5303 / F::new(384.0) + t3803 * t16366 / F::new(384.0) + t3803 * t16370 / F::new(768.0) + F::new(7.0) / F::new(576.0) * t12340 - F::new(119.0) / F::new(1728.0) * t12346 - F::new(35.0) / F::new(1152.0) * t12356 + F::new(7.0) / F::new(1152.0) * t12358 - F::new(119.0) / F::new(6912.0) * t12366 - t12215 * t16379 / F::new(4.0) + t3803 * t16383 / F::new(768.0) + t5246 * t16387 / F::new(512.0) - t5246 * t16391 / F::new(384.0) + t16394 * t3809 / F::new(384.0) - t16400 + t16401 * t5252 / F::new(768.0) - F::new(5.0) / F::new(768.0) * t3803 * t16405 - F::new(7.0) / F::new(2304.0) * t12386 + F::new(7.0) / F::new(4608.0) * t12388 + F::new(7.0) / F::new(4608.0) * t12395;
    t16411
}
