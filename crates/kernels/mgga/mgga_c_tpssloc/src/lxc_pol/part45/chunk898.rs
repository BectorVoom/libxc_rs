//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 898/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk898<F: Float>(t1992: F, t550: F, t6976: F, t84441: F, t22704: F, t22705: F, t31627: F, t1351: F, t7191: F, t31632: F, t6883: F, t22724: F, t31623: F, t22716: F, t8631: F, t114058: F, t114061: F, t114064: F, t114073: F, t114077: F) -> (F,) {
    let t115420 = t1992 * t6976 * t84441 * t550;
    let t115423 = t22704 * t22705 * t31627;
    let t115428 = t1992 * t6976 * t7191 * t1351 * t550;
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    let t115433 = 0.26044789391763585244e-1 * t115432;
    let t115434 = t22716 * t8631;
    let t115435 = 0.63969658155208805863e-1 * t115434;
    let t115436 = t114058 + t114061 - t114064 - 0.82246703342411321825e-2 * t115420 + 0.82246703342411321824e-2 * t115423 - 0.16449340668482264365e-1 * t115428 - 0.38381794893125283518e-1 * t115430 + t115433 + t115435 - t114073 - t114077;
    (t115436,)
}
