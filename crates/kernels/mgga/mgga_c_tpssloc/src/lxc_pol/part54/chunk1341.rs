//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1341/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1341<F: Float>(t1992: F, t550: F, t6976: F, t90946: F, t22704: F, t22705: F, t32744: F, t120437: F, t1352: F, t22633: F, t26403: F, t3807: F) -> (F, F, F, F) {
    let t120456 = F::new(0.16449340668482264365e-1) * t1992 * t6976 * t90946 * t550;
    let t120458 = t22704 * t22705 * t32744;
    let t120459 = F::new(0.82246703342411321825e-2) * t120458;
    let t120463 = F::new(0.3289868133696452873e-1) * t22633 * t6976 * t120437 * t1352;
    let t120467 = F::new(0.3289868133696452873e-1) * t22633 * t6976 * t26403 * t3807;
    (t120456, t120459, t120463, t120467)
}
