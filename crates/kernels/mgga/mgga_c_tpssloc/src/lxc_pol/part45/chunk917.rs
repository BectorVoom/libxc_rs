//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 917/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk917<F: Float>(t115723: F, t2039: F, t31537: F, t7056: F, t22479: F, t88: F, t31717: F, t23917: F, t8601: F, t114552: F, t1873: F, t84097: F, t115241: F, t22461: F, t2363: F, t31532: F, t6517: F, t671: F, t8446: F, t90041: F, t90044: F) -> (F,) {
    let t115783 = 4.0 * t115723 * t2039;
    let t115785 = 4.0 * t31537 * t7056;
    let t115786 = t88 * t22479;
    let t115788 = 2.0 * t115786 * t2039;
    let t115790 = 4.0 * t31717 * t7056;
    let t115792 = 2.0 * t8601 * t23917;
    let t115796 = 2.0 * t114552 * t2039;
    let t115802 = 2.0 * t84097 * t1873;
    let t115809 = 4.0 * t115241 * t671 + 4.0 * t2039 * t90041 + 2.0 * t2039 * t90044 + 4.0 * t22461 * t7056 + 2.0 * t2363 * t31532 + 2.0 * t23917 * t6517 + t115783 + t115785 + t115788 + t115790 + t115792 + t115796 + t115802 + t8446;
    (t115809,)
}
