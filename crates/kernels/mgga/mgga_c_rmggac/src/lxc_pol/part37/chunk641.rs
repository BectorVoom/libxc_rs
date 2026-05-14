//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 641/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk641<F: Float>(t2039: F, t2153: F, t270: F, t638: F, t2046: F, t2050: F, t31: F, t2128: F, t14136: F, t7292: F, t13966: F, t7301: F, t1343: F, t3076: F, t7553: F, t7765: F) -> (F, F, F, F, F, F, F) {
    let t70365 = t638 * t2039 * t2153 * t270;
    let t70369 = t2046 * t2050 * t2153 * t31;
    let t70373 = t638 * t2039 * t2128 * t270;
    let t70376 = t638 * t7292 * t14136;
    let t70381 = t2046 * t13966 * t7301;
    let t70383 = t3076 * t1343;
    let t70385 = t7553 * t70383 * t7765;
    (t70365, t70369, t70373, t70376, t70381, t70383, t70385)
}
