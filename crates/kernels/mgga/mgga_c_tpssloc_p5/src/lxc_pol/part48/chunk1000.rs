//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1000/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1000<F: Float>(t114264: F, t114270: F, t114279: F, t114288: F, t114292: F, t115596: F, t115601: F, t115617: F, t115619: F, t12033: F, t1375: F, t1386: F, t22670: F, t24088: F, t24092: F, t24095: F, t31564: F, t3752: F, t3882: F, t3887: F, t3911: F, t568: F, t6958: F, t6963: F, t7199: F, t8617: F, t8636: F, t8637: F) -> F {
    let t115622 = t3752 * t8617 * t568 + F::new(4.0) * t3882 * t31564 - F::cast_from(0.76763589786250567036e-1_f64) * t115596 + t114264 + t114270 - t114279 + F::new(4.0) * t24095 * t6963 + t114288 + F::cast_from(0.82246703342411321824e-2_f64) * t115601 + F::new(2.0) * t6958 * t24088 + F::new(2.0) * t1375 * t3887 * t8636 * t3911 + F::new(4.0) * t22670 * t7199 - t12033 * t8637 - F::new(6.0) * t6958 * t24092 + t114292 + F::cast_from(0.3289868133696452873e-1_f64) * t115617 - F::new(2.0) * t115619 * t1386;
    t115622
}
