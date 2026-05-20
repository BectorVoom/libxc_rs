//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1900/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1900<F: Float>(t22633: F, t22635: F, t3888: F, t90488: F, t1887: F, t80827: F, t26334: F, t26339: F, t81159: F, t22716: F, t7697: F, t1307: F, t1385: F) -> (F, F, F, F, F, F) {
    let t90491 = t22633 * t22635 * t90488 * t3888;
    let t90497 = t80827 * t1887;
    let t90498 = t90497 * t26334;
    let t90500 = t81159 * t26339;
    let t90503 = t22716 * t7697;
    let t90506 = t1307 * t1385;
    (t90491, t90497, t90498, t90500, t90503, t90506)
}
