//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 986/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk986<F: Float>(t31618: F, t3719: F, t6637: F, t6888: F, t22892: F, t22893: F, t31619: F, t22685: F, t3734: F, t31628: F, t6914: F, t114056: F, t115387: F, t115391: F, t115395: F, t115397: F, t115402: F, t3773: F, t8634: F) -> F {
    let t115406 = t6888 * t6637 * t31618 * t3719;
    let t115409 = t22892 * t22893 * t31619;
    let t115413 = t22685 * t6637 * t31618 * t3734;
    let t115415 = t6914 * t31628;
    let t115417 = -F::cast_from(0.82246703342411321825e-2_f64) * t115387 - t115391 + t3773 * t8634 + t114056 + F::cast_from(0.16449340668482264365e-1_f64) * t115395 + F::cast_from(0.76763589786250567036e-1_f64) * t115397 - F::cast_from(0.3289868133696452873e-1_f64) * t115402 - F::cast_from(0.16449340668482264365e-1_f64) * t115406 + F::cast_from(0.16449340668482264365e-1_f64) * t115409 + F::cast_from(0.49348022005446793095e-1_f64) * t115413 + F::cast_from(0.38381794893125283518e-1_f64) * t115415;
    t115417
}
