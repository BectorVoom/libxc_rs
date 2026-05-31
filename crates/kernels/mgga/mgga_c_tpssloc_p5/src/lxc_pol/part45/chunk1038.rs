//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1038/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1038<F: Float>(t114552: F, t2039: F, t1873: F, t84097: F, t115241: F, t115783: F, t115785: F, t115788: F, t115790: F, t115792: F, t22461: F, t2363: F, t23917: F, t31532: F, t6517: F, t671: F, t7056: F, t8446: F, t90041: F, t90044: F) -> F {
    let t115796 = F::cast_from(2.0_f64) * t114552 * t2039;
    let t115802 = F::cast_from(2.0_f64) * t84097 * t1873;
    let t115809 = F::cast_from(4.0_f64) * t115241 * t671 + F::cast_from(4.0_f64) * t2039 * t90041 + F::cast_from(2.0_f64) * t2039 * t90044 + F::cast_from(4.0_f64) * t22461 * t7056 + F::cast_from(2.0_f64) * t2363 * t31532 + F::cast_from(2.0_f64) * t23917 * t6517 + t115783 + t115785 + t115788 + t115790 + t115792 + t115796 + t115802 + t8446;
    t115809
}
