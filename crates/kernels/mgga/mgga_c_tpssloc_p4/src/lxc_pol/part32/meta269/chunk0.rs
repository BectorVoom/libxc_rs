//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1225/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1225<F: Float>(t671: F, t6867: F, t6869: F, t6871: F, t7264: F, t7266: F, t113: F, t1266: F, t1393: F, t2114: F, t2165: F, t2167: F, t510: F, t574: F, t650: F, t652: F, t6522: F, t6524: F, t6527: F, t6537: F, t672: F, t6877: F, t6882: F, t6998: F, t7001: F, t7271: F, t7408: F) -> (F, F) {
    let t7412 = F::new(2.0) * t671 * t7266 + t6867 + t6869 + t6871 + t7264;
    let t7415 = -t113 * t7408 - t1266 * t2114 + t1393 * t2167 - t2165 * t650 - t510 * t7264 + t574 * t7412 - F::new(2.0) * t652 * t7271 - F::new(2.0) * t672 * t7266 - t6522 - t6524 - t6527 - t6537 + t6877 + t6882 + t6998 - t7001;
    (t7412, t7415)
}
