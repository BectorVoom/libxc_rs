//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2348/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2348<F: Float>(t225: F, t26221: F, t1307: F, t1377: F, t22633: F, t22635: F, t5353: F, t26215: F, t80650: F, t12033: F, t1386: F, t16439: F, t22630: F, t22670: F, t22913: F, t26371: F, t3882: F, t5215: F, t5321: F, t5354: F, t6963: F, t7750: F, t81318: F, t81328: F) -> F {
    let t91441 = t26221 * t225;
    let t91449 = t22633 * t22635 * t1377 * t5353 * t1307;
    let t91455 = t22633 * t80650 * t26215;
    let t91459 = -F::new(6.0) * t5215 * t22630 - t81318 - t12033 * t7750 + F::new(4.0) * t16439 * t6963 - F::new(2.0) * t91441 * t1386 - F::new(2.0) * t22670 * t5354 + F::cast_from(0.3289868133696452873e-1_f64) * t91449 - F::cast_from(0.16449340668482264365e-1_f64) * t81328 + F::new(2.0) * t5321 * t22913 + F::cast_from(0.3289868133696452873e-1_f64) * t91455 + F::new(4.0) * t3882 * t26371;
    t91459
}
