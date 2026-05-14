//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1148/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1148<F: Float>(t6287: F, t671: F, t1774: F, t4072: F, t1266: F, t5493: F, t1271: F, t1393: F, t1459: F, t19450: F, t19451: F, t19456: F, t19461: F, t2314: F, t4028: F, t4034: F, t4037: F, t510: F, t5450: F, t5457: F, t5494: F, t6295: F, t6468: F, t650: F, t652: F, t672: F) -> (F,) {
    let t20127 = t6287 * t671;
    let t20136 = t1774 * t4072;
    let t20143 = t1266 * t5493;
    let t20147 = -t1266 * t5450 - 2.0 * t1266 * t5457 + t1271 * t6468 + t1393 * t6295 - 4.0 * t1459 * t19456 - t19450 * t510 - 2.0 * t19451 * t672 - 2.0 * t19461 * t510 - 2.0 * t20127 * t652 - 4.0 * t20136 * t652 - 2.0 * t20143 * t652 - 2.0 * t2314 * t5494 - 4.0 * t4028 * t4037 - 2.0 * t4034 * t5494 - t6287 * t650;
    (t20147,)
}
