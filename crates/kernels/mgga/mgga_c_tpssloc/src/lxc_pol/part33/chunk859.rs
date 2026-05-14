//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 859/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk859<F: Float>(t25: F, t28: F, t11987: F, t1298: F, t20216: F, t20376: F, t5170: F, t5397: F, t12000: F, t1302: F, t20385: F, t20390: F, t5178: F, t5966: F, zeta_threshold: F) -> (F,) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t20406 = piecewise3(t26, 0.0, 8.0 / 27.0 * t11987 * t20376 - 2.0 / 3.0 * t5170 * t5397 + 2.0 / 3.0 * t1298 * t20216);
    let t20414 = piecewise3(t29, 0.0, 8.0 / 27.0 * t12000 * t20385 - 2.0 / 3.0 * t5178 * t5966 + 2.0 / 3.0 * t1302 * t20390);
    let t20416 = t20406 / 2.0 + t20414 / 2.0;
    (t20416,)
}
