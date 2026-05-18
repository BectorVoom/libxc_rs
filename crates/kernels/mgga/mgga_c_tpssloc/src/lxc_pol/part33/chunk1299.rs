//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1299/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1299<F: Float>(t7583: F, t88383: F, t23384: F, t28684: F, t1920: F, t28474: F, t968: F, t5914: F, t6703: F, t28492: F, t28500: F, t28648: F, t82431: F) -> (F, F, F, F, F, F, F) {
    let t99834 = t88383 * t7583;
    let t99864 = t23384 * t28684;
    let t99877 = t1920 * t968 * t28474;
    let t99895 = t6703 * t5914;
    let t99948 = t23384 * t28492;
    let t99956 = t23384 * t28500;
    let t99960 = t82431 * t28648;
    (t99834, t99864, t99877, t99895, t99948, t99956, t99960)
}
