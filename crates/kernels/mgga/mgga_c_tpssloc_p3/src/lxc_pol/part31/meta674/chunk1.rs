//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2037/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2037<F: Float>(t12725: F, t1774: F, t19451: F, t19456: F, t20100: F, t20136: F, t20143: F, t22574: F, t23938: F, t26977: F, t27147: F, t27150: F, t27163: F, t27170: F, t27226: F, t28002: F, t28821: F, t28830: F, t29247: F, t32193: F, t4028: F, t5494: F, t6287: F, t652: F, t7042: F, t7056: F, t7057: F, t7061: F, t7220: F, t7458: F, t7796: F, t7802: F, t83886: F) -> F {
    let t103070 = -F::new(4.0) * t7458 * t27150 - F::new(6.0) * t22574 * t32193 * t28830 - F::new(6.0) * t83886 * t29247 - F::new(4.0) * t28002 * t7057 - F::new(4.0) * t12725 * t7796 - F::new(4.0) * t19456 * t7796 - F::new(4.0) * t4028 * t27163 - F::new(4.0) * t7042 * t20136 - F::new(2.0) * t19451 * t7061 - F::new(2.0) * t652 * t6287 * t7056 - F::new(2.0) * t7042 * t20100 - t28821 * t7220 - F::new(4.0) * t652 * t1774 * t27170 - F::new(2.0) * t23938 * t5494 - F::new(2.0) * t26977 * t5494 - F::new(2.0) * t7042 * t20143 - F::new(4.0) * t4028 * t27147 - F::new(4.0) * t19456 * t7802 - F::new(4.0) * t4028 * t27226;
    t103070
}
