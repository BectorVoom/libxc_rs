//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1011/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1011<F: Float>(t33358: F, t91655: F, t127107: F, t127109: F, t127111: F, t128298: F, t128300: F, t128302: F, t128303: F, t128306: F, t128371: F, t128375: F, t128377: F, t128381: F, t128383: F, t128385: F, t1849: F, t31532: F, t33601: F, t510: F, t5460: F, t6287: F, t8519: F) -> F {
    let t128387 = F::cast_from(6.0_f64) * t91655 * t33358;
    let t128388 = -t128371 * t510 + F::cast_from(2.0_f64) * t1849 * t33601 - F::cast_from(4.0_f64) * t31532 * t5460 - t6287 * t8519 - t127107 - t127109 - t127111 - t128298 - t128300 - t128302 + t128303 - t128306 - t128375 - t128377 - t128381 - t128383 - t128385 - t128387;
    t128388
}
