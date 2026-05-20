//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1047/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1047<F: Float>(t3923: F, t3928: F, t113: F, t1266: F, t1271: F, t1393: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t3652: F, t3660: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F) -> (F, F) {
    let t3929 = t3923 + t3928;
    let t3931 = -t113 * t3652 - F::new(2.0) * t1266 * t650 + F::new(2.0) * t1271 * t1393 - t2312 * t510 - F::new(4.0) * t2314 * t672 - F::new(2.0) * t2320 * t510 - F::new(4.0) * t2323 * t652 - F::new(2.0) * t2364 * t652 + t3660 * t574 + t3929 * t513;
    (t3929, t3931)
}
