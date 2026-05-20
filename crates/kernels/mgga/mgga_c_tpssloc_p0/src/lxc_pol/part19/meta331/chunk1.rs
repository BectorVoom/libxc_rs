//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1182/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1182<F: Float>(t1315: F, t210: F, t214: F, t39892: F, t40025: F, t40026: F, t40401: F, t40402: F, t40404: F, t40407: F, t40410: F, t40415: F, t40422: F, t40423: F, t40425: F, t40429: F, t40431: F) -> F {
    let t40437 = -F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t210 * t214 * t39892 - t40401 + F::cast_from(0.22469135802469135801e0_f64) * t40402 - F::cast_from(0.77777777777777777775e-1_f64) * t40404 + F::cast_from(0.13148148148148148148e0_f64) * t40407 + F::cast_from(0.94999999999999999997e-1_f64) * t40410 + F::cast_from(0.39999999999999999998e-1_f64) * t40415 + t40422 + F::cast_from(0.15555555555555555555e-1_f64) * t40423 - F::cast_from(0.31666666666666666666e-1_f64) * t40425 + F::cast_from(0.33333333333333333332e-2_f64) * t40429 + F::cast_from(0.18666666666666666665e0_f64) * t40431 + F::cast_from(0.99999999999999999995e-1_f64) * t40025 * t210 * t214 * t40026;
    t40437
}
