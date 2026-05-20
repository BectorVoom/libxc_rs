//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2717/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2717<F: Float>(t1266: F, t1271: F, t1459: F, t1778: F, t19451: F, t20098: F, t20136: F, t20143: F, t20296: F, t20698: F, t22425: F, t26114: F, t26179: F, t4026: F, t4028: F, t4037: F, t510: F, t5494: F, t55943: F, t6287: F, t650: F, t652: F, t671: F, t7458: F, t75560: F, t75701: F) -> F {
    let t75762 = -F::new(2.0) * t22425 * t652 * t671 - F::new(2.0) * t510 * t652 * t75701 - F::new(6.0) * t1266 * t20296 + t1271 * t20698 - F::new(6.0) * t1459 * t55943 - F::new(6.0) * t1459 * t75560 + F::new(3.0) * t1778 * t20098 - F::new(6.0) * t19451 * t4037 - F::new(12.0) * t20136 * t4028 - F::new(6.0) * t20143 * t7458 - t22425 * t650 - F::new(6.0) * t26114 * t5494 - F::new(6.0) * t26179 * t5494 - F::new(3.0) * t4026 * t6287;
    t75762
}
