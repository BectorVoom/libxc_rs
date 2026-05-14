//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 799/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk799<F: Float>(t5165: F, t5360: F, t113: F, t1266: F, t1271: F, t1393: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t2314: F, t4026: F, t4028: F, t4034: F, t4037: F, t4073: F, t4077: F, t510: F, t5107: F, t5118: F, t513: F, t574: F, t650: F, t652: F, t672: F) -> (F, F) {
    let t5361 = t5165 + t5360;
    let t5363 = -t113 * t5107 - t1266 * t1442 + t1271 * t1849 + t1393 * t1778 - 2.0 * t1459 * t2314 - 2.0 * t1459 * t4034 - t1774 * t650 - t4026 * t510 - 2.0 * t4028 * t672 - 2.0 * t4037 * t652 - 2.0 * t4073 * t652 - 2.0 * t4077 * t652 + t5118 * t574 + t513 * t5361;
    (t5361, t5363)
}
