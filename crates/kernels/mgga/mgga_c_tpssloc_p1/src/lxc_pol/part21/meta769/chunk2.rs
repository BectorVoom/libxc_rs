//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2667/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2667<F: Float>(t1266: F, t12724: F, t12728: F, t12835: F, t1774: F, t19289: F, t19450: F, t19451: F, t19461: F, t19534: F, t20100: F, t20127: F, t2314: F, t2363: F, t2364: F, t3652: F, t4026: F, t4028: F, t4034: F, t510: F, t5107: F, t5493: F, t55410: F, t55943: F, t6287: F, t652: F, t671: F, t672: F, t7458: F, t89: F) -> F {
    let t56075 = -F::new(4.0) * t1266 * t19534 * t652 - F::new(4.0) * t19289 * t652 * t671 - F::new(2.0) * t2363 * t6287 * t652 - F::new(2.0) * t3652 * t5493 * t652 - F::new(4.0) * t510 * t55410 * t89 - F::new(2.0) * t1266 * t19450 - F::new(4.0) * t1266 * t19461 - F::new(2.0) * t12724 * t1774 - F::new(4.0) * t12728 * t1774 - F::new(4.0) * t12835 * t4028 - F::new(4.0) * t12835 * t7458 - F::new(2.0) * t19451 * t2364 - F::new(4.0) * t20100 * t2314 - F::new(4.0) * t20100 * t4034 - F::new(4.0) * t20127 * t2314 - F::new(4.0) * t4026 * t5107 - F::new(4.0) * t55943 * t672;
    t56075
}
