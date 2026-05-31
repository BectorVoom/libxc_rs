//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1052/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1052<F: Float>(t127109: F, t127111: F, t128387: F, t128393: F, t128397: F, t128401: F, t128404: F, t128406: F, t128413: F, t128415: F, t19451: F, t2075: F, t2114: F, t2165: F, t27863: F, t28959: F, t29197: F, t29214: F, t29219: F, t29486: F, t33690: F, t7266: F, t7802: F, t8835: F) -> F {
    let t130342 = -F::cast_from(2.0_f64) * t19451 * t8835 - t2075 * t29486 - t2114 * t29197 - F::cast_from(2.0_f64) * t2165 * t28959 - F::cast_from(4.0_f64) * t27863 * t7802 - F::cast_from(2.0_f64) * t29214 * t7266 - F::cast_from(4.0_f64) * t29219 * t7266 - F::cast_from(4.0_f64) * t33690 * t7802 - t127109 - t127111 - t128387 + t128393 + t128397 - t128401 - t128404 - t128406 - t128413 - t128415;
    t130342
}
