//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 789/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk789<F: Float>(t22: F, t235: F, t36632: F, t504: F, t7191: F, t36012: F, t903: F, t1179: F, t1966: F, t1968: F, t483: F, t7367: F) -> (F, F, F, F, F) {
    let t36634 = t235 * t36632 * t22;
    let t36639 = t504 * t7191;
    let t36646 = t903 * t36012;
    let t36662 = t1966 * t1179 * t483 * t1968;
    let t36663 = t36662 * t7367;
    (t36634, t36639, t36646, t36662, t36663)
}
