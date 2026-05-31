//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1194/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1194<F: Float>(t2165: F, t7056: F, t2040: F, t2314: F, t24932: F, t31294: F, t31296: F, t31298: F, t31302: F, t32318: F, t32359: F, t4034: F, t574: F, t652: F, t7042: F, t7057: F, t7061: F, t7266: F, t7271: F, t8835: F) -> (F, F) {
    let t32365 = t2165 * t7056;
    let t32368 = -F::cast_from(2.0_f64) * t2040 * t24932 - F::cast_from(2.0_f64) * t2314 * t8835 - F::cast_from(2.0_f64) * t32318 * t652 + t32359 * t574 - F::cast_from(2.0_f64) * t32365 * t652 - F::cast_from(2.0_f64) * t4034 * t8835 - F::cast_from(2.0_f64) * t7042 * t7271 - F::cast_from(2.0_f64) * t7057 * t7266 - F::cast_from(2.0_f64) * t7061 * t7266 + t31294 - t31296 - t31298 - t31302;
    (t32365, t32368)
}
