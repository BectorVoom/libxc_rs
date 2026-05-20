//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2361/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2361<F: Float>(t20100: F, t20136: F, t20143: F, t2314: F, t24932: F, t27888: F, t29855: F, t4034: F, t5450: F, t5494: F, t6287: F, t6468: F, t7264: F, t7266: F, t7408: F, t7412: F, t97899: F, t97905: F, t97910: F, t97914: F, t97916: F, t97919: F, t97923: F, t97925: F, t97928: F) -> F {
    let t105092 = -F::new(2.0) * t20100 * t7266 - F::new(4.0) * t20136 * t7266 - F::new(2.0) * t20143 * t7266 - F::new(2.0) * t2314 * t29855 - F::new(2.0) * t24932 * t5494 - F::new(2.0) * t27888 * t5494 - F::new(2.0) * t29855 * t4034 - t5450 * t7408 - t6287 * t7264 + t6468 * t7412 + t97899 - t97905 - t97910 + t97914 - t97916 - t97919 + t97923 + t97925 - t97928;
    t105092
}
